use crate::api::ApiClient;
use crate::api::types::Notification;
use crate::auth::ResolvedContext;
use crate::cli::{NotificationArgs, NotificationCommand};
use crate::output;
use serde::Serialize;

pub async fn run(args: NotificationArgs, ctx: &ResolvedContext, json: bool) -> anyhow::Result<()> {
    let client = ApiClient::new(&ctx.api_url, &ctx.api_key)?;

    match args.command {
        NotificationCommand::List => {
            let notifications: Vec<Notification> = client.get("/notification").await?;

            if json {
                output::json_output(&notifications);
            } else {
                if notifications.is_empty() {
                    output::warn(false, "No notifications");
                    return Ok(());
                }
                let bold = console::Style::new().bold();
                let dim = console::Style::new().dim();
                let cyan = console::Style::new().cyan();
                for n in &notifications {
                    let read_icon = if n.is_read.unwrap_or(false) {
                        "  "
                    } else {
                        "● "
                    };
                    eprintln!(
                        "  {read_icon}{} {} {}",
                        cyan.apply_to(&n.r#type),
                        bold.apply_to(&n.title),
                        dim.apply_to(&n.created_at),
                    );
                    if let Some(content) = &n.content
                        && !content.is_empty()
                    {
                        eprintln!("    {}", dim.apply_to(content));
                    }
                }
            }
        }

        NotificationCommand::Read { id } => {
            let _: Notification = client
                .patch_empty(&format!("/notification/{id}/read"))
                .await?;

            if !json {
                output::success(false, "Marked as read");
            }
        }

        NotificationCommand::ReadAll => {
            let result: serde_json::Value = client.patch_empty("/notification/read-all").await?;

            if json {
                output::json_output(&result);
            } else {
                let count = result.get("count").and_then(|v| v.as_i64()).unwrap_or(0);
                output::success(false, &format!("Marked {count} notifications as read"));
            }
        }

        NotificationCommand::ClearAll => {
            let result: serde_json::Value = client.delete("/notification/clear-all").await?;

            if json {
                output::json_output(&result);
            } else {
                let count = result.get("count").and_then(|v| v.as_i64()).unwrap_or(0);
                output::success(false, &format!("Cleared {count} notifications"));
            }
        }

        NotificationCommand::Create {
            user_id,
            title,
            message,
            notification_type,
            related_entity_id,
            related_entity_type,
        } => {
            #[derive(Serialize)]
            #[serde(rename_all = "camelCase")]
            struct Body {
                user_id: String,
                title: String,
                message: String,
                r#type: String,
                #[serde(skip_serializing_if = "Option::is_none")]
                related_entity_id: Option<String>,
                #[serde(skip_serializing_if = "Option::is_none")]
                related_entity_type: Option<String>,
            }
            let notification: Notification = client
                .post(
                    "/notification",
                    &Body {
                        user_id,
                        title,
                        message,
                        r#type: notification_type,
                        related_entity_id,
                        related_entity_type,
                    },
                )
                .await?;

            if json {
                output::json_output(&notification);
            } else {
                output::success(
                    false,
                    &format!("Created notification '{}'", notification.title),
                );
            }
        }
    }

    Ok(())
}
