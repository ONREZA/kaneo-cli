use crate::api::client::upload_to_presigned_url;
use crate::api::types::{BulkUpdateResult, Column, CreateTaskBody, Task};
use crate::auth::ResolvedContext;
use crate::cli::resolve::{inject_task_ref, parse_task_ref, resolve_project, resolve_task_id};
use crate::cli::{TaskArgs, TaskCommand};
use crate::output;
use serde::Serialize;

use crate::api::ApiClient;

const PRIORITIES: &[&str] = &["no-priority", "low", "medium", "high", "urgent"];

fn priority_icon(priority: &str) -> &'static str {
    match priority {
        "urgent" => "🔴",
        "high" => "🟠",
        "medium" => "🟡",
        "low" => "🟢",
        _ => "⚪",
    }
}

/// Output a task as JSON with ref field injected.
async fn json_task(task: &Task, client: &ApiClient) -> anyhow::Result<()> {
    let mut val = serde_json::to_value(task)?;
    inject_task_ref(&mut val, &task.project_id, task.number, None, client).await;
    output::json_output(&val);
    Ok(())
}

pub async fn run(args: TaskArgs, ctx: &ResolvedContext, json: bool) -> anyhow::Result<()> {
    let client = ApiClient::new(&ctx.api_url, &ctx.api_key)?;

    match args.command {
        TaskCommand::List {
            project_id,
            status,
            priority,
            assignee,
            page,
            limit,
            sort_by,
            sort_order,
            due_before,
            due_after,
            all,
            board: board_flag,
        } => {
            let project_id = resolve_project(project_id, ctx)?;

            // Don't send "planned"/"archived" as server-side status filter —
            // these are virtual statuses that we filter client-side
            let is_virtual_status = status
                .as_deref()
                .is_some_and(|s| s == "planned" || s == "archived");

            let mut query: Vec<(&str, String)> = Vec::new();
            if let Some(s) = &status
                && !is_virtual_status
            {
                query.push(("status", s.clone()));
            }
            if let Some(p) = &priority {
                query.push(("priority", p.clone()));
            }
            if let Some(a) = &assignee {
                query.push(("assigneeId", a.clone()));
            }
            if let Some(l) = limit {
                query.push(("limit", l.to_string()));
            }
            if let Some(p) = page {
                query.push(("page", p.to_string()));
            }
            if let Some(sb) = &sort_by {
                query.push(("sortBy", sb.as_api_str().to_string()));
            }
            if let Some(so) = &sort_order {
                query.push(("sortOrder", so.as_api_str().to_string()));
            }
            if let Some(db) = &due_before {
                query.push(("dueBefore", db.clone()));
            }
            if let Some(da) = &due_after {
                query.push(("dueAfter", da.clone()));
            }

            let raw_board: serde_json::Value = client
                .get_query(&format!("/task/tasks/{project_id}"), &query)
                .await?;

            if board_flag {
                // Board view: raw structure for JSON, kanban for human
                if json {
                    output::json_output(&raw_board);
                } else {
                    print_board(&raw_board);
                }
            } else {
                // Default: flat list — same data for both JSON and human
                let tasks = flatten_board(&raw_board, status.as_deref(), all);
                if json {
                    output::json_output(&tasks);
                } else {
                    print_task_table(&tasks);
                }
            }
        }

        TaskCommand::Get { id } => {
            let known_slug = parse_task_ref(&id).map(|(s, _)| s.to_string());
            let id = resolve_task_id(&id, ctx, &client).await?;
            let task: Task = client.get(&format!("/task/{id}")).await?;

            let mut val = serde_json::to_value(&task)?;
            inject_task_ref(
                &mut val,
                &task.project_id,
                task.number,
                known_slug.as_deref(),
                &client,
            )
            .await;

            if json {
                output::json_output(&val);
            } else {
                let task_ref = val.get("ref").and_then(|v| v.as_str()).map(String::from);
                print_task(&task, task_ref.as_deref());
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
                json_task(&task, &client).await?;
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
            let id = resolve_task_id(&id, ctx, &client).await?;
            let status = match status {
                Some(s) => s,
                None if output::is_interactive() => {
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
                .put(&format!("/task/status/{id}"), &Body { status })
                .await?;

            if json {
                json_task(&task, &client).await?;
            } else {
                output::success(false, &format!("Task '{}' → {}", task.title, task.status));
            }
        }

        TaskCommand::Priority { id, priority } => {
            let id = resolve_task_id(&id, ctx, &client).await?;
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
                json_task(&task, &client).await?;
            } else {
                output::success(
                    false,
                    &format!("Task '{}' priority → {}", task.title, task.priority),
                );
            }
        }

        TaskCommand::Assign { id, user_id } => {
            let id = resolve_task_id(&id, ctx, &client).await?;
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
                json_task(&task, &client).await?;
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
            let id = resolve_task_id(&id, ctx, &client).await?;
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
                json_task(&task, &client).await?;
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
            let id = resolve_task_id(&id, ctx, &client).await?;
            #[derive(Serialize)]
            struct Body {
                title: String,
            }
            let task: Task = client
                .put(&format!("/task/title/{id}"), &Body { title })
                .await?;

            if json {
                json_task(&task, &client).await?;
            } else {
                output::success(false, &format!("Task title → '{}'", task.title));
            }
        }

        TaskCommand::Description { id, description } => {
            let id = resolve_task_id(&id, ctx, &client).await?;
            #[derive(Serialize)]
            struct Body {
                description: String,
            }
            let task: Task = client
                .put(&format!("/task/description/{id}"), &Body { description })
                .await?;

            if json {
                json_task(&task, &client).await?;
            } else {
                output::success(false, &format!("Updated description for '{}'", task.title));
            }
        }

        TaskCommand::Delete { id } => {
            let id = resolve_task_id(&id, ctx, &client).await?;
            let task: Task = client.delete(&format!("/task/{id}")).await?;

            if json {
                json_task(&task, &client).await?;
            } else {
                output::success(false, &format!("Deleted task '{}'", task.title));
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
            let task_id = resolve_task_id(&task_id, ctx, &client).await?;
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

            upload_to_presigned_url(presigned_url, data, content_type).await?;

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

        TaskCommand::Bulk {
            task_ids,
            operation,
            value,
        } => {
            let raw_ids: Vec<&str> = task_ids.split(',').map(|s| s.trim()).collect();
            let mut ids = Vec::with_capacity(raw_ids.len());
            for raw in &raw_ids {
                ids.push(resolve_task_id(raw, ctx, &client).await?);
            }
            #[derive(Serialize)]
            #[serde(rename_all = "camelCase")]
            struct Body {
                task_ids: Vec<String>,
                operation: String,
                #[serde(skip_serializing_if = "Option::is_none")]
                value: Option<String>,
            }
            let result: BulkUpdateResult = client
                .patch(
                    "/task/bulk",
                    &Body {
                        task_ids: ids,
                        operation: operation.as_api_str().to_string(),
                        value,
                    },
                )
                .await?;

            if json {
                output::json_output(&result);
            } else {
                output::success(
                    false,
                    &format!(
                        "Bulk operation complete: {} tasks updated",
                        result.updated_count
                    ),
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

/// Flatten the board response into a simple array of task objects.
/// Each task gets a `ref` field (e.g. "DEP-42") computed from the project slug.
/// By default, archived tasks are excluded unless `include_all` is true.
/// If `status_filter` is set, only tasks matching that status are returned.
fn flatten_board(
    board: &serde_json::Value,
    status_filter: Option<&str>,
    include_all: bool,
) -> Vec<serde_json::Value> {
    let root = board.get("data").unwrap_or(board);
    let slug = root.get("slug").and_then(|v| v.as_str()).unwrap_or("");

    let mut tasks: Vec<serde_json::Value> = Vec::new();

    // Collect tasks from columns
    let columns = root
        .as_array()
        .or_else(|| root.get("columns").and_then(|v| v.as_array()));
    if let Some(cols) = columns {
        for col in cols {
            if let Some(col_tasks) = col.get("tasks").and_then(|v| v.as_array()) {
                for t in col_tasks {
                    tasks.push(t.clone());
                }
            }
        }
    }

    // Collect planned tasks
    if let Some(planned) = root.get("plannedTasks").and_then(|v| v.as_array()) {
        for t in planned {
            tasks.push(t.clone());
        }
    }

    // Collect archived tasks (only if --all or --status archived)
    if (include_all || status_filter == Some("archived"))
        && let Some(archived) = root.get("archivedTasks").and_then(|v| v.as_array())
    {
        for t in archived {
            tasks.push(t.clone());
        }
    }

    // Add ref field to each task
    if !slug.is_empty() {
        for t in &mut tasks {
            if let Some(number) = t.get("number").and_then(|v| v.as_i64())
                && let Some(obj) = t.as_object_mut()
            {
                obj.insert(
                    "ref".to_string(),
                    serde_json::Value::String(format!("{slug}-{number}")),
                );
            }
        }
    }

    // Client-side status filter (for "planned", "archived", or any status)
    if let Some(filter) = status_filter {
        tasks.retain(|t| {
            t.get("status")
                .and_then(|v| v.as_str())
                .is_some_and(|s| s.eq_ignore_ascii_case(filter))
        });
    }

    tasks
}

fn print_task_table(tasks: &[serde_json::Value]) {
    if tasks.is_empty() {
        output::warn(false, "No tasks found");
        return;
    }

    let bold = console::Style::new().bold();
    let dim = console::Style::new().dim();
    let cyan = console::Style::new().cyan();

    for t in tasks {
        let ref_str = t.get("ref").and_then(|v| v.as_str()).unwrap_or("");
        let title = t.get("title").and_then(|v| v.as_str()).unwrap_or("?");
        let status = t.get("status").and_then(|v| v.as_str()).unwrap_or("");
        let priority = t.get("priority").and_then(|v| v.as_str()).unwrap_or("");
        let prio_icon = priority_icon(priority);

        eprintln!(
            "  {prio_icon} {} {} {} {}",
            cyan.apply_to(ref_str),
            bold.apply_to(title),
            dim.apply_to(status),
            dim.apply_to(t.get("assigneeName").and_then(|v| v.as_str()).unwrap_or("")),
        );
    }
    eprintln!();
}

fn print_board(board: &serde_json::Value) {
    let bold = console::Style::new().bold();
    let dim = console::Style::new().dim();
    let cyan = console::Style::new().cyan();

    // Handle both formats: direct { columns, ... } and wrapped { data: { columns, ... }, pagination }
    let root = board.get("data").unwrap_or(board);

    let columns_arr = root
        .as_array()
        .or_else(|| root.get("columns").and_then(|v| v.as_array()));

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
                    print_task_row(t, &cyan, &dim);
                }
            }
        }

        // Show planned tasks if present
        if let Some(planned) = root.get("plannedTasks").and_then(|v| v.as_array())
            && !planned.is_empty()
        {
            eprintln!(
                "\n  {} {}",
                bold.apply_to("Planned"),
                dim.apply_to(format!("({})", planned.len()))
            );
            for t in planned {
                print_task_row(t, &cyan, &dim);
            }
        }

        // Show archived tasks if present
        if let Some(archived) = root.get("archivedTasks").and_then(|v| v.as_array())
            && !archived.is_empty()
        {
            eprintln!(
                "\n  {} {}",
                bold.apply_to("Archived"),
                dim.apply_to(format!("({})", archived.len()))
            );
            for t in archived {
                print_task_row(t, &cyan, &dim);
            }
        }

        // Show pagination info if present
        if let Some(pag) = board.get("pagination") {
            let page = pag.get("page").and_then(|v| v.as_i64()).unwrap_or(1);
            let total_pages = pag.get("totalPages").and_then(|v| v.as_i64()).unwrap_or(1);
            let total = pag.get("total").and_then(|v| v.as_i64()).unwrap_or(0);
            if total_pages > 1 {
                eprintln!(
                    "\n  {} page {page}/{total_pages} ({total} total)",
                    dim.apply_to("…"),
                );
            }
        }

        eprintln!();
    } else {
        eprintln!(
            "{}",
            serde_json::to_string_pretty(board).unwrap_or_default()
        );
    }
}

fn print_task_row(t: &serde_json::Value, cyan: &console::Style, dim: &console::Style) {
    let title = t.get("title").and_then(|v| v.as_str()).unwrap_or("?");
    let id = t.get("id").and_then(|v| v.as_str()).unwrap_or("");
    let number = t.get("number").and_then(|v| v.as_i64()).unwrap_or(0);
    let priority = t.get("priority").and_then(|v| v.as_str()).unwrap_or("");
    let prio_icon = priority_icon(priority);

    eprintln!(
        "    {prio_icon} {} {} {}",
        cyan.apply_to(format!("#{number}")),
        title,
        dim.apply_to(id),
    );
}

fn print_task(task: &Task, task_ref: Option<&str>) {
    let bold = console::Style::new().bold();
    let dim = console::Style::new().dim();
    let cyan = console::Style::new().cyan();

    let prio_icon = priority_icon(&task.priority);

    let ref_label = task_ref
        .map(|r| r.to_string())
        .unwrap_or_else(|| format!("#{}", task.number.unwrap_or(0)));

    eprintln!(
        "  {prio_icon} {} {}",
        cyan.apply_to(&ref_label),
        bold.apply_to(&task.title),
    );
    eprintln!("  {} {}", dim.apply_to("id:"), task.id);
    if let Some(r) = task_ref {
        eprintln!("  {} {r}", dim.apply_to("ref:"));
    }
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
