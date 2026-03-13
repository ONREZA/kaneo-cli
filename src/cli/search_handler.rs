use crate::api::ApiClient;
use crate::auth::{self, ResolvedContext};
use crate::cli::SearchArgs;
use crate::output;

pub async fn run(args: SearchArgs, ctx: &ResolvedContext, json: bool) -> anyhow::Result<()> {
    let ws = auth::require_workspace(ctx)?;
    let client = ApiClient::new(&ctx.api_url, &ctx.api_key)?;

    let mut query_params = vec![
        ("q".to_string(), args.query.clone()),
        ("workspaceId".to_string(), ws.to_string()),
        ("type".to_string(), args.r#type.clone()),
        ("limit".to_string(), args.limit.clone()),
    ];

    if let Some(pid) = &args.project_id {
        query_params.push(("projectId".to_string(), pid.clone()));
    }

    let results: serde_json::Value = client
        .get_query("/search", &query_params)
        .await?;

    if json {
        output::json_output(&results);
        return Ok(());
    }

    let bold = console::Style::new().bold();
    let dim = console::Style::new().dim();
    let cyan = console::Style::new().cyan();
    let mut found = false;

    // Tasks
    if let Some(tasks) = results.get("tasks").and_then(|v| v.as_array()) {
        if !tasks.is_empty() {
            found = true;
            output::header(false, "Tasks");
            for t in tasks {
                let title = t.get("title").and_then(|v| v.as_str()).unwrap_or("?");
                let id = t.get("id").and_then(|v| v.as_str()).unwrap_or("");
                let number = t.get("number").and_then(|v| v.as_i64()).unwrap_or(0);
                let status = t.get("status").and_then(|v| v.as_str()).unwrap_or("");
                eprintln!(
                    "  {} {} {} {}",
                    cyan.apply_to(format!("#{number}")),
                    bold.apply_to(title),
                    dim.apply_to(status),
                    dim.apply_to(id),
                );
            }
        }
    }

    // Projects
    if let Some(projects) = results.get("projects").and_then(|v| v.as_array()) {
        if !projects.is_empty() {
            found = true;
            output::header(false, "Projects");
            for p in projects {
                let name = p.get("name").and_then(|v| v.as_str()).unwrap_or("?");
                let id = p.get("id").and_then(|v| v.as_str()).unwrap_or("");
                eprintln!("  {} {}", bold.apply_to(name), dim.apply_to(id));
            }
        }
    }

    // Workspaces
    if let Some(workspaces) = results.get("workspaces").and_then(|v| v.as_array()) {
        if !workspaces.is_empty() {
            found = true;
            output::header(false, "Workspaces");
            for w in workspaces {
                let name = w.get("name").and_then(|v| v.as_str()).unwrap_or("?");
                let id = w.get("id").and_then(|v| v.as_str()).unwrap_or("");
                eprintln!("  {} {}", bold.apply_to(name), dim.apply_to(id));
            }
        }
    }

    // Comments
    if let Some(comments) = results.get("comments").and_then(|v| v.as_array()) {
        if !comments.is_empty() {
            found = true;
            output::header(false, "Comments");
            for c in comments {
                let content = c.get("content").and_then(|v| v.as_str()).unwrap_or("?");
                let id = c.get("id").and_then(|v| v.as_str()).unwrap_or("");
                eprintln!("  {} {}", content, dim.apply_to(id));
            }
        }
    }

    if !found {
        output::warn(false, &format!("No results for '{}'", args.query));
    }

    eprintln!();
    Ok(())
}
