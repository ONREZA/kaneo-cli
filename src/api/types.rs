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
}
