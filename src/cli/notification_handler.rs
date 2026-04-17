use crate::api::ApiClient;
use crate::api::types::{Notification, NotificationPreferences};
use crate::auth::ResolvedContext;
use crate::cli::resolve::resolve_workspace;
use crate::cli::{NotificationArgs, NotificationCommand, NotificationPrefsCommand};
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
            let notification: Notification = client
                .patch_empty(&format!("/notification/{id}/read"))
                .await?;

            if json {
                output::json_output(&notification);
            } else {
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

        NotificationCommand::Prefs(args) => run_prefs(args.command, ctx, json, &client).await?,
    }

    Ok(())
}

async fn run_prefs(
    cmd: NotificationPrefsCommand,
    ctx: &ResolvedContext,
    json: bool,
    client: &ApiClient,
) -> anyhow::Result<()> {
    match cmd {
        NotificationPrefsCommand::Show => {
            let prefs: NotificationPreferences = client.get("/notification-preferences").await?;
            if json {
                output::json_output(&prefs);
            } else {
                print_prefs(&prefs);
            }
        }

        NotificationPrefsCommand::Set {
            email_enabled,
            ntfy_enabled,
            ntfy_server_url,
            ntfy_topic,
            ntfy_token,
            gotify_enabled,
            gotify_server_url,
            gotify_token,
            webhook_enabled,
            webhook_url,
            webhook_secret,
        } => {
            #[derive(Serialize)]
            #[serde(rename_all = "camelCase")]
            struct Body {
                #[serde(skip_serializing_if = "Option::is_none")]
                email_enabled: Option<bool>,
                #[serde(skip_serializing_if = "Option::is_none")]
                ntfy_enabled: Option<bool>,
                #[serde(skip_serializing_if = "Option::is_none")]
                ntfy_server_url: Option<String>,
                #[serde(skip_serializing_if = "Option::is_none")]
                ntfy_topic: Option<String>,
                #[serde(skip_serializing_if = "Option::is_none")]
                ntfy_token: Option<String>,
                #[serde(skip_serializing_if = "Option::is_none")]
                gotify_enabled: Option<bool>,
                #[serde(skip_serializing_if = "Option::is_none")]
                gotify_server_url: Option<String>,
                #[serde(skip_serializing_if = "Option::is_none")]
                gotify_token: Option<String>,
                #[serde(skip_serializing_if = "Option::is_none")]
                webhook_enabled: Option<bool>,
                #[serde(skip_serializing_if = "Option::is_none")]
                webhook_url: Option<String>,
                #[serde(skip_serializing_if = "Option::is_none")]
                webhook_secret: Option<String>,
            }

            let body = Body {
                email_enabled,
                ntfy_enabled,
                ntfy_server_url,
                ntfy_topic,
                ntfy_token,
                gotify_enabled,
                gotify_server_url,
                gotify_token,
                webhook_enabled,
                webhook_url,
                webhook_secret,
            };

            let prefs: NotificationPreferences =
                client.put("/notification-preferences", &body).await?;

            if json {
                output::json_output(&prefs);
            } else {
                output::success(false, "Updated notification preferences");
                print_prefs(&prefs);
            }
        }

        NotificationPrefsCommand::Workspace {
            id,
            active,
            email_enabled,
            ntfy_enabled,
            gotify_enabled,
            webhook_enabled,
            project_mode,
            projects,
        } => {
            let ws_id = resolve_workspace(id.as_deref(), ctx)?;

            if let Some(mode) = project_mode.as_deref()
                && mode != "all"
                && mode != "selected"
            {
                anyhow::bail!("--project-mode must be 'all' or 'selected'");
            }

            let current: NotificationPreferences = client.get("/notification-preferences").await?;
            let existing = current.workspaces.iter().find(|r| r.workspace_id == ws_id);

            let selected_ids: Vec<String> = match projects.as_deref() {
                Some(s) => s.split(',').map(|p| p.trim().to_string()).collect(),
                None => existing
                    .map(|r| r.selected_project_ids.clone())
                    .unwrap_or_default(),
            };

            #[derive(Serialize)]
            #[serde(rename_all = "camelCase")]
            struct Body {
                is_active: bool,
                email_enabled: bool,
                ntfy_enabled: bool,
                gotify_enabled: bool,
                webhook_enabled: bool,
                project_mode: String,
                selected_project_ids: Vec<String>,
            }

            let body = Body {
                is_active: active.unwrap_or_else(|| existing.map(|r| r.is_active).unwrap_or(true)),
                email_enabled: email_enabled
                    .unwrap_or_else(|| existing.map(|r| r.email_enabled).unwrap_or(true)),
                ntfy_enabled: ntfy_enabled
                    .unwrap_or_else(|| existing.map(|r| r.ntfy_enabled).unwrap_or(true)),
                gotify_enabled: gotify_enabled
                    .unwrap_or_else(|| existing.map(|r| r.gotify_enabled).unwrap_or(true)),
                webhook_enabled: webhook_enabled
                    .unwrap_or_else(|| existing.map(|r| r.webhook_enabled).unwrap_or(true)),
                project_mode: project_mode.unwrap_or_else(|| {
                    existing
                        .map(|r| r.project_mode.clone())
                        .unwrap_or_else(|| "all".into())
                }),
                selected_project_ids: selected_ids,
            };

            let prefs: NotificationPreferences = client
                .put(
                    &format!("/notification-preferences/workspaces/{ws_id}"),
                    &body,
                )
                .await?;

            if json {
                output::json_output(&prefs);
            } else {
                output::success(false, &format!("Updated notification rule for {ws_id}"));
            }
        }

        NotificationPrefsCommand::DeleteWorkspace { id } => {
            let ws_id = resolve_workspace(id.as_deref(), ctx)?;
            let prefs: NotificationPreferences = client
                .delete(&format!("/notification-preferences/workspaces/{ws_id}"))
                .await?;

            if json {
                output::json_output(&prefs);
            } else {
                output::success(false, &format!("Removed notification rule for {ws_id}"));
            }
        }
    }

    Ok(())
}

