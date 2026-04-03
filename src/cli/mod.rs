pub mod api_check_handler;
pub mod column_handler;
pub mod invitation_handler;
pub mod label_handler;
pub mod link_handler;
pub mod login_handler;
pub mod notification_handler;
pub mod profile_handler;
pub mod project_handler;
pub mod resolve;
pub mod search_handler;
pub mod task_handler;
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
    #[command(alias = "lbl")]
    Label(LabelArgs),

    /// Manage notifications
    #[command(alias = "notif")]
    Notification(NotificationArgs),

    /// View invitations
    #[command(alias = "inv")]
    Invitation(InvitationArgs),

    /// Search across tasks, projects, comments
    #[command(alias = "s")]
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
        /// Workspace ID (interactive if omitted)
        id: Option<String>,
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
    List {
        /// Include archived projects
        #[arg(long)]
        include_archived: bool,
    },
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
    /// Archive a project
    Archive {
        /// Project ID
        id: String,
    },
    /// Unarchive a project
    Unarchive {
        /// Project ID
        id: String,
    },
}

// --- Task ---

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum TaskSortBy {
    CreatedAt,
    Priority,
    DueDate,
    Position,
    Title,
    Number,
}

impl TaskSortBy {
    pub fn as_api_str(&self) -> &'static str {
        match self {
            Self::CreatedAt => "createdAt",
            Self::Priority => "priority",
            Self::DueDate => "dueDate",
            Self::Position => "position",
            Self::Title => "title",
            Self::Number => "number",
        }
    }
}

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum SortOrder {
    Asc,
    Desc,
}

impl SortOrder {
    pub fn as_api_str(&self) -> &'static str {
        match self {
            Self::Asc => "asc",
            Self::Desc => "desc",
        }
    }
}

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum BulkOperation {
    UpdateStatus,
    UpdatePriority,
    UpdateAssignee,
    Delete,
    AddLabel,
    RemoveLabel,
    UpdateDueDate,
}

impl BulkOperation {
    pub fn as_api_str(&self) -> &'static str {
        match self {
            Self::UpdateStatus => "updateStatus",
            Self::UpdatePriority => "updatePriority",
            Self::UpdateAssignee => "updateAssignee",
            Self::Delete => "delete",
            Self::AddLabel => "addLabel",
            Self::RemoveLabel => "removeLabel",
            Self::UpdateDueDate => "updateDueDate",
        }
    }
}

#[derive(Parser)]
pub struct TaskArgs {
    #[command(subcommand)]
    pub command: TaskCommand,
}

