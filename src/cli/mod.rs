pub mod activity_handler;
pub mod api_check_handler;
pub mod column_handler;
pub mod label_handler;
pub mod login_handler;
pub mod notification_handler;
pub mod profile_handler;
pub mod project_handler;
pub mod search_handler;
pub mod task_handler;
pub mod time_entry_handler;
pub mod whoami_handler;
pub mod workspace_handler;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "kaneo", version, about = "CLI for Kaneo project management")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,

    /// Output as JSON (auto-enabled when stdout is not a TTY)
    #[arg(long, global = true, env = "KANEO_JSON")]
    pub json: bool,

    /// Force human-readable output
    #[arg(long, global = true)]
    pub human: bool,

    /// API key (overrides config)
    #[arg(long, global = true, env = "KANEO_API_KEY")]
    pub token: Option<String>,

    /// API base URL
    #[arg(long, global = true, env = "KANEO_API_URL")]
    pub api_url: Option<String>,

    /// Workspace ID
    #[arg(long, short = 'w', global = true, env = "KANEO_WORKSPACE")]
    pub workspace: Option<String>,

    /// Project ID
    #[arg(long, short = 'p', global = true, env = "KANEO_PROJECT")]
    pub project: Option<String>,
}

#[derive(Subcommand)]
pub enum Command {
    /// Authenticate with a Kaneo instance
    Login(LoginArgs),
    /// Show current user info
    Whoami,
    /// Remove stored credentials
    Logout,
    /// Manage connection profiles
    Profile(ProfileArgs),

    /// Link current directory to a workspace/project (.kaneo.json)
    Link(LinkArgs),
    /// Remove .kaneo.json from current directory
    Unlink,
    /// Show resolved context (workspace, project, profile)
    Context,

    /// Manage workspaces
    #[command(alias = "ws")]
    Workspace(WorkspaceArgs),

    /// Manage projects
    #[command(alias = "proj")]
    Project(ProjectArgs),

    /// Manage tasks
    #[command(alias = "t")]
    Task(TaskArgs),

    /// Manage board columns
    #[command(alias = "col")]
    Column(ColumnArgs),

    /// Manage labels
    Label(LabelArgs),

    /// View task activity and comments
    Activity(ActivityArgs),

    /// Manage notifications
    #[command(alias = "notif")]
    Notification(NotificationArgs),

    /// Track time on tasks
    #[command(alias = "time")]
    TimeEntry(TimeEntryArgs),

    /// Search across tasks, projects, comments
    Search(SearchArgs),

    /// Check CLI compatibility with the server's API
    ApiCheck,

    /// Upgrade kaneo to the latest version
    Upgrade(crate::upgrade::UpgradeArgs),
}

// --- Link ---

#[derive(Parser)]
pub struct LinkArgs {
    /// Workspace ID to link
    #[arg(long, short = 'w')]
    pub workspace: Option<String>,
    /// Project ID to link
    #[arg(long, short = 'p')]
    pub project: Option<String>,
}

// --- Login ---

#[derive(Parser)]
pub struct LoginArgs {
    /// Kaneo API URL (e.g. https://kaneo.example.com or https://cloud.kaneo.app)
    #[arg(long)]
    pub url: Option<String>,

    /// API key
    #[arg(long)]
    pub key: Option<String>,

    /// Profile name (use different profiles for different instances)
    #[arg(long, default_value = "default")]
    pub profile: String,

    /// Set default workspace ID for this profile
    #[arg(long)]
    pub workspace: Option<String>,
}

// --- Profile ---

#[derive(Parser)]
pub struct ProfileArgs {
    #[command(subcommand)]
    pub command: ProfileCommand,
}

#[derive(Subcommand)]
pub enum ProfileCommand {
    /// List all profiles
    #[command(alias = "ls")]
    List,
    /// Switch default profile
    Use {
        /// Profile name
        name: String,
    },
    /// Remove a profile
    #[command(alias = "rm")]
    Remove {
        /// Profile name
        name: String,
    },
    /// Show current profile details
    Current,
    /// Set workspace ID on a profile
    SetWorkspace {
        /// Workspace ID
        workspace_id: String,
        /// Profile name (default = current)
        #[arg(long)]
        profile: Option<String>,
    },
}

// --- Workspace ---

#[derive(Parser)]
pub struct WorkspaceArgs {
    #[command(subcommand)]
    pub command: WorkspaceCommand,
}

