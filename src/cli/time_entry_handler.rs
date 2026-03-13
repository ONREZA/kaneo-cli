use crate::api::ApiClient;
use crate::api::types::TimeEntry;
use crate::auth::ResolvedContext;
use crate::cli::{TimeEntryArgs, TimeEntryCommand};
use crate::output;
use serde::Serialize;

pub async fn run(args: TimeEntryArgs, ctx: &ResolvedContext, json: bool) -> anyhow::Result<()> {
    let client = ApiClient::new(&ctx.api_url, &ctx.api_key)?;

    match args.command {
        TimeEntryCommand::List { task_id } => {
            let entries: Vec<TimeEntry> =
                client.get(&format!("/time-entry/task/{task_id}")).await?;

            if json {
                output::json_output(&entries);
            } else {
                if entries.is_empty() {
                    output::warn(false, "No time entries");
                    return Ok(());
                }
                let dim = console::Style::new().dim();
                let bold = console::Style::new().bold();
                for e in &entries {
                    let dur = e
                        .duration
                        .map(format_duration)
                        .unwrap_or_else(|| "running…".into());
                    let desc = e.description.as_deref().unwrap_or("");
                    eprintln!(
                        "  {} {} {} {}",
                        bold.apply_to(&dur),
                        desc,
                        dim.apply_to(&e.start_time),
                        dim.apply_to(&e.id),
                    );
                }
            }
        }

        TimeEntryCommand::Get { id } => {
            let entry: TimeEntry = client.get(&format!("/time-entry/{id}")).await?;

            if json {
                output::json_output(&entry);
            } else {
                let dim = console::Style::new().dim();
                let bold = console::Style::new().bold();
                let dur = entry
                    .duration
                    .map(format_duration)
                    .unwrap_or_else(|| "running…".into());
                eprintln!("  {} {}", dim.apply_to("id:"), entry.id);
                eprintln!("  {} {}", dim.apply_to("task:"), entry.task_id);
                eprintln!("  {} {}", dim.apply_to("duration:"), bold.apply_to(dur));
                eprintln!("  {} {}", dim.apply_to("start:"), entry.start_time);
                if let Some(end) = &entry.end_time {
                    eprintln!("  {} {end}", dim.apply_to("end:"));
                }
                if let Some(desc) = &entry.description
                    && !desc.is_empty()
                {
                    eprintln!("  {} {desc}", dim.apply_to("desc:"));
                }
            }
        }

        TimeEntryCommand::Create {
            task_id,
            start,
            end,
            description,
        } => {
            #[derive(Serialize)]
            #[serde(rename_all = "camelCase")]
            struct Body {
                task_id: String,
                start_time: String,
                #[serde(skip_serializing_if = "Option::is_none")]
                end_time: Option<String>,
                #[serde(skip_serializing_if = "Option::is_none")]
                description: Option<String>,
            }
            let entry: TimeEntry = client
                .post(
                    "/time-entry",
                    &Body {
                        task_id,
                        start_time: start,
                        end_time: end,
                        description,
                    },
                )
                .await?;

            if json {
                output::json_output(&entry);
            } else {
                output::success(false, &format!("Created time entry {}", entry.id));
            }
        }

        TimeEntryCommand::Update {
            id,
            start,
            end,
            description,
        } => {
            #[derive(Serialize)]
            #[serde(rename_all = "camelCase")]
            struct Body {
                start_time: String,
                #[serde(skip_serializing_if = "Option::is_none")]
                end_time: Option<String>,
                #[serde(skip_serializing_if = "Option::is_none")]
                description: Option<String>,
            }
            let entry: TimeEntry = client
                .put(
                    &format!("/time-entry/{id}"),
                    &Body {
                        start_time: start,
                        end_time: end,
                        description,
                    },
                )
                .await?;

            if json {
                output::json_output(&entry);
            } else {
                output::success(false, &format!("Updated time entry {}", entry.id));
            }
        }
    }

    Ok(())
}

fn format_duration(seconds: i64) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    if hours > 0 {
        format!("{hours}h {minutes}m")
    } else {
        format!("{minutes}m")
    }
}
