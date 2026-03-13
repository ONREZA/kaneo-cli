use crate::api::ApiClient;
use crate::api::types::{Column, CreateColumnBody};
use crate::auth::{self, ResolvedContext};
use crate::cli::{ColumnArgs, ColumnCommand};
use crate::output;
use serde::Serialize;

fn resolve_project(arg: Option<String>, ctx: &ResolvedContext) -> anyhow::Result<String> {
    arg.or_else(|| ctx.project_id.clone())
        .ok_or_else(|| anyhow::anyhow!("{}", auth::require_project(ctx).unwrap_err()))
}

pub async fn run(args: ColumnArgs, ctx: &ResolvedContext, json: bool) -> anyhow::Result<()> {
    let client = ApiClient::new(&ctx.api_url, &ctx.api_key)?;

    match args.command {
        ColumnCommand::List { project_id } => {
            let project_id = resolve_project(project_id, ctx)?;
            let columns: Vec<Column> = client.get(&format!("/column/{project_id}")).await?;

            if json {
                output::json_output(&columns);
            } else {
                if columns.is_empty() {
                    output::warn(false, "No columns found");
                    return Ok(());
                }
                let bold = console::Style::new().bold();
                let dim = console::Style::new().dim();
                for c in &columns {
                    let icon = c.icon.as_deref().unwrap_or("▪");
                    let fin = if c.is_final.unwrap_or(false) {
                        " [final]"
                    } else {
                        ""
                    };
                    eprintln!(
                        "  {icon} {} {}{fin}",
                        bold.apply_to(&c.name),
                        dim.apply_to(&c.id),
                    );
                }
            }
        }

        ColumnCommand::Create {
            name,
            icon,
            color,
            is_final,
        } => {
            let project_id = resolve_project(None, ctx)?;
            let body = CreateColumnBody {
                name: name.clone(),
                icon,
                color,
                is_final,
            };
            let col: Column = client.post(&format!("/column/{project_id}"), &body).await?;

            if json {
                output::json_output(&col);
            } else {
                output::success(false, &format!("Created column '{}'", col.name));
            }
        }

        ColumnCommand::Update {
            id,
            name,
            icon,
            color,
            is_final,
        } => {
            #[derive(Serialize)]
            #[serde(rename_all = "camelCase")]
            struct Body {
                #[serde(skip_serializing_if = "Option::is_none")]
                name: Option<String>,
                #[serde(skip_serializing_if = "Option::is_none")]
                icon: Option<String>,
                #[serde(skip_serializing_if = "Option::is_none")]
                color: Option<String>,
                #[serde(skip_serializing_if = "Option::is_none")]
                is_final: Option<bool>,
            }
            let body = Body {
                name,
                icon,
                color,
                is_final,
            };
            let col: Column = client.put(&format!("/column/{id}"), &body).await?;

            if json {
                output::json_output(&col);
            } else {
                output::success(false, &format!("Updated column '{}'", col.name));
            }
        }

        ColumnCommand::Reorder { order } => {
            let project_id = resolve_project(None, ctx)?;
            #[derive(Serialize)]
            struct ReorderBody {
                columns: Vec<ColumnPosition>,
            }
            #[derive(Serialize)]
            struct ColumnPosition {
                id: String,
                position: i64,
            }

            let columns: Vec<ColumnPosition> = order
                .split(',')
                .map(|s| s.trim().to_string())
                .enumerate()
                .map(|(i, id)| ColumnPosition {
                    id,
                    position: i as i64,
                })
                .collect();

            let result: serde_json::Value = client
                .put(
                    &format!("/column/reorder/{project_id}"),
                    &ReorderBody { columns },
                )
                .await?;

            if json {
                output::json_output(&result);
            } else {
                output::success(false, "Columns reordered");
            }
        }

        ColumnCommand::Delete { id } => {
            let col: Column = client.delete(&format!("/column/{id}")).await?;

            if json {
                output::json_output(&col);
            } else {
                output::success(false, &format!("Deleted column '{}'", col.name));
            }
        }
    }

    Ok(())
}
