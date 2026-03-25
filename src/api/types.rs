use serde::{Deserialize, Serialize};

// --- Project ---

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: String,
    pub workspace_id: String,
    pub slug: String,
    pub icon: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub created_at: String,
    pub is_public: Option<bool>,
    #[serde(default)]
    pub archived_at: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateProjectBody {
    pub name: String,
    pub workspace_id: String,
    pub icon: String,
    pub slug: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateProjectBody {
    pub name: String,
    pub icon: String,
    pub slug: String,
    pub description: String,
    pub is_public: bool,
}

// --- Task ---

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub id: String,
    pub project_id: String,
    pub position: Option<f64>,
    pub number: Option<i64>,
    pub user_id: Option<String>,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub priority: String,
    pub due_date: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTaskBody {
    pub title: String,
    pub description: String,
    pub priority: String,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTaskBody {
    pub title: String,
    pub description: String,
    pub priority: String,
    pub status: String,
    pub project_id: String,
    pub position: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

// --- Column ---

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Column {
    pub id: String,
    pub project_id: String,
    pub name: String,
    pub position: i64,
    pub icon: Option<String>,
    pub color: Option<String>,
    pub is_final: Option<bool>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateColumnBody {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_final: Option<bool>,
}

// --- Label ---

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Label {
    pub id: String,
    pub name: String,
    pub color: String,
    pub created_at: String,
    pub task_id: Option<String>,
    pub workspace_id: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateLabelBody {
    pub name: String,
    pub color: String,
    pub workspace_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
}

// --- Activity ---

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Activity {
    pub id: String,
    pub task_id: String,
    pub r#type: String,
    pub created_at: String,
    pub user_id: Option<String>,
    pub content: Option<String>,
    #[serde(default)]
    pub external_user_name: Option<String>,
    #[serde(default)]
    pub external_user_avatar: Option<String>,
    #[serde(default)]
    pub external_source: Option<String>,
    #[serde(default)]
    pub external_url: Option<String>,
}

// --- Notification ---

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Notification {
    pub id: String,
    pub user_id: String,
    pub title: String,
    pub content: Option<String>,
    pub r#type: String,
    pub is_read: Option<bool>,
    pub resource_id: Option<String>,
    pub resource_type: Option<String>,
    pub created_at: String,
}

// --- Time Entry ---

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeEntry {
    pub id: String,
    pub task_id: String,
    pub user_id: Option<String>,
    pub description: Option<String>,
    pub start_time: String,
    pub end_time: Option<String>,
    pub duration: Option<i64>,
    pub created_at: String,
}

// --- Search ---

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResult {
    pub tasks: Vec<Task>,
}

// --- Workspace (from better-auth organization) ---

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Workspace {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub created_at: String,
    #[serde(default)]
    pub logo: Option<String>,
    #[serde(default)]
    pub metadata: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceMember {
    pub id: String,
    pub user_id: String,
    pub role: String,
}

// --- Tasks with columns (board view) ---

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BoardColumn {
    pub id: String,
    pub name: String,
    pub position: i64,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub color: Option<String>,
    #[serde(default)]
    pub tasks: Vec<Task>,
}

// --- User session ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionResponse {
    pub user: Option<SessionUser>,
    pub session: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionUser {
    pub id: String,
    pub name: String,
    pub email: String,
    #[serde(default)]
    pub image: Option<String>,
}

// --- Config ---

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerConfig {
    pub disable_registration: Option<bool>,
    pub is_demo_mode: bool,
    pub has_smtp: bool,
    pub has_github_sign_in: Option<bool>,
    pub has_google_sign_in: Option<bool>,
    pub has_discord_sign_in: Option<bool>,
    #[serde(default)]
    pub disable_password_registration: Option<bool>,
    #[serde(default, rename = "hasCustomOAuth")]
    pub has_custom_oauth: Option<bool>,
    #[serde(default)]
    pub has_guest_access: Option<bool>,
}

// --- Comment (first-class) ---

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    pub id: String,
    pub task_id: String,
    pub user_id: String,
    pub content: String,
    pub created_at: String,
    pub updated_at: String,
    #[serde(default)]
    pub user: Option<CommentUser>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentUser {
    pub name: String,
    #[serde(default)]
    pub image: Option<String>,
}

// --- Task Relation ---

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskRelation {
    pub id: String,
    pub source_task_id: String,
    pub target_task_id: String,
    pub relation_type: String,
    pub created_at: String,
}

// --- External Link ---

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalLink {
    pub id: String,
    pub task_id: String,
    pub integration_id: String,
    pub resource_type: String,
    pub external_id: String,
    pub url: String,
    pub title: Option<String>,
    pub metadata: Option<serde_json::Value>,
    pub created_at: String,
    pub updated_at: String,
}

// --- Invitation ---

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Invitation {
    pub id: String,
    pub email: String,
    pub workspace_id: String,
    pub workspace_name: String,
    pub inviter_name: String,
    pub expires_at: String,
    pub created_at: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvitationDetails {
    pub valid: bool,
    pub invitation: Option<InvitationInfo>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvitationInfo {
    pub id: String,
    pub email: String,
    pub workspace_name: String,
    pub inviter_name: String,
    pub expires_at: String,
    pub status: String,
    pub expired: bool,
}

// --- Workspace Member (dedicated endpoint) ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceMemberInfo {
    pub id: String,
    pub name: String,
    pub email: String,
    #[serde(default)]
    pub image: Option<String>,
    pub role: String,
}

// --- Bulk Update ---

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BulkUpdateResult {
    pub success: bool,
    pub updated_count: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_deserialize_camel_case() {
        let json = r#"{
            "id": "abc",
            "projectId": "proj-1",
            "title": "Fix bug",
            "status": "todo",
            "priority": "high",
            "createdAt": "2026-01-01",
            "position": 1.0,
            "number": 42
        }"#;
        let task: Task = serde_json::from_str(json).unwrap();
        assert_eq!(task.id, "abc");
        assert_eq!(task.project_id, "proj-1");
        assert_eq!(task.number, Some(42));
        assert!(task.description.is_none());
        assert!(task.user_id.is_none());
        assert!(task.due_date.is_none());
    }

    #[test]
    fn task_roundtrip() {
        let task = Task {
            id: "t1".into(),
            project_id: "p1".into(),
            position: Some(0.0),
            number: Some(1),
            user_id: Some("u1".into()),
            title: "Test".into(),
            description: Some("desc".into()),
            status: "done".into(),
            priority: "low".into(),
            due_date: Some("2026-12-31".into()),
            created_at: "2026-01-01".into(),
        };
        let json = serde_json::to_string(&task).unwrap();
        let restored: Task = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.id, task.id);
        assert_eq!(restored.title, task.title);
        assert_eq!(restored.user_id, task.user_id);
    }

    #[test]
    fn create_task_body_skips_none() {
        let body = CreateTaskBody {
            title: "Test".into(),
            description: "".into(),
            priority: "low".into(),
            status: "backlog".into(),
            due_date: None,
            user_id: None,
        };
        let json = serde_json::to_string(&body).unwrap();
        assert!(!json.contains("dueDate"));
        assert!(!json.contains("userId"));
        assert!(json.contains("title"));
    }

    #[test]
    fn create_task_body_includes_present() {
        let body = CreateTaskBody {
            title: "Test".into(),
            description: "".into(),
            priority: "low".into(),
            status: "backlog".into(),
            due_date: Some("2026-06-01".into()),
            user_id: Some("user-1".into()),
        };
        let json = serde_json::to_string(&body).unwrap();
        assert!(json.contains("dueDate"));
        assert!(json.contains("userId"));
    }

    #[test]
    fn project_deserialize() {
        let json = r#"{
            "id": "p1",
            "workspaceId": "ws1",
            "slug": "my-project",
            "name": "My Project",
            "createdAt": "2026-01-01"
        }"#;
        let project: Project = serde_json::from_str(json).unwrap();
        assert_eq!(project.slug, "my-project");
        assert!(project.icon.is_none());
        assert!(project.description.is_none());
        assert!(project.is_public.is_none());
    }

    #[test]
    fn label_deserialize_with_task() {
        let json = r##"{
            "id": "l1",
            "name": "bug",
            "color": "#ff0000",
            "createdAt": "2026-01-01",
            "taskId": "t1",
            "workspaceId": "ws1"
        }"##;
        let label: Label = serde_json::from_str(json).unwrap();
        assert_eq!(label.task_id.as_deref(), Some("t1"));
    }

    #[test]
    fn label_deserialize_without_task() {
        let json = r##"{
            "id": "l1",
            "name": "feature",
            "color": "#00ff00",
            "createdAt": "2026-01-01",
            "taskId": null,
            "workspaceId": null
        }"##;
        let label: Label = serde_json::from_str(json).unwrap();
        assert!(label.task_id.is_none());
    }

    #[test]
    fn column_deserialize() {
        let json = r#"{
            "id": "c1",
            "projectId": "p1",
            "name": "Done",
            "position": 2
        }"#;
        let col: Column = serde_json::from_str(json).unwrap();
        assert_eq!(col.name, "Done");
        assert_eq!(col.position, 2);
        assert!(col.icon.is_none());
        assert!(col.is_final.is_none());
    }

    #[test]
    fn session_response_null_user() {
        let json = r#"{"user": null, "session": null}"#;
        let resp: SessionResponse = serde_json::from_str(json).unwrap();
        assert!(resp.user.is_none());
    }

    #[test]
    fn create_column_body_skips_none() {
        let body = CreateColumnBody {
            name: "Todo".into(),
            icon: None,
            color: None,
            is_final: None,
        };
        let json = serde_json::to_string(&body).unwrap();
        assert!(json.contains("name"));
        assert!(!json.contains("icon"));
        assert!(!json.contains("color"));
        assert!(!json.contains("isFinal"));
    }

    #[test]
    fn project_deserialize_with_archived() {
        let json = r#"{
            "id": "p1",
            "workspaceId": "ws1",
            "slug": "archived-proj",
            "name": "Archived",
            "createdAt": "2026-01-01",
            "archivedAt": "2026-03-20T12:00:00Z"
        }"#;
        let project: Project = serde_json::from_str(json).unwrap();
        assert_eq!(project.archived_at.as_deref(), Some("2026-03-20T12:00:00Z"));
    }

    #[test]
    fn project_deserialize_without_archived() {
        let json = r#"{
            "id": "p1",
            "workspaceId": "ws1",
            "slug": "active-proj",
            "name": "Active",
            "createdAt": "2026-01-01"
        }"#;
        let project: Project = serde_json::from_str(json).unwrap();
        assert!(project.archived_at.is_none());
    }

    #[test]
    fn activity_deserialize_with_external() {
        let json = r#"{
            "id": "a1",
            "taskId": "t1",
            "type": "comment",
            "createdAt": "2026-01-01",
            "externalUserName": "github-bot",
            "externalSource": "github"
        }"#;
        let activity: Activity = serde_json::from_str(json).unwrap();
        assert_eq!(activity.external_user_name.as_deref(), Some("github-bot"));
        assert_eq!(activity.external_source.as_deref(), Some("github"));
        assert!(activity.external_url.is_none());
    }

    #[test]
    fn external_link_deserialize() {
        let json = r#"{
            "id": "el1",
            "taskId": "t1",
            "integrationId": "gh1",
            "resourceType": "issue",
            "externalId": "123",
            "url": "https://github.com/org/repo/issues/123",
            "title": "Fix bug",
            "metadata": {"state": "open"},
            "createdAt": "2026-01-01",
            "updatedAt": "2026-01-02"
        }"#;
        let link: ExternalLink = serde_json::from_str(json).unwrap();
        assert_eq!(link.resource_type, "issue");
        assert_eq!(link.title.as_deref(), Some("Fix bug"));
        assert!(link.metadata.is_some());
    }

    #[test]
    fn invitation_deserialize() {
        let json = r#"{
            "id": "inv1",
            "email": "test@example.com",
            "workspaceId": "ws1",
            "workspaceName": "My Workspace",
            "inviterName": "Admin",
            "expiresAt": "2026-04-01",
            "createdAt": "2026-03-01",
            "status": "pending"
        }"#;
        let inv: Invitation = serde_json::from_str(json).unwrap();
        assert_eq!(inv.email, "test@example.com");
        assert_eq!(inv.status, "pending");
    }

    #[test]
    fn invitation_details_valid() {
        let json = r#"{
            "valid": true,
            "invitation": {
                "id": "inv1",
                "email": "test@example.com",
                "workspaceName": "WS",
                "inviterName": "Admin",
                "expiresAt": "2026-04-01",
                "status": "pending",
                "expired": false
            }
        }"#;
        let details: InvitationDetails = serde_json::from_str(json).unwrap();
        assert!(details.valid);
        assert!(details.invitation.is_some());
        assert!(!details.invitation.unwrap().expired);
    }

    #[test]
    fn invitation_details_invalid() {
        let json = r#"{
            "valid": false,
            "error": "Invitation not found"
        }"#;
        let details: InvitationDetails = serde_json::from_str(json).unwrap();
        assert!(!details.valid);
        assert_eq!(details.error.as_deref(), Some("Invitation not found"));
    }

    #[test]
    fn workspace_member_info_deserialize() {
        let json = r#"{
            "id": "u1",
            "name": "Alice",
            "email": "alice@example.com",
            "image": null,
            "role": "admin"
        }"#;
        let member: WorkspaceMemberInfo = serde_json::from_str(json).unwrap();
        assert_eq!(member.name, "Alice");
        assert_eq!(member.role, "admin");
        assert!(member.image.is_none());
    }

    #[test]
    fn comment_deserialize() {
        let json = r#"{
            "id": "c1",
            "taskId": "t1",
            "userId": "u1",
            "content": "Hello",
            "createdAt": "2026-01-01",
            "updatedAt": "2026-01-02",
            "user": {"name": "Alice", "image": null}
        }"#;
        let comment: Comment = serde_json::from_str(json).unwrap();
        assert_eq!(comment.content, "Hello");
        assert!(comment.user.is_some());
        assert_eq!(comment.user.unwrap().name, "Alice");
    }

    #[test]
    fn task_relation_deserialize() {
        let json = r#"{
            "id": "r1",
            "sourceTaskId": "t1",
            "targetTaskId": "t2",
            "relationType": "subtask",
            "createdAt": "2026-01-01"
        }"#;
        let rel: TaskRelation = serde_json::from_str(json).unwrap();
        assert_eq!(rel.relation_type, "subtask");
        assert_eq!(rel.source_task_id, "t1");
    }

    #[test]
    fn server_config_new_fields() {
        let json = r#"{
            "disableRegistration": false,
            "disablePasswordRegistration": true,
            "isDemoMode": false,
            "hasSmtp": true,
            "hasGithubSignIn": false,
            "hasGoogleSignIn": false,
            "hasDiscordSignIn": false,
            "hasCustomOAuth": true,
            "hasGuestAccess": false
        }"#;
        let config: ServerConfig = serde_json::from_str(json).unwrap();
        assert_eq!(config.disable_password_registration, Some(true));
        assert_eq!(config.has_custom_oauth, Some(true));
        assert_eq!(config.has_guest_access, Some(false));
    }
}
