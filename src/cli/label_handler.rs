use crate::api::ApiClient;
use crate::api::types::{CreateLabelBody, Label};
use crate::auth::{self, ResolvedContext};
use crate::cli::resolve::resolve_task_id;
use crate::cli::{LabelArgs, LabelCommand};
use crate::output;
use serde::Serialize;

pub async fn run(args: LabelArgs, ctx: &ResolvedContext, json: bool) -> anyhow::Result<()> {
    let client = ApiClient::new(&ctx.api_url, &ctx.api_key)?;

    match args.command {
        LabelCommand::List => {
            let ws = auth::require_workspace(ctx)?;
            let labels: Vec<Label> = client.get(&format!("/label/workspace/{ws}")).await?;

            if json {
                output::json_output(&labels);
            } else {
                if labels.is_empty() {
                    output::warn(false, "No labels found");
                    return Ok(());
                }
                let bold = console::Style::new().bold();
                let dim = console::Style::new().dim();
                for l in &labels {
                    eprintln!(
                        "  ● {} {} {}",
                        bold.apply_to(&l.name),
                        dim.apply_to(&l.color),
                        dim.apply_to(&l.id),
                    );
                }
            }
        }

        LabelCommand::Get { id } => {
            let label: Label = client.get(&format!("/label/{id}")).await?;

            if json {
                output::json_output(&label);
            } else {
                let bold = console::Style::new().bold();
                let dim = console::Style::new().dim();
                eprintln!("  ● {}", bold.apply_to(&label.name));
                eprintln!("  {} {}", dim.apply_to("id:"), label.id);
                eprintln!("  {} {}", dim.apply_to("color:"), label.color);
                if let Some(ws) = &label.workspace_id {
                    eprintln!("  {} {ws}", dim.apply_to("workspace:"));
                }
                if let Some(tid) = &label.task_id {
                    eprintln!("  {} {tid}", dim.apply_to("task:"));
                }
            }
        }

        LabelCommand::Task { task_id } => {
            let task_id = resolve_task_id(&task_id, ctx, &client).await?;
            let labels: Vec<Label> = client.get(&format!("/label/task/{task_id}")).await?;

            if json {
                output::json_output(&labels);
            } else {
                if labels.is_empty() {
                    output::warn(false, "No labels on this task");
                    return Ok(());
                }
                for l in &labels {
                    let dim = console::Style::new().dim();
                    eprintln!("  ● {} {}", l.name, dim.apply_to(&l.color));
                }
            }
        }

        LabelCommand::Create {
            name,
            color,
            task_id,
        } => {
            let ws = auth::require_workspace(ctx)?;
            let resolved_task_id = match task_id {
                Some(tid) => Some(resolve_task_id(&tid, ctx, &client).await?),
                None => None,
            };
            let body = CreateLabelBody {
                name: name.clone(),
                color,
                workspace_id: ws.to_string(),
                task_id: resolved_task_id,
            };
            let label: Label = client.post("/label", &body).await?;

            if json {
                output::json_output(&label);
            } else {
                output::success(
                    false,
                    &format!("Created label '{}' ({})", label.name, label.id),
                );
            }
        }

        LabelCommand::Attach { id, task } => {
            let task = resolve_task_id(&task, ctx, &client).await?;
            #[derive(Serialize)]
            #[serde(rename_all = "camelCase")]
            struct Body {
                task_id: String,
            }
            let label: Label = client
                .put(&format!("/label/{id}/task"), &Body { task_id: task })
                .await?;

            if json {
                output::json_output(&label);
            } else {
                output::success(false, &format!("Attached label '{}' to task", label.name));
            }
        }

        LabelCommand::Detach { id } => {
            let label: Label = client.delete(&format!("/label/{id}/task")).await?;

            if json {
                output::json_output(&label);
            } else {
                output::success(false, &format!("Detached label '{}' from task", label.name));
            }
        }

        LabelCommand::Update { id, name, color } => {
            let current: Label = client.get(&format!("/label/{id}")).await?;
            #[derive(Serialize)]
            struct Body {
                name: String,
                color: String,
            }
            let label: Label = client
                .put(
                    &format!("/label/{id}"),
                    &Body {
                        name: name.unwrap_or(current.name),
                        color: color.unwrap_or(current.color),
                    },
                )
                .await?;

            if json {
                output::json_output(&label);
            } else {
                output::success(false, &format!("Updated label '{}'", label.name));
            }
        }

        LabelCommand::Delete { id } => {
            let label: Label = client.delete(&format!("/label/{id}")).await?;

            if json {
                output::json_output(&label);
            } else {
                output::success(false, &format!("Deleted label '{}'", label.name));
            }
        }
    }

    Ok(())
}
