use crate::api::ApiClient;
use crate::api::types::TaskRelation;
use crate::auth::ResolvedContext;
use crate::cli::{TaskRelationArgs, TaskRelationCommand};
use crate::output;
use serde::Serialize;

pub async fn run(args: TaskRelationArgs, ctx: &ResolvedContext, json: bool) -> anyhow::Result<()> {
    let client = ApiClient::new(&ctx.api_url, &ctx.api_key)?;

    match args.command {
        TaskRelationCommand::List { task_id } => {
            let relations: Vec<TaskRelation> =
                client.get(&format!("/task-relation/{task_id}")).await?;

            if json {
                output::json_output(&relations);
            } else {
                if relations.is_empty() {
                    output::warn(false, "No relations on this task");
                    return Ok(());
                }
                let bold = console::Style::new().bold();
                let dim = console::Style::new().dim();
                for r in &relations {
                    let arrow = match r.relation_type.as_str() {
                        "subtask" => "⊂",
                        "blocks" => "⊳",
                        "related" => "↔",
                        _ => "→",
                    };
                    eprintln!(
                        "  {arrow} {} {} → {} {}",
                        bold.apply_to(&r.relation_type),
                        dim.apply_to(&r.source_task_id),
                        dim.apply_to(&r.target_task_id),
                        dim.apply_to(&r.id),
                    );
                }
            }
        }

        TaskRelationCommand::Create {
            source,
            target,
            r#type,
        } => {
            #[derive(Serialize)]
            #[serde(rename_all = "camelCase")]
            struct Body {
                source_task_id: String,
                target_task_id: String,
                relation_type: String,
            }
            let relation: TaskRelation = client
                .post(
                    "/task-relation",
                    &Body {
                        source_task_id: source,
                        target_task_id: target,
                        relation_type: r#type.as_api_str().to_string(),
                    },
                )
                .await?;

            if json {
                output::json_output(&relation);
            } else {
                output::success(
                    false,
                    &format!(
                        "Created {} relation ({})",
                        relation.relation_type, relation.id
                    ),
                );
            }
        }

        TaskRelationCommand::Delete { id } => {
            let relation: TaskRelation = client.delete(&format!("/task-relation/{id}")).await?;

            if json {
                output::json_output(&relation);
            } else {
                output::success(
                    false,
                    &format!("Deleted {} relation", relation.relation_type),
                );
            }
        }
    }

    Ok(())
}
