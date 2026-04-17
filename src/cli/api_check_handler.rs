use crate::api::ApiClient;
use crate::auth::ResolvedContext;
use crate::output;

/// All operation IDs that the CLI covers, mapped to the Kaneo OpenAPI spec.
const EXPECTED_OPERATIONS: &[(&str, &str)] = &[
    // Projects
    ("listProjects", "project ls"),
    ("getProject", "project get"),
    ("createProject", "project create"),
    ("updateProject", "project update"),
    ("deleteProject", "project delete"),
    ("archiveProject", "project archive"),
    ("unarchiveProject", "project unarchive"),
    // Tasks
    ("listTasks", "task ls"),
    ("getTask", "task get"),
    ("createTask", "task create"),
    ("updateTask", "task start-date (via generic update)"),
    ("updateTaskStatus", "task status"),
    ("updateTaskPriority", "task priority"),
    ("updateTaskAssignee", "task assign"),
    ("updateTaskTitle", "task title"),
    ("updateTaskDescription", "task description"),
    ("updateTaskDueDate", "task due-date"),
    ("deleteTask", "task delete"),
    ("exportTasks", "task export"),
    ("importTasks", "task import"),
    ("createTaskImageUpload", "task upload (step 1)"),
    ("finalizeTaskImageUpload", "task upload (step 2)"),
    // Columns
    ("getColumns", "column ls"),
    ("createColumn", "column create"),
    ("updateColumn", "column update"),
    ("reorderColumns", "column reorder"),
    ("deleteColumn", "column delete"),
    // Labels
    ("getTaskLabels", "task label ls"),
    ("getWorkspaceLabels", "label ls"),
    ("createLabel", "label create"),
    ("getLabel", "label get"),
    ("updateLabel", "label update"),
    ("deleteLabel", "label delete"),
    ("attachLabelToTask", "task label add"),
    ("detachLabelFromTask", "task label rm"),
    // Activity / Comments (via activity endpoints)
    ("getActivities", "task comment ls / task get"),
    ("createComment", "task comment add"),
    ("updateComment", "task comment edit"),
    ("deleteComment", "task comment rm"),
    // Notifications
    ("listNotifications", "notification ls"),
    ("createNotification", "notification create"),
    ("markNotificationAsRead", "notification read"),
    ("markAllNotificationsAsRead", "notification read-all"),
    ("clearAllNotifications", "notification clear-all"),
    // Time Entries
    ("getTaskTimeEntries", "task time ls"),
    ("getTimeEntry", "task time get"),
    ("createTimeEntry", "task time add"),
    ("updateTimeEntry", "task time edit"),
    // Search
    ("globalSearch", "search"),
    // Workspace members (dedicated endpoint)
    ("getWorkspaceMembers", "workspace members"),
    // Comments (first-class, server-only — CLI uses activity endpoints)
    ("getTaskComments", "server-only (unused by CLI)"),
    ("createTaskComment", "server-only (unused by CLI)"),
    ("updateTaskComment", "server-only (unused by CLI)"),
    ("deleteTaskComment", "server-only (unused by CLI)"),
    // Task Relations
    ("getTaskRelations", "task rel ls"),
    ("createTaskRelation", "task rel add"),
    ("deleteTaskRelation", "task rel rm"),
    // Bulk operations
    ("bulkUpdateTasks", "task bulk"),
    // Move task
    ("moveTask", "task transfer"),
    // Internal
    ("createActivity", "internal"),
    ("getConfig", "internal"),
    // Invitations
    ("getUserPendingInvitations", "invitation pending"),
    ("getInvitationDetails", "invitation get"),
    // External Links
    ("getExternalLinksByTask", "task links"),
    // Workflow Rules
    ("getWorkflowRules", "workflow-rule ls"),
    ("upsertWorkflowRule", "workflow-rule upsert"),
    ("deleteWorkflowRule", "workflow-rule delete"),
    // Auth / Assets
    ("getSession", "whoami / internal"),
    ("getAsset", "task asset"),
    // Organization (workspace) CRUD
    ("createOrganization", "workspace create"),
    ("updateOrganization", "workspace update"),
    ("deleteOrganization", "workspace delete"),
    ("listOrganization", "workspace ls / link"),
    ("getOrganizationFullOrganization", "workspace get"),
    ("inviteOrganizationMember", "workspace invite"),
    ("removeOrganizationMember", "workspace remove-member"),
    ("updateOrganizationMemberRole", "workspace update-role"),
    ("listOrganizationMembers", "workspace members"),
    ("listOrganizationInvitations", "workspace invitations"),
    (
        "acceptOrganizationInvitation",
        "workspace accept-invitation",
    ),
    (
        "rejectOrganizationInvitation",
        "workspace reject-invitation",
    ),
    (
        "cancelOrganizationInvitation",
        "workspace cancel-invitation",
    ),
    ("leaveOrganization", "workspace leave"),
    ("setOrganizationActive", "workspace set-active"),
    ("checkOrganizationSlug", "workspace check-slug"),
    ("getOrganizationActiveMember", "workspace me"),
    // Notification preferences
    ("getNotificationPreferences", "notification prefs show"),
    ("updateNotificationPreferences", "notification prefs set"),
    (
        "upsertNotificationPreferenceWorkspaceRule",
        "notification prefs workspace",
    ),
    (
        "deleteNotificationPreferenceWorkspaceRule",
        "notification prefs delete-workspace",
    ),
];