fn print_prefs(prefs: &NotificationPreferences) {
    let bold = console::Style::new().bold();
    let dim = console::Style::new().dim();
    let green = console::Style::new().green();
    let red = console::Style::new().red();

    let onoff = |v: bool| {
        if v {
            green.apply_to("on").to_string()
        } else {
            red.apply_to("off").to_string()
        }
    };

    eprintln!("  {}", bold.apply_to("Global delivery"));
    eprintln!(
        "  {} email={} (to {})",
        dim.apply_to("•"),
        onoff(prefs.email_enabled),
        prefs.email_address.as_deref().unwrap_or("—"),
    );
    eprintln!(
        "  {} ntfy={} server={} topic={} token={}",
        dim.apply_to("•"),
        onoff(prefs.ntfy_enabled),
        prefs.ntfy_server_url.as_deref().unwrap_or("—"),
        prefs.ntfy_topic.as_deref().unwrap_or("—"),
        prefs.masked_ntfy_token.as_deref().unwrap_or("—"),
    );
    eprintln!(
        "  {} gotify={} server={} token={}",
        dim.apply_to("•"),
        onoff(prefs.gotify_enabled),
        prefs.gotify_server_url.as_deref().unwrap_or("—"),
        prefs.masked_gotify_token.as_deref().unwrap_or("—"),
    );
    eprintln!(
        "  {} webhook={} url={} secret={}",
        dim.apply_to("•"),
        onoff(prefs.webhook_enabled),
        prefs.webhook_url.as_deref().unwrap_or("—"),
        prefs.masked_webhook_secret.as_deref().unwrap_or("—"),
    );

    if prefs.workspaces.is_empty() {
        eprintln!(
            "\n  {} (no per-workspace rules — global preferences apply)",
            dim.apply_to("Workspace rules")
        );
    } else {
        eprintln!("\n  {}", bold.apply_to("Workspace rules"));
        for ws in &prefs.workspaces {
            let projects = if ws.project_mode == "selected" {
                format!("{} project(s)", ws.selected_project_ids.len(),)
            } else {
                "all projects".to_string()
            };
            eprintln!(
                "  {} {} {} [{}] email={} ntfy={} gotify={} webhook={} → {}",
                dim.apply_to("•"),
                bold.apply_to(&ws.workspace_name),
                dim.apply_to(&ws.workspace_id),
                onoff(ws.is_active),
                onoff(ws.email_enabled),
                onoff(ws.ntfy_enabled),
                onoff(ws.gotify_enabled),
                onoff(ws.webhook_enabled),
                projects,
            );
        }
    }
}
