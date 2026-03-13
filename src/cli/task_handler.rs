use crate::api::ApiClient;
use crate::api::client::upload_to_presigned_url;
use crate::api::types::{Column, CreateTaskBody, Task};
use crate::auth::{self, ResolvedContext};
use crate::cli::{TaskArgs, TaskCommand};
use crate::output;
use serde::Serialize;

const PRIORITIES: &[&str] = &["no-priority", "low", "medium", "high", "urgent"];

fn resolve_project(arg: Option<String>, ctx: &ResolvedContext) -> anyhow::Result<String> {
    arg.or_else(|| ctx.project_id.clone())
        .ok_or_else(|| anyhow::anyhow!("{}", auth::require_project(ctx).unwrap_err()))
}

pub async fn run(args: TaskArgs, ctx: &ResolvedContext, json: bool) -> anyhow::Result<()> {
    let client = ApiClient::new(&ctx.api_url, &ctx.api_key)?;

    match args.command {
        TaskCommand::List {
            project_id,
            status,
            priority,
        } => {
            let project_id = resolve_project(project_id, ctx)?;
            let board: serde_json::Value = client.get(&format!("/task/tasks/{project_id}")).await?;

            let board = filter_board(&board, status.as_deref(), priority.as_deref());

            if json {
                output::json_output(&board);
            } else {
                print_board(&board);
            }
        }

        TaskCommand::Get { id } => {
            let task: Task = client.get(&format!("/task/{id}")).await?;

            if json {
                output::json_output(&task);
            } else {
                print_task(&task);
            }
        }

        TaskCommand::Create {
            title,
            description,
            priority,
            status,
            due_date,
            assignee,
        } => {
            let project_id = resolve_project(None, ctx)?;
            let body = CreateTaskBody {
                title,
                description,
                priority,
                status,
                due_date,
                user_id: assignee,
            };
            let task: Task = client.post(&format!("/task/{project_id}"), &body).await?;

            if json {
                output::json_output(&task);
            } else {
                output::success(
                    false,
                    &format!(
                        "Created task #{} '{}'",
                        task.number.unwrap_or(0),
                        task.title
                    ),
                );
            }
        }

        TaskCommand::Status { id, status } => {
            let status = match status {
                Some(s) => s,
                None if output::is_interactive() => {
                    // Fetch task to get project_id, then columns
                    let current: Task = client.get(&format!("/task/{id}")).await?;
                    let columns: Vec<Column> = client
                        .get(&format!("/column/{}", current.project_id))
                        .await?;
                    let labels: Vec<String> = columns.iter().map(|c| c.name.clone()).collect();
                    let idx = output::select("Status", &labels)?;
                    labels[idx].clone()
                }
                None => anyhow::bail!("status required (or run interactively in a terminal)"),
            };
            #[derive(Serialize)]
            struct Body {
                status: String,
            }
            let task: Task = client
                .put(
                    &format!("/task/status/{id}"),
                    &Body {
                        status: status.clone(),
                    },
                )
                .await?;

            if json {
                output::json_output(&task);
            } else {
                // Use the requested status — server response may return stale value
                output::success(false, &format!("Task '{}' → {}", task.title, status));
            }
        }

        TaskCommand::Priority { id, priority } => {
            let priority = match priority {
                Some(p) => p,
                None if output::is_interactive() => {
                    let labels: Vec<String> = PRIORITIES.iter().map(|s| (*s).to_string()).collect();
                    let idx = output::select("Priority", &labels)?;
                    labels[idx].clone()
                }
                None => anyhow::bail!("priority required (or run interactively in a terminal)"),
            };
            #[derive(Serialize)]
            struct Body {
                priority: String,
            }
            let task: Task = client
                .put(&format!("/task/priority/{id}"), &Body { priority })
                .await?;

            if json {
                output::json_output(&task);
            } else {
                output::success(
                    false,
                    &format!("Task '{}' priority → {}", task.title, task.priority),
                );
            }
        }

        TaskCommand::Assign { id, user_id } => {
            #[derive(Serialize)]
            #[serde(rename_all = "camelCase")]
            struct Body {
                user_id: String,
            }
            let task: Task = client
                .put(
                    &format!("/task/assignee/{id}"),
                    &Body {
                        user_id: user_id.clone(),
                    },
                )
                .await?;

            if json {
                output::json_output(&task);
            } else if user_id.is_empty() {
                output::success(false, &format!("Unassigned task '{}'", task.title));
            } else {
                output::success(
                    false,
                    &format!("Assigned task '{}' to {user_id}", task.title),
                );
            }
        }

        TaskCommand::DueDate { id, date } => {
            #[derive(Serialize)]
            #[serde(rename_all = "camelCase")]
            struct Body {
                due_date: Option<String>,
            }
            let task: Task = client
                .put(
                    &format!("/task/due-date/{id}"),
                    &Body {
                        due_date: date.clone(),
                    },
                )
                .await?;

            if json {
                output::json_output(&task);
            } else {
                match date {
                    Some(d) => output::success(false, &format!("Task '{}' due → {d}", task.title)),
                    None => {
                        output::success(false, &format!("Cleared due date for '{}'", task.title))
                    }
                }
            }
        }

        TaskCommand::Title { id, title } => {
            #[derive(Serialize)]
            struct Body {
                title: String,
            }
            let task: Task = client
                .put(&format!("/task/title/{id}"), &Body { title })
                .await?;

            if json {
                output::json_output(&task);
            } else {
                output::success(false, &format!("Task title → '{}'", task.title));
            }
        }

        TaskCommand::Description { id, description } => {
            #[derive(Serialize)]
            struct Body {
                description: String,
            }
            let task: Task = client
                .put(&format!("/task/description/{id}"), &Body { description })
                .await?;

            if json {
                output::json_output(&task);
            } else {
                output::success(false, &format!("Updated description for '{}'", task.title));
            }
        }

        TaskCommand::Delete { id } => {
            let result: serde_json::Value = client.delete(&format!("/task/{id}")).await?;

            if json {
                output::json_output(&result);
            } else {
                let title = result.get("title").and_then(|v| v.as_str()).unwrap_or(&id);
                output::success(false, &format!("Deleted task '{title}'"));
            }
        }

        TaskCommand::Export { project_id } => {
            let project_id = resolve_project(project_id, ctx)?;
            let data: serde_json::Value = client.get(&format!("/task/export/{project_id}")).await?;

            output::json_output(&data);
        }

        TaskCommand::Import { file } => {
            let project_id = resolve_project(None, ctx)?;
            let content = std::fs::read_to_string(&file)
                .map_err(|e| anyhow::anyhow!("reading {file}: {e}"))?;
            let body: serde_json::Value = serde_json::from_str(&content)
                .map_err(|e| anyhow::anyhow!("parsing {file}: {e}"))?;

            // Accept either { "tasks": [...] } or just [...]
            let import_body = if body.is_array() {
                serde_json::json!({ "tasks": body })
            } else {
                body
            };

            let result: serde_json::Value = client
                .post(&format!("/task/import/{project_id}"), &import_body)
                .await?;

            if json {
                output::json_output(&result);
            } else {
                let count = result
                    .get("imported")
                    .or_else(|| result.get("count"))
                    .and_then(|v| v.as_i64())
                    .unwrap_or(0);
                output::success(false, &format!("Imported {count} tasks"));
            }
        }

        TaskCommand::Upload {
            task_id,
            file,
            surface,
        } => {
            // Step 1: Read file and determine content type
            let path = std::path::Path::new(&file);
            let data = std::fs::read(path).map_err(|e| anyhow::anyhow!("reading {file}: {e}"))?;

            let filename = path
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| "upload".to_string());

            let content_type = match path.extension().and_then(|e| e.to_str()).unwrap_or("") {
                "png" => "image/png",
                "jpg" | "jpeg" => "image/jpeg",
                "gif" => "image/gif",
                "webp" => "image/webp",
                "svg" => "image/svg+xml",
                _ => "application/octet-stream",
            };

            let size = data.len();

            output::status(json, "↑", &format!("Uploading {filename} ({size} bytes)…"));

            // Step 2: Get presigned upload URL from Kaneo
            #[derive(Serialize)]
            #[serde(rename_all = "camelCase")]
            struct UploadRequest {
                filename: String,
                content_type: String,
                size: usize,
                surface: String,
            }

            let upload_info: serde_json::Value = client
                .put(
                    &format!("/task/image-upload/{task_id}"),
                    &UploadRequest {
                        filename: filename.clone(),
                        content_type: content_type.to_string(),
                        size,
                        surface: surface.clone(),
                    },
                )
                .await?;

            let presigned_url = upload_info
                .get("url")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow::anyhow!("no presigned URL in response"))?;

            let object_key = upload_info
                .get("key")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow::anyhow!("no object key in response"))?
                .to_string();

            // Step 3: Upload directly to S3
            upload_to_presigned_url(presigned_url, data, content_type).await?;

            // Step 4: Finalize — tell Kaneo the upload completed
            #[derive(Serialize)]
            #[serde(rename_all = "camelCase")]
            struct FinalizeRequest {
                key: String,
                filename: String,
                content_type: String,
                size: usize,
                surface: String,
            }

            let result: serde_json::Value = client
                .post(
                    &format!("/task/image-upload/{task_id}/finalize"),
                    &FinalizeRequest {
                        key: object_key,
                        filename: filename.clone(),
                        content_type: content_type.to_string(),
                        size,
                        surface,
                    },
                )
                .await?;

            if json {
                output::json_output(&result);
            } else {
                let asset_url = result
                    .get("url")
                    .and_then(|v| v.as_str())
                    .unwrap_or("(created)");
                let asset_id = result.get("id").and_then(|v| v.as_str()).unwrap_or("");
                output::success(
                    false,
                    &format!("Uploaded '{filename}' → {asset_id}\n    {asset_url}"),
                );
            }
        }

        TaskCommand::Asset { id, output: out } => {
            let (bytes, content_type) = client.get_bytes(&format!("/asset/{id}")).await?;

            let filename = out.unwrap_or_else(|| {
                let ext = match content_type.as_str() {
                    "image/png" => "png",
                    "image/jpeg" => "jpg",
                    "image/gif" => "gif",
                    "image/webp" => "webp",
                    "application/pdf" => "pdf",
                    _ => "bin",
                };
                format!("{id}.{ext}")
            });

            std::fs::write(&filename, &bytes)
                .map_err(|e| anyhow::anyhow!("writing {filename}: {e}"))?;

            if json {
                output::json_output(&serde_json::json!({
                    "file": filename,
                    "size": bytes.len(),
                    "content_type": content_type,
                }));
            } else {
                output::success(
                    false,
                    &format!("Downloaded {} ({} bytes)", filename, bytes.len()),
                );
            }
        }
    }

    Ok(())
}

