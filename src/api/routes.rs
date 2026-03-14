// Central registry of all API routes used by the CLI.
//
// Each entry is (HTTP_METHOD, PATH_PATTERN) where PATH_PATTERN uses `{id}`,
// `{projectId}`, etc. as placeholders for dynamic segments.
//
// This module is the single source of truth — handlers should use these
// constants, and the integration test validates them against the OpenAPI spec.

// --- Auth / Session ---
pub const AUTH_GET_SESSION: (&str, &str) = ("GET", "/auth/get-session");

// --- Organization (Workspace) ---
pub const ORG_LIST: (&str, &str) = ("GET", "/auth/organization/list");
pub const ORG_GET_FULL: (&str, &str) = ("GET", "/auth/organization/get-full-organization");
pub const ORG_CREATE: (&str, &str) = ("POST", "/auth/organization/create");
pub const ORG_UPDATE: (&str, &str) = ("POST", "/auth/organization/update");
pub const ORG_DELETE: (&str, &str) = ("POST", "/auth/organization/delete");
pub const ORG_INVITE_MEMBER: (&str, &str) = ("POST", "/auth/organization/invite-member");
pub const ORG_REMOVE_MEMBER: (&str, &str) = ("POST", "/auth/organization/remove-member");
pub const ORG_UPDATE_MEMBER_ROLE: (&str, &str) = ("POST", "/auth/organization/update-member-role");
pub const ORG_LEAVE: (&str, &str) = ("POST", "/auth/organization/leave");
pub const ORG_SET_ACTIVE: (&str, &str) = ("POST", "/auth/organization/set-active");
pub const ORG_CHECK_SLUG: (&str, &str) = ("POST", "/auth/organization/check-slug");
pub const ORG_LIST_INVITATIONS: (&str, &str) = ("GET", "/auth/organization/list-invitations");
pub const ORG_CANCEL_INVITATION: (&str, &str) = ("POST", "/auth/organization/cancel-invitation");
pub const ORG_ACCEPT_INVITATION: (&str, &str) = ("POST", "/auth/organization/accept-invitation");
pub const ORG_REJECT_INVITATION: (&str, &str) = ("POST", "/auth/organization/reject-invitation");

// --- Project ---
pub const PROJECT_LIST: (&str, &str) = ("GET", "/project");
pub const PROJECT_CREATE: (&str, &str) = ("POST", "/project");
pub const PROJECT_GET: (&str, &str) = ("GET", "/project/{id}");
pub const PROJECT_UPDATE: (&str, &str) = ("PUT", "/project/{id}");
pub const PROJECT_DELETE: (&str, &str) = ("DELETE", "/project/{id}");

// --- Task ---
pub const TASK_LIST: (&str, &str) = ("GET", "/task/tasks/{projectId}");
pub const TASK_GET: (&str, &str) = ("GET", "/task/{id}");
pub const TASK_CREATE: (&str, &str) = ("POST", "/task/{projectId}");
pub const TASK_UPDATE: (&str, &str) = ("PUT", "/task/{id}");
pub const TASK_DELETE: (&str, &str) = ("DELETE", "/task/{id}");
pub const TASK_UPDATE_STATUS: (&str, &str) = ("PUT", "/task/status/{id}");
pub const TASK_UPDATE_PRIORITY: (&str, &str) = ("PUT", "/task/priority/{id}");
pub const TASK_UPDATE_ASSIGNEE: (&str, &str) = ("PUT", "/task/assignee/{id}");
pub const TASK_UPDATE_TITLE: (&str, &str) = ("PUT", "/task/title/{id}");
pub const TASK_UPDATE_DESCRIPTION: (&str, &str) = ("PUT", "/task/description/{id}");
pub const TASK_UPDATE_DUE_DATE: (&str, &str) = ("PUT", "/task/due-date/{id}");
pub const TASK_EXPORT: (&str, &str) = ("GET", "/task/export/{projectId}");
pub const TASK_IMPORT: (&str, &str) = ("POST", "/task/import/{projectId}");
pub const TASK_IMAGE_UPLOAD: (&str, &str) = ("PUT", "/task/image-upload/{id}");
pub const TASK_IMAGE_UPLOAD_FINALIZE: (&str, &str) = ("POST", "/task/image-upload/{id}/finalize");

// --- Column ---
pub const COLUMN_LIST: (&str, &str) = ("GET", "/column/{projectId}");
pub const COLUMN_CREATE: (&str, &str) = ("POST", "/column/{projectId}");
pub const COLUMN_UPDATE: (&str, &str) = ("PUT", "/column/{id}");
pub const COLUMN_DELETE: (&str, &str) = ("DELETE", "/column/{id}");
pub const COLUMN_REORDER: (&str, &str) = ("PUT", "/column/reorder/{projectId}");

// --- Label ---
pub const LABEL_LIST_WORKSPACE: (&str, &str) = ("GET", "/label/workspace/{workspaceId}");
pub const LABEL_LIST_TASK: (&str, &str) = ("GET", "/label/task/{taskId}");
pub const LABEL_CREATE: (&str, &str) = ("POST", "/label");
pub const LABEL_UPDATE: (&str, &str) = ("PUT", "/label/{id}");
pub const LABEL_DELETE: (&str, &str) = ("DELETE", "/label/{id}");
pub const LABEL_ATTACH: (&str, &str) = ("PUT", "/label/{id}/task");
pub const LABEL_DETACH: (&str, &str) = ("DELETE", "/label/{id}/task");

