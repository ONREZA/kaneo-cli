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