/// Client-side filtering of board data by status (column name) and/or priority.
fn filter_board(
    board: &serde_json::Value,
    status: Option<&str>,
    priority: Option<&str>,
) -> serde_json::Value {
    if status.is_none() && priority.is_none() {
        return board.clone();
    }

    let columns_arr = board
        .as_array()
        .or_else(|| board.get("columns").and_then(|v| v.as_array()));

    let Some(columns) = columns_arr else {
        return board.clone();
    };

    let filtered: Vec<serde_json::Value> = columns
        .iter()
        .filter(|col| {
            if let Some(s) = status {
                let name = col.get("name").and_then(|v| v.as_str()).unwrap_or("");
                if !name.eq_ignore_ascii_case(s) {
                    return false;
                }
            }
            true
        })
        .map(|col| {
            if let Some(p) = priority {
                let mut col = col.clone();
                if let Some(tasks) = col.get("tasks").and_then(|v| v.as_array()) {
                    let filtered_tasks: Vec<&serde_json::Value> = tasks
                        .iter()
                        .filter(|t| {
                            t.get("priority")
                                .and_then(|v| v.as_str())
                                .is_some_and(|tp| tp.eq_ignore_ascii_case(p))
                        })
                        .collect();
                    col["tasks"] = serde_json::json!(filtered_tasks);
                }
                col
            } else {
                col.clone()
            }
        })
        .collect();

    serde_json::json!(filtered)
}