#[derive(Subcommand)]
pub enum WorkspaceCommand {
    /// List workspaces you belong to
    #[command(alias = "ls")]
    List,
    /// Get workspace details
    Get {
        /// Workspace ID (defaults to active)
        id: Option<String>,
    },
    /// Create a new workspace
    Create {
        /// Workspace name
        name: String,
        /// URL-safe slug
        #[arg(long)]
        slug: Option<String>,
        /// Logo URL
        #[arg(long)]
        logo: Option<String>,
    },
    /// Update a workspace
    Update {
        /// Workspace ID (defaults to active)
        #[arg(long)]
        id: Option<String>,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        slug: Option<String>,
        #[arg(long)]
        logo: Option<String>,
    },
    /// Delete a workspace
    #[command(alias = "rm")]
    Delete {
        /// Workspace ID
        id: String,
    },
    /// List workspace members
    Members,
    /// Invite a member by email
    Invite {
        /// Email to invite
        email: String,
        /// Role: member or admin
        #[arg(long, default_value = "member")]
        role: String,
    },
    /// Remove a member
    RemoveMember {
        /// User ID to remove
        user_id: String,
    },
    /// Update a member's role
    UpdateRole {
        /// User ID
        user_id: String,
        /// New role: member, admin, or owner
        role: String,
    },
    /// Leave the workspace
    Leave {
        /// Workspace ID (defaults to active)
        id: Option<String>,
    },
    /// Set active workspace
    SetActive {
        /// Workspace ID
        id: String,
    },
    /// Check if a slug is available
    CheckSlug {
        /// Slug to check
        slug: String,
    },
    /// List pending invitations for the workspace
    Invitations,
    /// Cancel a pending invitation
    CancelInvitation {
        /// Invitation ID
        id: String,
    },
    /// Accept an invitation
    AcceptInvitation {
        /// Invitation ID
        id: String,
    },
    /// Reject an invitation
    RejectInvitation {
        /// Invitation ID
        id: String,
    },
}

// --- Project ---

#[derive(Parser)]
pub struct ProjectArgs {
    #[command(subcommand)]
    pub command: ProjectCommand,
}

#[derive(Subcommand)]
pub enum ProjectCommand {
    /// List projects in workspace
    #[command(alias = "ls")]
    List,
    /// Get project details
    Get {
        /// Project ID
        id: String,
    },
    /// Create a new project
    Create {
        /// Project name
        name: String,
        /// Slug (URL-safe identifier)
        #[arg(long)]
        slug: Option<String>,
        /// Icon emoji
        #[arg(long, default_value = "📋")]
        icon: String,
    },
    /// Update a project
    Update {
        /// Project ID
        id: String,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        slug: Option<String>,
        #[arg(long)]
        icon: Option<String>,
        #[arg(long)]
        description: Option<String>,
        #[arg(long)]
        public: Option<bool>,
    },
    /// Delete a project
    #[command(alias = "rm")]
    Delete {
        /// Project ID
        id: String,
    },
}

// --- Task ---

#[derive(Parser)]
pub struct TaskArgs {
    #[command(subcommand)]
    pub command: TaskCommand,
}

#[derive(Subcommand)]
pub enum TaskCommand {
    /// List tasks in a project (board view)
    #[command(alias = "ls")]
    List {
        /// Project ID (falls back to -p / .kaneo.json)
        project_id: Option<String>,
    },
    /// Get task details
    Get {
        /// Task ID
        id: String,
    },
    /// Create a new task
    Create {
        /// Project ID (falls back to -p / .kaneo.json)
        project_id: Option<String>,
        /// Task title
        title: String,
        /// Description
        #[arg(long, default_value = "")]
        description: String,
        /// Priority: no-priority, low, medium, high, urgent
        #[arg(long, default_value = "no-priority")]
        priority: String,
        /// Status (column name, e.g. "todo", "in-progress")
        #[arg(long, default_value = "backlog")]
        status: String,
        /// Due date (ISO format)
        #[arg(long)]
        due_date: Option<String>,
        /// Assign to user ID
        #[arg(long)]
        assignee: Option<String>,
    },
    /// Update task status
    Status {
        /// Task ID
        id: String,
        /// New status
        status: String,
    },
    /// Update task priority
    Priority {
        /// Task ID
        id: String,
        /// New priority: no-priority, low, medium, high, urgent
        priority: String,
    },
    /// Update task assignee
    Assign {
        /// Task ID
        id: String,
        /// User ID to assign (empty to unassign)
        #[arg(default_value = "")]
        user_id: String,
    },
    /// Update task title
    Title {
        /// Task ID
        id: String,
        /// New title
        title: String,
    },
    /// Update task description
    Description {
        /// Task ID
        id: String,
        /// New description
        description: String,
    },
    /// Update task due date
    DueDate {
        /// Task ID
        id: String,
        /// Due date in ISO format (omit to clear)
        date: Option<String>,
    },
    /// Delete a task
    #[command(alias = "rm")]
    Delete {
        /// Task ID
        id: String,
    },
    /// Export tasks from a project
    Export {
        /// Project ID (falls back to -p / .kaneo.json)
        project_id: Option<String>,
    },
    /// Import tasks into a project from JSON
    Import {
        /// Project ID (falls back to -p / .kaneo.json)
        project_id: Option<String>,
        /// Path to JSON file with tasks array
        file: String,
    },
    /// Download a task attachment/asset
    Asset {
        /// Asset ID
        id: String,
        /// Output file path (default: original filename)
        #[arg(long, short = 'o')]
        output: Option<String>,
    },
    /// Upload an image to a task (description or comment)
    Upload {
        /// Task ID
        task_id: String,
        /// Path to image file
        file: String,
        /// Surface: description or comment
        #[arg(long, default_value = "description")]
        surface: String,
    },
}

