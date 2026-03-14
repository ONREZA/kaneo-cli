use crate::api::ApiClient;
use crate::api::types::{CreateLabelBody, Label};
use crate::auth::{self, ResolvedContext};
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

        LabelCommand::Task { task_id } => {
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
            let body = CreateLabelBody {
                name: name.clone(),
                color,
                workspace_id: ws.to_string(),
                task_id,
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
            #[derive(Serialize)]
            struct Body {
                name: String,
                color: String,
            }
            let label: Label = client
                .put(&format!("/label/{id}"), &Body { name, color })
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