fn print_board(board: &serde_json::Value) {
    let bold = console::Style::new().bold();
    let dim = console::Style::new().dim();
    let cyan = console::Style::new().cyan();

    // API may return [...] (array of columns) or { "columns": [...] }
    let columns_arr = board
        .as_array()
        .or_else(|| board.get("columns").and_then(|v| v.as_array()));

    if let Some(columns) = columns_arr {
        for col in columns {
            let name = col.get("name").and_then(|v| v.as_str()).unwrap_or("?");
            let tasks = col.get("tasks").and_then(|v| v.as_array());
            let count = tasks.map(|t| t.len()).unwrap_or(0);

            eprintln!(
                "\n  {} {}",
                bold.apply_to(name),
                dim.apply_to(format!("({count})"))
            );

            if let Some(tasks) = tasks {
                for t in tasks {
                    let title = t.get("title").and_then(|v| v.as_str()).unwrap_or("?");
                    let id = t.get("id").and_then(|v| v.as_str()).unwrap_or("");
                    let number = t.get("number").and_then(|v| v.as_i64()).unwrap_or(0);
                    let priority = t.get("priority").and_then(|v| v.as_str()).unwrap_or("");

                    let prio_icon = match priority {
                        "urgent" => "🔴",
                        "high" => "🟠",
                        "medium" => "🟡",
                        "low" => "🟢",
                        _ => "⚪",
                    };

                    eprintln!(
                        "    {prio_icon} {} {} {}",
                        cyan.apply_to(format!("#{number}")),
                        title,
                        dim.apply_to(id),
                    );
                }
            }
        }
        eprintln!();
    } else {
        // Might be a different structure, just dump it
        eprintln!(
            "{}",
            serde_json::to_string_pretty(board).unwrap_or_default()
        );
    }
}

fn print_task(task: &Task) {
    let bold = console::Style::new().bold();
    let dim = console::Style::new().dim();

    let prio_icon = match task.priority.as_str() {
        "urgent" => "🔴",
        "high" => "🟠",
        "medium" => "🟡",
        "low" => "🟢",
        _ => "⚪",
    };

    eprintln!(
        "  {prio_icon} {} {}",
        bold.apply_to(format!("#{}", task.number.unwrap_or(0))),
        bold.apply_to(&task.title),
    );
    eprintln!("  {} {}", dim.apply_to("id:"), task.id);
    eprintln!("  {} {}", dim.apply_to("status:"), task.status);
    eprintln!("  {} {}", dim.apply_to("priority:"), task.priority);

    if let Some(uid) = &task.user_id {
        eprintln!("  {} {uid}", dim.apply_to("assignee:"));
    }
    if let Some(dd) = &task.due_date {
        eprintln!("  {} {dd}", dim.apply_to("due:"));
    }
    if let Some(desc) = &task.description
        && !desc.is_empty()
    {
        eprintln!("  {} {desc}", dim.apply_to("desc:"));
    }
}
