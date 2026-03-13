use crate::api::ApiClient;
use crate::api::types::Project;
use crate::auth::{self, LocalConfig, ResolvedContext};
use crate::cli::LinkArgs;
use crate::output;

pub async fn run(args: LinkArgs, ctx: &ResolvedContext, json: bool) -> anyhow::Result<()> {
    let client = ApiClient::new(&ctx.api_url, &ctx.api_key)?;
    let interactive = output::is_interactive() && !json;

    // Resolve workspace
    let workspace_id = match args.workspace {
        Some(id) => Some(id),
        None if interactive => Some(select_workspace(&client).await?),
        None => ctx.workspace_id.clone(),
    };

    // Resolve project
    let project_id = match args.project {
        Some(id) => Some(id),
        None if interactive => {
            let ws = workspace_id
                .as_deref()
                .ok_or_else(|| anyhow::anyhow!("workspace ID required to list projects"))?;
            Some(select_project(&client, ws).await?)
        }
        None => ctx.project_id.clone(),
    };

    let config = LocalConfig {
        workspace: workspace_id,
        project: project_id,
    };
    let cwd = std::env::current_dir()?;
    auth::write_local_config(&cwd, &config)?;

    if json {
        output::json_output(&serde_json::json!({
            "linked": true,
            "path": cwd.join(".kaneo.json").display().to_string(),
            "workspace": config.workspace,
            "project": config.project,
        }));
    } else {
        output::success(
            false,
            &format!("Linked {}", cwd.join(".kaneo.json").display()),
        );
    }

    Ok(())
}

async fn select_workspace(client: &ApiClient) -> anyhow::Result<String> {
    let orgs: Vec<serde_json::Value> = client.get("/auth/organization/list-organizations").await?;

    if orgs.is_empty() {
        anyhow::bail!("no workspaces found");
    }

    let labels: Vec<String> = orgs
        .iter()
        .map(|o| {
            let name = o.get("name").and_then(|v| v.as_str()).unwrap_or("?");
            let slug = o.get("slug").and_then(|v| v.as_str()).unwrap_or("");
            format!("{name} ({slug})")
        })
        .collect();

    let idx = output::select("Workspace", &labels)?;

    orgs[idx]
        .get("id")
        .and_then(|v| v.as_str())
        .map(String::from)
        .ok_or_else(|| anyhow::anyhow!("workspace missing id"))
}

async fn select_project(client: &ApiClient, workspace_id: &str) -> anyhow::Result<String> {
    let projects: Vec<Project> = client
        .get_query("/project", &[("workspaceId", workspace_id)])
        .await?;

    if projects.is_empty() {
        anyhow::bail!("no projects found in this workspace");
    }

    let labels: Vec<String> = projects
        .iter()
        .map(|p| {
            let icon = p.icon.as_deref().unwrap_or("📋");
            format!("{icon} {}", p.name)
        })
        .collect();

    let idx = output::select("Project", &labels)?;

    Ok(projects[idx].id.clone())
}
