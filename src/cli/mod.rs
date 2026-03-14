pub mod activity_handler;
pub mod api_check_handler;
pub mod column_handler;
pub mod label_handler;
pub mod link_handler;
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
        /// Filter by status (column name)
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
    },
    /// Get task details
    Get {
        /// Task ID
        id: String,
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
    /// Attach a label to a task
    Attach {
        /// Label ID
        id: String,
        /// Task ID to attach the label to
        #[arg(long)]
        task: String,
    },
    /// Detach a label from its current task
    Detach {
        /// Label ID
        id: String,
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

    // --- Label commands ---

    #[test]
    fn label_attach() {
        let cli = parse(&["kaneo", "label", "attach", "label-1", "--task", "task-1"]);
        if let Command::Label(args) = cli.command {
            if let LabelCommand::Attach { id, task } = args.command {
                assert_eq!(id, "label-1");
                assert_eq!(task, "task-1");
            } else {
                panic!("expected Attach");
            }
        } else {
            panic!("expected Label");
        }
    }

    #[test]
    fn label_detach() {
        let cli = parse(&["kaneo", "label", "detach", "label-1"]);
        if let Command::Label(args) = cli.command {
            assert!(matches!(args.command, LabelCommand::Detach { .. }));
        } else {
            panic!("expected Label");
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
            assert_eq!(args.r#type, "all");
            assert_eq!(args.limit, "20");
        } else {
            panic!("expected Search");
        }
    }
}
