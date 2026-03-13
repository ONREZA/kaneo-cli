use crate::api::ApiClient;
use crate::api::types::{CreateProjectBody, Project, UpdateProjectBody};
use crate::auth::{self, ResolvedContext};
use crate::cli::{ProjectArgs, ProjectCommand};
use crate::output;

pub async fn run(args: ProjectArgs, ctx: &ResolvedContext, json: bool) -> anyhow::Result<()> {
    let client = ApiClient::new(&ctx.api_url, &ctx.api_key)?;

    match args.command {
        ProjectCommand::List => {
            let ws = auth::require_workspace(ctx)?;
            let projects: Vec<Project> =
                client.get_query("/project", &[("workspaceId", ws)]).await?;

            if json {
                output::json_output(&projects);
            } else {
                if projects.is_empty() {
                    output::warn(false, "No projects found");
                    return Ok(());
                }
                let bold = console::Style::new().bold();
                let dim = console::Style::new().dim();
                for p in &projects {
                    let icon = p.icon.as_deref().unwrap_or("📋");
                    eprintln!(
                        "  {icon} {} {}",
                        bold.apply_to(&p.name),
                        dim.apply_to(&p.id),
                    );
                    if let Some(desc) = &p.description
                        && !desc.is_empty()
                    {
                        eprintln!("    {}", dim.apply_to(desc));
                    }
                }
            }
        }

        ProjectCommand::Get { id } => {
            let project: Project = client.get(&format!("/project/{id}")).await?;

            if json {
                output::json_output(&project);
            } else {
                let bold = console::Style::new().bold();
                let dim = console::Style::new().dim();
                let icon = project.icon.as_deref().unwrap_or("📋");
                eprintln!("  {icon} {}", bold.apply_to(&project.name));
                eprintln!("  {} {}", dim.apply_to("id:"), project.id);
                eprintln!("  {} {}", dim.apply_to("slug:"), project.slug);
                if let Some(desc) = &project.description
                    && !desc.is_empty()
                {
                    eprintln!("  {} {desc}", dim.apply_to("desc:"));
                }
                let public = project.is_public.unwrap_or(false);
                eprintln!("  {} {public}", dim.apply_to("public:"));
            }
        }

        ProjectCommand::Create { name, slug, icon } => {
            let ws = auth::require_workspace(ctx)?;
            let slug = slug.unwrap_or_else(|| slug_from_name(&name));
            let body = CreateProjectBody {
                name: name.clone(),
                workspace_id: ws.to_string(),
                icon,
                slug,
            };
            let project: Project = client.post("/project", &body).await?;

            if json {
                output::json_output(&project);
            } else {
                output::success(
                    false,
                    &format!("Created project '{}' ({})", project.name, project.id),
                );
            }
        }

        ProjectCommand::Update {
            id,
            name,
            slug,
            icon,
            description,
            public,
        } => {
            // Fetch current to merge partial updates
            let current: Project = client.get(&format!("/project/{id}")).await?;
            let body = UpdateProjectBody {
                name: name.unwrap_or(current.name),
                icon: icon.unwrap_or_else(|| current.icon.unwrap_or_default()),
                slug: slug.unwrap_or(current.slug),
                description: description.unwrap_or_else(|| current.description.unwrap_or_default()),
                is_public: public.unwrap_or(current.is_public.unwrap_or(false)),
            };
            let project: Project = client.put(&format!("/project/{id}"), &body).await?;

            if json {
                output::json_output(&project);
            } else {
                output::success(false, &format!("Updated project '{}'", project.name));
            }
        }

        ProjectCommand::Delete { id } => {
            let result: serde_json::Value = client.delete(&format!("/project/{id}")).await?;

            if json {
                output::json_output(&result);
            } else {
                let name = result.get("name").and_then(|v| v.as_str()).unwrap_or(&id);
                output::success(false, &format!("Deleted project '{name}'"));
            }
        }
    }

    Ok(())
}

fn slug_from_name(name: &str) -> String {
    name.to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .trim_matches('-')
        .to_string()
}