pub async fn run(ctx: &ResolvedContext, json: bool) -> anyhow::Result<()> {
    // OpenAPI endpoint is public — no auth needed
    let client = ApiClient::anonymous(&ctx.api_url)?;

    output::status(json, "↓", "Fetching OpenAPI spec from server…");

    let spec: serde_json::Value = client.get("/openapi").await?;

    // Extract all operationIds from the spec
    let mut server_ops: Vec<String> = Vec::new();
    if let Some(paths) = spec.get("paths").and_then(|v| v.as_object()) {
        for (_path, methods) in paths {
            if let Some(methods) = methods.as_object() {
                for (_method, operation) in methods {
                    if let Some(op_id) = operation.get("operationId").and_then(|v| v.as_str()) {
                        server_ops.push(op_id.to_string());
                    }
                }
            }
        }
    }

    server_ops.sort();
    server_ops.dedup();

    if json {
        let mut covered = Vec::new();
        let mut missing_in_server = Vec::new();
        let mut new_in_server = Vec::new();

        for (op_id, cli_cmd) in EXPECTED_OPERATIONS {
            if server_ops.contains(&op_id.to_string()) {
                covered.push(serde_json::json!({
                    "operationId": op_id,
                    "command": cli_cmd,
                    "status": "ok"
                }));
            } else {
                missing_in_server.push(serde_json::json!({
                    "operationId": op_id,
                    "command": cli_cmd,
                    "status": "missing_in_server"
                }));
            }
        }

        let known_ops: Vec<&str> = EXPECTED_OPERATIONS.iter().map(|(op, _)| *op).collect();
        for op in &server_ops {
            if !known_ops.contains(&op.as_str()) {
                new_in_server.push(op.clone());
            }
        }

        output::json_output(&serde_json::json!({
            "server_operations": server_ops.len(),
            "cli_operations": EXPECTED_OPERATIONS.len(),
            "covered": covered,
            "missing_in_server": missing_in_server,
            "new_in_server": new_in_server,
        }));

        return Ok(());
    }

    let green = console::Style::new().green();
    let red = console::Style::new().red();
    let yellow = console::Style::new().yellow();
    let bold = console::Style::new().bold();
    let dim = console::Style::new().dim();

    eprintln!(
        "\n  {} {} server operations, {} CLI mappings\n",
        bold.apply_to("API Check"),
        server_ops.len(),
        EXPECTED_OPERATIONS.len(),
    );

    // Check our expected operations against server
    let mut ok_count = 0;
    let mut missing_count = 0;

    for (op_id, cli_cmd) in EXPECTED_OPERATIONS {
        if server_ops.contains(&op_id.to_string()) {
            ok_count += 1;
            eprintln!(
                "  {} {} → {}",
                green.apply_to("✓"),
                dim.apply_to(op_id),
                cli_cmd,
            );
        } else {
            missing_count += 1;
            eprintln!(
                "  {} {} → {} {}",
                red.apply_to("✗"),
                dim.apply_to(op_id),
                cli_cmd,
                red.apply_to("(not found on server)"),
            );
        }
    }

    // Find server operations we don't cover
    let known_ops: Vec<&str> = EXPECTED_OPERATIONS.iter().map(|(op, _)| *op).collect();
    let new_ops: Vec<&String> = server_ops
        .iter()
        .filter(|op| !known_ops.contains(&op.as_str()))
        .collect();

    if !new_ops.is_empty() {
        eprintln!(
            "\n  {} New server operations (not in CLI):",
            yellow.apply_to("!")
        );
        for op in &new_ops {
            eprintln!("    {} {op}", yellow.apply_to("→"));
        }
    }

    eprintln!();
    if missing_count == 0 && new_ops.is_empty() {
        output::success(false, &format!("All {ok_count} operations matched"));
    } else {
        eprintln!(
            "  {} {ok_count} matched, {missing_count} missing on server, {} new on server",
            bold.apply_to("Summary:"),
            new_ops.len(),
        );
    }

    Ok(())
}