// --- Column ---

#[derive(Parser)]
pub struct ColumnArgs {
    #[command(subcommand)]
    pub command: ColumnCommand,
}

#[derive(Subcommand)]
pub enum ColumnCommand {
    /// List columns in a project
    #[command(alias = "ls")]
    List {
        /// Project ID (falls back to -p / .kaneo.json)
        project_id: Option<String>,
    },
    /// Create a column
    Create {
        /// Project ID (falls back to -p / .kaneo.json)
        project_id: Option<String>,
        /// Column name
        name: String,
        #[arg(long)]
        icon: Option<String>,
        #[arg(long)]
        color: Option<String>,
        #[arg(long)]
        is_final: Option<bool>,
    },
    /// Update a column
    Update {
        /// Column ID
        id: String,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        icon: Option<String>,
        #[arg(long)]
        color: Option<String>,
        #[arg(long)]
        is_final: Option<bool>,
    },
    /// Reorder columns in a project
    Reorder {
        /// Project ID (falls back to -p / .kaneo.json)
        project_id: Option<String>,
        /// Column IDs in desired order (comma-separated)
        order: String,
    },
    /// Delete a column
    #[command(alias = "rm")]
    Delete {
        /// Column ID
        id: String,
    },
}

// --- Label ---

#[derive(Parser)]
pub struct LabelArgs {
    #[command(subcommand)]
    pub command: LabelCommand,
}

#[derive(Subcommand)]
pub enum LabelCommand {
    /// List labels in workspace
    #[command(alias = "ls")]
    List,
    /// List labels for a task
    Task {
        /// Task ID
        task_id: String,
    },
    /// Create a label
    Create {
        /// Label name
        name: String,
        /// Color (hex, e.g. #ff0000)
        #[arg(long)]
        color: String,
        /// Optionally attach to a task
        #[arg(long)]
        task_id: Option<String>,
    },
    /// Update a label
    Update {
        /// Label ID
        id: String,
        /// New name
        #[arg(long)]
        name: String,
        /// New color
        #[arg(long)]
        color: String,
    },
    /// Delete a label
    #[command(alias = "rm")]
    Delete {
        /// Label ID
        id: String,
    },
}

// --- Activity ---

#[derive(Parser)]
pub struct ActivityArgs {
    #[command(subcommand)]
    pub command: ActivityCommand,
}

#[derive(Subcommand)]
pub enum ActivityCommand {
    /// List activities for a task
    #[command(alias = "ls")]
    List {
        /// Task ID
        task_id: String,
    },
    /// Add a comment to a task
    Comment {
        /// Task ID
        task_id: String,
        /// Comment text
        comment: String,
    },
    /// Edit a comment
    EditComment {
        /// Activity/comment ID
        id: String,
        /// New comment text
        comment: String,
    },
    /// Delete a comment
    DeleteComment {
        /// Activity/comment ID
        id: String,
    },
}

// --- Notification ---

#[derive(Parser)]
pub struct NotificationArgs {
    #[command(subcommand)]
    pub command: NotificationCommand,
}

#[derive(Subcommand)]
pub enum NotificationCommand {
    /// List notifications
    #[command(alias = "ls")]
    List,
    /// Mark a notification as read
    Read {
        /// Notification ID
        id: String,
    },
    /// Mark all notifications as read
    ReadAll,
    /// Clear all notifications
    ClearAll,
}

// --- Time Entry ---

#[derive(Parser)]
pub struct TimeEntryArgs {
    #[command(subcommand)]
    pub command: TimeEntryCommand,
}

#[derive(Subcommand)]
pub enum TimeEntryCommand {
    /// List time entries for a task
    #[command(alias = "ls")]
    List {
        /// Task ID
        task_id: String,
    },
    /// Get a time entry
    Get {
        /// Time entry ID
        id: String,
    },
    /// Create a time entry
    Create {
        /// Task ID
        task_id: String,
        /// Start time (ISO format)
        start: String,
        /// End time (ISO format, optional for running entries)
        #[arg(long)]
        end: Option<String>,
        /// Description
        #[arg(long)]
        description: Option<String>,
    },
    /// Update a time entry
    Update {
        /// Time entry ID
        id: String,
        /// Start time (ISO format)
        start: String,
        /// End time (ISO format)
        #[arg(long)]
        end: Option<String>,
        /// Description
        #[arg(long)]
        description: Option<String>,
    },
}

// --- Search ---

#[derive(Parser)]
pub struct SearchArgs {
    /// Search query
    pub query: String,

    /// Filter type: all, tasks, projects, workspaces, comments, activities
    #[arg(long, default_value = "all")]
    pub r#type: String,

    /// Limit results (1-50)
    #[arg(long, default_value = "20")]
    pub limit: String,

    /// Filter by project ID
    #[arg(long)]
    pub project_id: Option<String>,
}
