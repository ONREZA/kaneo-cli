use crate::api::ApiClient;
use crate::api::types::Project;
use crate::auth::{self, ResolvedContext};

/// Parse a human-readable task ref like "DEP-65" into (slug, number).
pub fn parse_task_ref(id: &str) -> Option<(&str, i64)> {
    let (slug, num_str) = id.rsplit_once('-')?;
    if slug.is_empty() {
        return None;
    }
    let num = num_str.parse::<i64>().ok()?;
    Some((slug, num))
}

/// Resolve a task identifier. If it looks like "PREFIX-123", resolve it
/// via server-side search first, then fall back to board scan.
pub async fn resolve_task_id(
    id: &str,
    ctx: &ResolvedContext,
    client: &ApiClient,
) -> anyhow::Result<String> {
    let (slug, number) = match parse_task_ref(id) {
        Some(pair) => pair,
        None => return Ok(id.to_string()),
    };

    let ws = auth::require_workspace(ctx)?;

    // Try search API first (single call, works when search is indexed)
    let query = format!("{slug}-{number}");
    let search_query: Vec<(&str, &str)> = vec![
        ("q", query.as_str()),
        ("workspaceId", ws),
        ("type", "tasks"),
        ("limit", "5"),
    ];
    if let Ok(results) = client
        .get_query::<serde_json::Value, _>("/search", &search_query)
        .await
    {
        let tasks = results
            .get("tasks")
            .or_else(|| results.get("results"))
            .and_then(|v| v.as_array());
        if let Some(tasks) = tasks {
            for t in tasks {
                if t.get("number").and_then(|v| v.as_i64()) == Some(number)
                    && let Some(tid) = t.get("id").and_then(|v| v.as_str())
                {
                    return Ok(tid.to_string());
                }
            }
        }
    }

    // Fallback: find project by slug, then scan the board
    let projects: Vec<Project> = client.get_query("/project", &[("workspaceId", ws)]).await?;
    let project = projects
        .iter()
        .find(|p| p.slug.eq_ignore_ascii_case(slug))
        .ok_or_else(|| anyhow::anyhow!("no project with slug '{slug}' (from '{id}')"))?;

    let board: serde_json::Value = client.get(&format!("/task/tasks/{}", project.id)).await?;
    let root = board.get("data").unwrap_or(&board);

    let columns = root
        .as_array()
        .or_else(|| root.get("columns").and_then(|v| v.as_array()));

    if let Some(cols) = columns {
        for col in cols {
            if let Some(tasks) = col.get("tasks").and_then(|v| v.as_array()) {
                for t in tasks {
                    if t.get("number").and_then(|v| v.as_i64()) == Some(number)
                        && let Some(tid) = t.get("id").and_then(|v| v.as_str())
                    {
                        return Ok(tid.to_string());
                    }
                }
            }
        }
    }

    for section in ["plannedTasks", "archivedTasks"] {
        if let Some(tasks) = root.get(section).and_then(|v| v.as_array()) {
            for t in tasks {
                if t.get("number").and_then(|v| v.as_i64()) == Some(number)
                    && let Some(tid) = t.get("id").and_then(|v| v.as_str())
                {
                    return Ok(tid.to_string());
                }
            }
        }
    }

    anyhow::bail!("task {slug}-{number} not found")
}

/// Resolve a workspace from an explicit arg or context.
pub fn resolve_workspace<'a>(
    arg: Option<&'a str>,
    ctx: &'a ResolvedContext,
) -> anyhow::Result<&'a str> {
    match arg.or(ctx.workspace_id.as_deref()) {
        Some(ws) => Ok(ws),
        None => auth::require_workspace(ctx),
    }
}

/// Resolve a project from an explicit arg or context.
pub fn resolve_project(arg: Option<String>, ctx: &ResolvedContext) -> anyhow::Result<String> {
    match arg.or_else(|| ctx.project_id.clone()) {
        Some(id) => Ok(id),
        None => auth::require_project(ctx).map(|s| s.to_string()),
    }
}

/// Inject a `ref` field into a task's JSON value. Fetches the project if slug is unknown.
pub async fn inject_task_ref(
    val: &mut serde_json::Value,
    task_project_id: &str,
    task_number: Option<i64>,
    known_slug: Option<&str>,
    client: &ApiClient,
) {
    let slug = match known_slug {
        Some(s) => Some(s.to_string()),
        None => client
            .get::<Project>(&format!("/project/{task_project_id}"))
            .await
            .ok()
            .map(|p| p.slug),
    };
    if let (Some(slug), Some(number)) = (slug, task_number)
        && let Some(obj) = val.as_object_mut()
    {
        obj.insert(
            "ref".to_string(),
            serde_json::json!(format!("{slug}-{number}")),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_task_ref_valid() {
        assert_eq!(parse_task_ref("DEP-65"), Some(("DEP", 65)));
        assert_eq!(parse_task_ref("my-proj-1"), Some(("my-proj", 1)));
        assert_eq!(parse_task_ref("X-0"), Some(("X", 0)));
    }

    #[test]
    fn parse_task_ref_plain_id() {
        assert_eq!(parse_task_ref("cvwyowgibnsfumrhdi6mxh4v"), None);
        assert_eq!(parse_task_ref("abc123"), None);
    }

    #[test]
    fn parse_task_ref_invalid() {
        assert_eq!(parse_task_ref("-65"), None);
        assert_eq!(parse_task_ref("DEP-"), None);
        assert_eq!(parse_task_ref("DEP-abc"), None);
        assert_eq!(parse_task_ref(""), None);
    }
}