#[derive(Subcommand)]
pub enum TaskCommand {
    /// List tasks (flat list in JSON, board view in terminal)
    #[command(alias = "ls")]
    List {
        /// Project ID (falls back to -p / .kaneo.json)
        project_id: Option<String>,
        /// Filter by status (column name, "planned", or "archived")
        #[arg(long)]
        status: Option<String>,
        /// Filter by priority
        #[arg(long)]
        priority: Option<String>,
        /// Filter by assignee user ID
        #[arg(long)]
        assignee: Option<String>,
        /// Page number (requires --limit)
        #[arg(long)]
        page: Option<u32>,
        /// Max tasks per page (1-100)
        #[arg(long)]
        limit: Option<u32>,
        /// Sort by field
        #[arg(long)]
        sort_by: Option<TaskSortBy>,
        /// Sort order
        #[arg(long)]
        sort_order: Option<SortOrder>,
        /// Filter tasks due before this date (ISO format)
        #[arg(long)]
        due_before: Option<String>,
        /// Filter tasks due after this date (ISO format)
        #[arg(long)]
        due_after: Option<String>,
        /// Include archived tasks
        #[arg(long)]
        all: bool,
        /// Force board view (columns) instead of flat list in JSON mode
        #[arg(long)]
        board: bool,
    },
    /// Get task details (compact summary by default, --full for all sub-resources)
    Get {
        /// Task ID
        id: String,
        /// Show full sub-resource details instead of compact summary
        #[arg(long)]
        full: bool,
    },
    /// Create a new task
    Create {
        /// Task title
        title: String,
        /// Description
        #[arg(long, default_value = "")]
        description: String,
        /// Priority: no-priority, low, medium, high, urgent
        #[arg(long, default_value = "no-priority")]
        priority: String,
        /// Column name to place the task in (e.g. "backlog", "todo", "in-progress").
        /// Note: "planned" puts the task in the planned backlog, not a board column.
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
        /// New status (interactive if omitted)
        status: Option<String>,
    },
    /// Update task priority
    Priority {
        /// Task ID
        id: String,
        /// New priority: no-priority, low, medium, high, urgent (interactive if omitted)
        priority: Option<String>,
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
    /// Manage task comments
    #[command(alias = "cmt")]
    Comment(TaskCommentArgs),

    /// Manage task labels
    #[command(alias = "lbl")]
    Label(TaskLabelArgs),

    /// Manage task relations
    Rel(TaskRelArgs),

    /// Manage time entries
    Time(TaskTimeArgs),

    /// List external links on a task
    Links {
        /// Task ID
        task_id: String,
    },

    /// Transfer a task to a different project
    Transfer {
        /// Task ID
        task_id: String,
        /// Target project ID
        #[arg(long)]
        project: String,
    },

    /// Bulk update multiple tasks
    Bulk {
        /// Comma-separated task IDs
        task_ids: String,
        /// Operation to perform
        #[arg(long)]
        operation: BulkOperation,
        /// Value for the operation (status name, priority, user ID, label ID, date)
        #[arg(long)]
        value: Option<String>,
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
    /// Get label details
    Get {
        /// Label ID
        id: String,
    },
    /// Create a label
    Create {
        /// Label name
        name: String,
        /// Color (hex, e.g. #ff0000)
        #[arg(long)]
        color: String,
    },
    /// Update a label
    Update {
        /// Label ID
        id: String,
        /// New name
        #[arg(long)]
        name: Option<String>,
        /// New color (hex, e.g. #ff0000)
        #[arg(long)]
        color: Option<String>,
    },
    /// Delete a label
    #[command(alias = "rm")]
    Delete {
        /// Label ID
        id: String,
    },
}

// --- Task Comment ---

#[derive(Parser)]
pub struct TaskCommentArgs {
    #[command(subcommand)]
    pub command: TaskCommentCommand,
}

#[derive(Subcommand)]
pub enum TaskCommentCommand {
    /// List comments on a task
    #[command(alias = "ls")]
    List {
        /// Task ID
        task_id: String,
    },
    /// Add a comment to a task
    Add {
        /// Task ID
        task_id: String,
        /// Comment text
        text: String,
    },
    /// Edit a comment
    Edit {
        /// Activity/comment ID
        id: String,
        /// New comment text
        text: String,
    },
    /// Delete a comment
    #[command(alias = "rm")]
    Delete {
        /// Activity/comment ID
        id: String,
    },
}

// --- Task Label ---

#[derive(Parser)]
pub struct TaskLabelArgs {
    #[command(subcommand)]
    pub command: TaskLabelCommand,
}

#[derive(Subcommand)]
pub enum TaskLabelCommand {
    /// List labels on a task
    #[command(alias = "ls")]
    List {
        /// Task ID
        task_id: String,
    },
    /// Attach a label to a task
    Add {
        /// Task ID
        task_id: String,
        /// Label ID to attach
        label_id: String,
    },
    /// Detach a label from a task
    #[command(alias = "rm")]
    Delete {
        /// Label ID to detach
        label_id: String,
    },
}

// --- Task Relation ---

#[derive(Parser)]
pub struct TaskRelArgs {
    #[command(subcommand)]
    pub command: TaskRelCommand,
}

#[derive(Subcommand)]
pub enum TaskRelCommand {
    /// List relations for a task
    #[command(alias = "ls")]
    List {
        /// Task ID
        task_id: String,
    },
    /// Create a relation between tasks
    Add {
        /// Source task ID
        source: String,
        /// Target task ID
        target: String,
        /// Relation type
        #[arg(long, value_name = "TYPE")]
        r#type: RelationType,
    },
    /// Delete a relation
    #[command(alias = "rm")]
    Delete {
        /// Relation ID
        id: String,
    },
}

// --- Task Time ---

#[derive(Parser)]
pub struct TaskTimeArgs {
    #[command(subcommand)]
    pub command: TaskTimeCommand,
}

#[derive(Subcommand)]
pub enum TaskTimeCommand {
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
    Add {
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
    Edit {
        /// Time entry ID
        id: String,
        /// Start time (ISO format)
        #[arg(long)]
        start: Option<String>,
        /// End time (ISO format)
        #[arg(long)]
        end: Option<String>,
        /// Description
        #[arg(long)]
        description: Option<String>,
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
    /// Create a notification
    Create {
        /// Target user ID
        user_id: String,
        /// Notification title
        title: String,
        /// Notification message
        message: String,
        /// Type: info, task_created, workspace_created, etc.
        #[arg(long, default_value = "info")]
        notification_type: String,
        /// Related entity ID
        #[arg(long)]
        related_entity_id: Option<String>,
        /// Related entity type (task, workspace)
        #[arg(long)]
        related_entity_type: Option<String>,
    },
}

// --- Relation Type ---

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum RelationType {
    Subtask,
    Blocks,
    Related,
}

impl RelationType {
    pub fn as_api_str(&self) -> &'static str {
        match self {
            Self::Subtask => "subtask",
            Self::Blocks => "blocks",
            Self::Related => "related",
        }
    }
}

// --- Invitation ---

#[derive(Parser)]
pub struct InvitationArgs {
    #[command(subcommand)]
    pub command: InvitationCommand,
}

#[derive(Subcommand)]
pub enum InvitationCommand {
    /// List your pending invitations
    Pending,
    /// Get invitation details
    Get {
        /// Invitation ID
        id: String,
    },
}

// --- Search ---

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum SearchType {
    All,
    Tasks,
    Projects,
    Workspaces,
    Comments,
    Activities,
}

impl SearchType {
    pub fn as_api_str(&self) -> &'static str {
        match self {
            Self::All => "all",
            Self::Tasks => "tasks",
            Self::Projects => "projects",
            Self::Workspaces => "workspaces",
            Self::Comments => "comments",
            Self::Activities => "activities",
        }
    }
}

#[derive(Parser)]
pub struct SearchArgs {
    /// Search query
    pub query: String,

    /// Filter type
    #[arg(long, default_value = "all")]
    pub r#type: SearchType,

    /// Limit results (1-50)
    #[arg(long, default_value_t = 20)]
    pub limit: u32,

    /// Filter by project ID
    #[arg(long)]
    pub project_id: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    fn parse(args: &[&str]) -> Cli {
        Cli::try_parse_from(args).unwrap()
    }

    // --- Command aliases ---

    #[test]
    fn task_alias_t() {
        let cli = parse(&["kaneo", "t", "ls"]);
        assert!(matches!(cli.command, Command::Task(_)));
    }

    #[test]
    fn project_alias_proj() {
        let cli = parse(&["kaneo", "proj", "ls"]);
        assert!(matches!(cli.command, Command::Project(_)));
    }

    #[test]
    fn workspace_alias_ws() {
        let cli = parse(&["kaneo", "ws", "ls"]);
        assert!(matches!(cli.command, Command::Workspace(_)));
    }

    #[test]
    fn column_alias_col() {
        let cli = parse(&["kaneo", "col", "ls"]);
        assert!(matches!(cli.command, Command::Column(_)));
    }

    #[test]
    fn notification_alias_notif() {
        let cli = parse(&["kaneo", "notif", "ls"]);
        assert!(matches!(cli.command, Command::Notification(_)));
    }

    // --- Global flags ---

    #[test]
    fn global_json_flag() {
        let cli = parse(&["kaneo", "--json", "context"]);
        assert!(cli.json);
    }

    #[test]
    fn global_human_flag() {
        let cli = parse(&["kaneo", "--human", "context"]);
        assert!(cli.human);
    }

    #[test]
    fn global_workspace_short() {
        let cli = parse(&["kaneo", "-w", "ws-123", "context"]);
        assert_eq!(cli.workspace.as_deref(), Some("ws-123"));
    }

    #[test]
    fn global_project_short() {
        let cli = parse(&["kaneo", "-p", "proj-456", "context"]);
        assert_eq!(cli.project.as_deref(), Some("proj-456"));
    }

    #[test]
    fn global_token() {
        let cli = parse(&["kaneo", "--token", "my-key", "context"]);
        assert_eq!(cli.token.as_deref(), Some("my-key"));
    }

    // --- Task commands ---

    #[test]
    fn task_create_with_flags() {
        let cli = parse(&[
            "kaneo",
            "t",
            "create",
            "Fix bug",
            "--priority",
            "high",
            "--status",
            "todo",
            "--description",
            "Something broken",
        ]);
        if let Command::Task(args) = cli.command {
            if let TaskCommand::Create {
                title,
                priority,
                status,
                description,
                ..
            } = args.command
            {
                assert_eq!(title, "Fix bug");
                assert_eq!(priority, "high");
                assert_eq!(status, "todo");
                assert_eq!(description, "Something broken");
            } else {
                panic!("expected Create");
            }
        } else {
            panic!("expected Task");
        }
    }

    #[test]
    fn task_create_defaults() {
        let cli = parse(&["kaneo", "t", "create", "New task"]);
        if let Command::Task(args) = cli.command {
            if let TaskCommand::Create {
                priority,
                status,
                description,
                ..
            } = args.command
            {
                assert_eq!(priority, "no-priority");
                assert_eq!(status, "backlog");
                assert_eq!(description, "");
            } else {
                panic!("expected Create");
            }
        } else {
            panic!("expected Task");
        }
    }

    #[test]
    fn task_list_with_filters() {
        let cli = parse(&[
            "kaneo",
            "t",
            "ls",
            "--status",
            "todo",
            "--priority",
            "high",
            "--assignee",
            "user-1",
            "--limit",
            "10",
            "--page",
            "2",
        ]);
        if let Command::Task(args) = cli.command {
            if let TaskCommand::List {
                status,
                priority,
                assignee,
                limit,
                page,
                ..
            } = args.command
            {
                assert_eq!(status.as_deref(), Some("todo"));
                assert_eq!(priority.as_deref(), Some("high"));
                assert_eq!(assignee.as_deref(), Some("user-1"));
                assert_eq!(limit, Some(10));
                assert_eq!(page, Some(2));
            } else {
                panic!("expected List");
            }
        } else {
            panic!("expected Task");
        }
    }

    #[test]
    fn task_list_with_project_id() {
        let cli = parse(&["kaneo", "t", "ls", "proj-abc"]);
        if let Command::Task(args) = cli.command {
            if let TaskCommand::List { project_id, .. } = args.command {
                assert_eq!(project_id.as_deref(), Some("proj-abc"));
            } else {
                panic!("expected List");
            }
        } else {
            panic!("expected Task");
        }
    }

    #[test]
    fn task_status_interactive() {
        let cli = parse(&["kaneo", "t", "status", "task-id"]);
        if let Command::Task(args) = cli.command {
            if let TaskCommand::Status { id, status } = args.command {
                assert_eq!(id, "task-id");
                assert!(status.is_none());
            } else {
                panic!("expected Status");
            }
        } else {
            panic!("expected Task");
        }
    }

    #[test]
    fn task_status_explicit() {
        let cli = parse(&["kaneo", "t", "status", "task-id", "done"]);
        if let Command::Task(args) = cli.command {
            if let TaskCommand::Status { status, .. } = args.command {
                assert_eq!(status.as_deref(), Some("done"));
            } else {
                panic!("expected Status");
            }
        } else {
            panic!("expected Task");
        }
    }

    #[test]
    fn task_delete_alias_rm() {
        let cli = parse(&["kaneo", "t", "rm", "task-id"]);
        if let Command::Task(args) = cli.command {
            assert!(matches!(args.command, TaskCommand::Delete { .. }));
        } else {
            panic!("expected Task");
        }
    }

    // --- Task comment subcommands ---

    #[test]
    fn task_comment_add() {
        let cli = parse(&["kaneo", "t", "comment", "add", "DEP-1", "hello"]);
        if let Command::Task(args) = cli.command {
            if let TaskCommand::Comment(cargs) = args.command {
                if let TaskCommentCommand::Add { task_id, text } = cargs.command {
                    assert_eq!(task_id, "DEP-1");
                    assert_eq!(text, "hello");
                } else {
                    panic!("expected Add");
                }
            } else {
                panic!("expected Comment");
            }
        } else {
            panic!("expected Task");
        }
    }

    #[test]
    fn task_comment_alias_cmt() {
        let cli = parse(&["kaneo", "t", "cmt", "ls", "DEP-1"]);
        if let Command::Task(args) = cli.command {
            assert!(matches!(args.command, TaskCommand::Comment(_)));
        } else {
            panic!("expected Task");
        }
    }

    // --- Task label subcommands ---

    #[test]
    fn task_label_add() {
        let cli = parse(&["kaneo", "t", "label", "add", "DEP-1", "label-1"]);
        if let Command::Task(args) = cli.command {
            if let TaskCommand::Label(largs) = args.command {
                if let TaskLabelCommand::Add { task_id, label_id } = largs.command {
                    assert_eq!(task_id, "DEP-1");
                    assert_eq!(label_id, "label-1");
                } else {
                    panic!("expected Add");
                }
            } else {
                panic!("expected Label");
            }
        } else {
            panic!("expected Task");
        }
    }

    // --- Task transfer ---

    #[test]
    fn task_transfer() {
        let cli = parse(&["kaneo", "t", "transfer", "DEP-1", "--project", "proj-2"]);
        if let Command::Task(args) = cli.command {
            if let TaskCommand::Transfer { task_id, project } = args.command {
                assert_eq!(task_id, "DEP-1");
                assert_eq!(project, "proj-2");
            } else {
                panic!("expected Transfer");
            }
        } else {
            panic!("expected Task");
        }
    }

    // --- Task rel subcommands ---

    #[test]
    fn task_rel_add() {
        let cli = parse(&[
            "kaneo", "t", "rel", "add", "src-1", "tgt-1", "--type", "subtask",
        ]);
        if let Command::Task(args) = cli.command {
            if let TaskCommand::Rel(rargs) = args.command {
                if let TaskRelCommand::Add {
                    source,
                    target,
                    r#type,
                } = rargs.command
                {
                    assert_eq!(source, "src-1");
                    assert_eq!(target, "tgt-1");
                    assert!(matches!(r#type, RelationType::Subtask));
                } else {
                    panic!("expected Add");
                }
            } else {
                panic!("expected Rel");
            }
        } else {
            panic!("expected Task");
        }
    }

    // --- Task time subcommands ---

    #[test]
    fn task_time_add() {
        let cli = parse(&[
            "kaneo",
            "t",
            "time",
            "add",
            "DEP-1",
            "2026-01-01T09:00:00Z",
            "--end",
            "2026-01-01T11:00:00Z",
        ]);
        if let Command::Task(args) = cli.command {
            if let TaskCommand::Time(targs) = args.command {
                if let TaskTimeCommand::Add {
                    task_id,
                    start,
                    end,
                    ..
                } = targs.command
                {
                    assert_eq!(task_id, "DEP-1");
                    assert_eq!(start, "2026-01-01T09:00:00Z");
                    assert_eq!(end.as_deref(), Some("2026-01-01T11:00:00Z"));
                } else {
                    panic!("expected Add");
                }
            } else {
                panic!("expected Time");
            }
        } else {
            panic!("expected Task");
        }
    }

    // --- Link ---

    #[test]
    fn link_with_flags() {
        let cli = parse(&["kaneo", "link", "-w", "ws-1", "-p", "proj-1"]);
        if let Command::Link(args) = cli.command {
            assert_eq!(args.workspace.as_deref(), Some("ws-1"));
            assert_eq!(args.project.as_deref(), Some("proj-1"));
        } else {
            panic!("expected Link");
        }
    }

    #[test]
    fn link_no_flags() {
        let cli = parse(&["kaneo", "link"]);
        if let Command::Link(args) = cli.command {
            assert!(args.workspace.is_none());
            assert!(args.project.is_none());
        } else {
            panic!("expected Link");
        }
    }

    // --- Search ---

    #[test]
    fn search_defaults() {
        let cli = parse(&["kaneo", "search", "login bug"]);
        if let Command::Search(args) = cli.command {
            assert_eq!(args.query, "login bug");
            assert!(matches!(args.r#type, SearchType::All));
            assert_eq!(args.limit, 20);
        } else {
            panic!("expected Search");
        }
    }

    // --- New command tests ---

    #[test]
    fn project_list_include_archived() {
        let cli = parse(&["kaneo", "proj", "ls", "--include-archived"]);
        if let Command::Project(args) = cli.command {
            if let ProjectCommand::List { include_archived } = args.command {
                assert!(include_archived);
            } else {
                panic!("expected List");
            }
        } else {
            panic!("expected Project");
        }
    }

    #[test]
    fn project_list_default_no_archived() {
        let cli = parse(&["kaneo", "proj", "ls"]);
        if let Command::Project(args) = cli.command {
            if let ProjectCommand::List { include_archived } = args.command {
                assert!(!include_archived);
            } else {
                panic!("expected List");
            }
        } else {
            panic!("expected Project");
        }
    }

    #[test]
    fn project_archive() {
        let cli = parse(&["kaneo", "proj", "archive", "proj-1"]);
        if let Command::Project(args) = cli.command {
            if let ProjectCommand::Archive { id } = args.command {
                assert_eq!(id, "proj-1");
            } else {
                panic!("expected Archive");
            }
        } else {
            panic!("expected Project");
        }
    }

    #[test]
    fn project_unarchive() {
        let cli = parse(&["kaneo", "proj", "unarchive", "proj-1"]);
        if let Command::Project(args) = cli.command {
            if let ProjectCommand::Unarchive { id } = args.command {
                assert_eq!(id, "proj-1");
            } else {
                panic!("expected Unarchive");
            }
        } else {
            panic!("expected Project");
        }
    }

    #[test]
    fn task_list_with_sort() {
        let cli = parse(&[
            "kaneo",
            "t",
            "ls",
            "--sort-by",
            "priority",
            "--sort-order",
            "desc",
        ]);
        if let Command::Task(args) = cli.command {
            if let TaskCommand::List {
                sort_by,
                sort_order,
                ..
            } = args.command
            {
                assert!(matches!(sort_by, Some(TaskSortBy::Priority)));
                assert!(matches!(sort_order, Some(SortOrder::Desc)));
            } else {
                panic!("expected List");
            }
        } else {
            panic!("expected Task");
        }
    }

    #[test]
    fn task_list_with_due_filters() {
        let cli = parse(&[
            "kaneo",
            "t",
            "ls",
            "--due-before",
            "2026-04-01",
            "--due-after",
            "2026-03-01",
        ]);
        if let Command::Task(args) = cli.command {
            if let TaskCommand::List {
                due_before,
                due_after,
                ..
            } = args.command
            {
                assert_eq!(due_before.as_deref(), Some("2026-04-01"));
                assert_eq!(due_after.as_deref(), Some("2026-03-01"));
            } else {
                panic!("expected List");
            }
        } else {
            panic!("expected Task");
        }
    }

    #[test]
    fn label_get() {
        let cli = parse(&["kaneo", "label", "get", "label-1"]);
        if let Command::Label(args) = cli.command {
            if let LabelCommand::Get { id } = args.command {
                assert_eq!(id, "label-1");
            } else {
                panic!("expected Get");
            }
        } else {
            panic!("expected Label");
        }
    }

    #[test]
    fn invitation_alias() {
        let cli = parse(&["kaneo", "inv", "pending"]);
        assert!(matches!(cli.command, Command::Invitation(_)));
    }

    #[test]
    fn invitation_pending() {
        let cli = parse(&["kaneo", "invitation", "pending"]);
        if let Command::Invitation(args) = cli.command {
            assert!(matches!(args.command, InvitationCommand::Pending));
        } else {
            panic!("expected Invitation");
        }
    }

    #[test]
    fn invitation_get() {
        let cli = parse(&["kaneo", "invitation", "get", "inv-1"]);
        if let Command::Invitation(args) = cli.command {
            if let InvitationCommand::Get { id } = args.command {
                assert_eq!(id, "inv-1");
            } else {
                panic!("expected Get");
            }
        } else {
            panic!("expected Invitation");
        }
    }

    #[test]
    fn notification_create() {
        let cli = parse(&[
            "kaneo",
            "notif",
            "create",
            "user-1",
            "Title",
            "Message body",
            "--notification-type",
            "info",
        ]);
        if let Command::Notification(args) = cli.command {
            if let NotificationCommand::Create {
                user_id,
                title,
                message,
                notification_type,
                ..
            } = args.command
            {
                assert_eq!(user_id, "user-1");
                assert_eq!(title, "Title");
                assert_eq!(message, "Message body");
                assert_eq!(notification_type, "info");
            } else {
                panic!("expected Create");
            }
        } else {
            panic!("expected Notification");
        }
    }

    #[test]
    fn sort_by_api_str() {
        assert_eq!(TaskSortBy::CreatedAt.as_api_str(), "createdAt");
        assert_eq!(TaskSortBy::DueDate.as_api_str(), "dueDate");
        assert_eq!(TaskSortBy::Priority.as_api_str(), "priority");
        assert_eq!(TaskSortBy::Position.as_api_str(), "position");
        assert_eq!(TaskSortBy::Title.as_api_str(), "title");
        assert_eq!(TaskSortBy::Number.as_api_str(), "number");
    }

    #[test]
    fn sort_order_api_str() {
        assert_eq!(SortOrder::Asc.as_api_str(), "asc");
        assert_eq!(SortOrder::Desc.as_api_str(), "desc");
    }

    // --- Bulk operations ---

    #[test]
    fn task_bulk() {
        let cli = parse(&["kaneo", "t", "bulk", "id1,id2,id3", "--operation", "delete"]);
        if let Command::Task(args) = cli.command {
            if let TaskCommand::Bulk {
                task_ids,
                operation,
                value,
            } = args.command
            {
                assert_eq!(task_ids, "id1,id2,id3");
                assert!(matches!(operation, BulkOperation::Delete));
                assert!(value.is_none());
            } else {
                panic!("expected Bulk");
            }
        } else {
            panic!("expected Task");
        }
    }

    #[test]
    fn task_bulk_with_value() {
        let cli = parse(&[
            "kaneo",
            "t",
            "bulk",
            "id1,id2",
            "--operation",
            "update-status",
            "--value",
            "done",
        ]);
        if let Command::Task(args) = cli.command {
            if let TaskCommand::Bulk {
                operation, value, ..
            } = args.command
            {
                assert!(matches!(operation, BulkOperation::UpdateStatus));
                assert_eq!(value.as_deref(), Some("done"));
            } else {
                panic!("expected Bulk");
            }
        } else {
            panic!("expected Task");
        }
    }

    #[test]
    fn bulk_operation_api_str() {
        assert_eq!(BulkOperation::UpdateStatus.as_api_str(), "updateStatus");
        assert_eq!(BulkOperation::UpdatePriority.as_api_str(), "updatePriority");
        assert_eq!(BulkOperation::UpdateAssignee.as_api_str(), "updateAssignee");
        assert_eq!(BulkOperation::Delete.as_api_str(), "delete");
        assert_eq!(BulkOperation::AddLabel.as_api_str(), "addLabel");
        assert_eq!(BulkOperation::RemoveLabel.as_api_str(), "removeLabel");
        assert_eq!(BulkOperation::UpdateDueDate.as_api_str(), "updateDueDate");
    }

    #[test]
    fn relation_type_api_str() {
        assert_eq!(RelationType::Subtask.as_api_str(), "subtask");
        assert_eq!(RelationType::Blocks.as_api_str(), "blocks");
        assert_eq!(RelationType::Related.as_api_str(), "related");
    }
}
