use crate::api::ApiClient;
use crate::api::types::Activity;
use crate::auth::ResolvedContext;
use crate::cli::resolve::resolve_task_id;
use crate::cli::{ActivityArgs, ActivityCommand};
use crate::output;
use serde::Serialize;

pub async fn run(args: ActivityArgs, ctx: &ResolvedContext, json: bool) -> anyhow::Result<()> {
    let client = ApiClient::new(&ctx.api_url, &ctx.api_key)?;

    match args.command {
        ActivityCommand::List { task_id } => {
            let task_id = resolve_task_id(&task_id, ctx, &client).await?;
            let activities: Vec<Activity> = client.get(&format!("/activity/{task_id}")).await?;

            if json {
                output::json_output(&activities);
            } else {
                if activities.is_empty() {
                    output::warn(false, "No activity");
                    return Ok(());
                }
                let dim = console::Style::new().dim();
                let cyan = console::Style::new().cyan();
                for a in &activities {
                    let content = a.content.as_deref().unwrap_or("");
                    eprintln!(
                        "  {} {} {}",
                        cyan.apply_to(&a.r#type),
                        content,
                        dim.apply_to(&a.created_at),
                    );
                }
            }
        }

        ActivityCommand::Comment { task_id, comment } => {
            let task_id = resolve_task_id(&task_id, ctx, &client).await?;
            #[derive(Serialize)]
            #[serde(rename_all = "camelCase")]
            struct Body {
                task_id: String,
                comment: String,
            }
            let activity: Activity = client
                .post("/activity/comment", &Body { task_id, comment })
                .await?;

            if json {
                output::json_output(&activity);
            } else {
                output::success(false, "Comment added");
            }
        }

        ActivityCommand::EditComment { id, comment } => {
            #[derive(Serialize)]
            #[serde(rename_all = "camelCase")]
            struct Body {
                activity_id: String,
                comment: String,
            }
            let activity: Activity = client
                .put(
                    "/activity/comment",
                    &Body {
                        activity_id: id,
                        comment,
                    },
                )
                .await?;

            if json {
                output::json_output(&activity);
            } else {
                output::success(false, "Comment updated");
            }
        }

        ActivityCommand::DeleteComment { id } => {
            #[derive(Serialize)]
            #[serde(rename_all = "camelCase")]
            struct Body {
                activity_id: String,
            }
            let activity: Activity = client
                .delete_json("/activity/comment", &Body { activity_id: id })
                .await?;

            if json {
                output::json_output(&activity);
            } else {
                output::success(false, "Comment deleted");
            }
        }
    }

    Ok(())
}