// --- Activity ---
pub const ACTIVITY_LIST: (&str, &str) = ("GET", "/activity/{taskId}");
pub const ACTIVITY_CREATE_COMMENT: (&str, &str) = ("POST", "/activity/comment");
pub const ACTIVITY_UPDATE_COMMENT: (&str, &str) = ("PUT", "/activity/comment");
pub const ACTIVITY_DELETE_COMMENT: (&str, &str) = ("DELETE", "/activity/comment");

// --- Notification ---
pub const NOTIFICATION_LIST: (&str, &str) = ("GET", "/notification");
pub const NOTIFICATION_MARK_READ: (&str, &str) = ("PATCH", "/notification/{id}/read");
pub const NOTIFICATION_MARK_ALL_READ: (&str, &str) = ("PATCH", "/notification/read-all");
pub const NOTIFICATION_CLEAR_ALL: (&str, &str) = ("DELETE", "/notification/clear-all");

// --- Time Entry ---
pub const TIME_ENTRY_LIST: (&str, &str) = ("GET", "/time-entry/task/{taskId}");
pub const TIME_ENTRY_GET: (&str, &str) = ("GET", "/time-entry/{id}");
pub const TIME_ENTRY_CREATE: (&str, &str) = ("POST", "/time-entry");
pub const TIME_ENTRY_UPDATE: (&str, &str) = ("PUT", "/time-entry/{id}");

// --- Search ---
pub const SEARCH: (&str, &str) = ("GET", "/search");

// --- Asset ---
pub const ASSET_GET: (&str, &str) = ("GET", "/asset/{id}");

// --- OpenAPI ---
pub const OPENAPI: (&str, &str) = ("GET", "/openapi");

/// All Kaneo API routes used by the CLI, validated against the OpenAPI spec.
///
/// Duplicates in this list will cause test failures.
pub const ALL_ROUTES: &[(&str, &str)] = &[
    AUTH_GET_SESSION,
    ORG_LIST,
    ORG_GET_FULL,
    ORG_CREATE,
    ORG_UPDATE,
    ORG_DELETE,
    ORG_INVITE_MEMBER,
    ORG_REMOVE_MEMBER,
    ORG_UPDATE_MEMBER_ROLE,
    ORG_LEAVE,
    ORG_SET_ACTIVE,
    ORG_CHECK_SLUG,
    ORG_LIST_INVITATIONS,
    ORG_CANCEL_INVITATION,
    ORG_ACCEPT_INVITATION,
    ORG_REJECT_INVITATION,
    PROJECT_LIST,
    PROJECT_CREATE,
    PROJECT_GET,
    PROJECT_UPDATE,
    PROJECT_DELETE,
    TASK_LIST,
    TASK_GET,
    TASK_CREATE,
    TASK_UPDATE,
    TASK_DELETE,
    TASK_UPDATE_STATUS,
    TASK_UPDATE_PRIORITY,
    TASK_UPDATE_ASSIGNEE,
    TASK_UPDATE_TITLE,
    TASK_UPDATE_DESCRIPTION,
    TASK_UPDATE_DUE_DATE,
    TASK_EXPORT,
    TASK_IMPORT,
    TASK_IMAGE_UPLOAD,
    TASK_IMAGE_UPLOAD_FINALIZE,
    COLUMN_LIST,
    COLUMN_CREATE,
    COLUMN_UPDATE,
    COLUMN_DELETE,
    COLUMN_REORDER,
    LABEL_LIST_WORKSPACE,
    LABEL_LIST_TASK,
    LABEL_CREATE,
    LABEL_UPDATE,
    LABEL_DELETE,
    LABEL_ATTACH,
    LABEL_DETACH,
    ACTIVITY_LIST,
    ACTIVITY_CREATE_COMMENT,
    ACTIVITY_UPDATE_COMMENT,
    ACTIVITY_DELETE_COMMENT,
    NOTIFICATION_LIST,
    NOTIFICATION_MARK_READ,
    NOTIFICATION_MARK_ALL_READ,
    NOTIFICATION_CLEAR_ALL,
    TIME_ENTRY_LIST,
    TIME_ENTRY_GET,
    TIME_ENTRY_CREATE,
    TIME_ENTRY_UPDATE,
    SEARCH,
    ASSET_GET,
];

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn no_duplicate_routes() {
        let mut seen = HashSet::new();
        for (method, path) in ALL_ROUTES {
            let key = format!("{method} {path}");
            assert!(seen.insert(key.clone()), "duplicate route: {key}");
        }
    }

    #[test]
    fn all_routes_have_valid_methods() {
        let valid = ["GET", "POST", "PUT", "PATCH", "DELETE"];
        for (method, path) in ALL_ROUTES {
            assert!(
                valid.contains(method),
                "invalid HTTP method '{method}' for {path}"
            );
        }
    }

    #[test]
    fn all_paths_start_with_slash() {
        for (method, path) in ALL_ROUTES {
            assert!(
                path.starts_with('/'),
                "{method} {path} — path must start with '/'"
            );
        }
    }

    #[test]
    fn route_count_sanity() {
        // Guard against accidentally emptying the list
        assert!(
            ALL_ROUTES.len() >= 50,
            "expected at least 50 routes, got {}",
            ALL_ROUTES.len()
        );
    }

    #[test]
    fn path_params_use_braces() {
        for (method, path) in ALL_ROUTES {
            // No colons (Express-style :id), only {id}
            assert!(
                !path.contains(':'),
                "{method} {path} — use {{param}} not :param"
            );
        }
    }
}
