# kaneo

CLI for [Kaneo](https://kaneo.app/) project management — a minimalist, open-source task tracker.

Works with both **Kaneo Cloud** (`cloud.kaneo.app`) and **self-hosted** instances.

## Install

**Linux / macOS:**

```bash
curl -fsSL https://raw.githubusercontent.com/onreza/kaneo-cli/main/install.sh | bash
```

**Windows (PowerShell):**

```powershell
irm https://raw.githubusercontent.com/onreza/kaneo-cli/main/install.ps1 | iex
```

**npm:**

```bash
npm install -g kaneo-cli
```

**Cargo:**

```bash
cargo install kaneo
```

## Quick Start

```bash
# Authenticate with your Kaneo instance
kaneo login --url https://cloud.kaneo.app --key YOUR_API_KEY

# List workspaces
kaneo workspace ls

# Set default workspace
kaneo profile set-workspace WORKSPACE_ID

# List projects
kaneo project ls

# Create a task
kaneo task create PROJECT_ID "Fix login bug" --priority high --status todo

# View board
kaneo task ls PROJECT_ID

# Search across everything
kaneo search "login bug"
```

## Commands

### Authentication

| Command | Description |
|---------|-------------|
| `kaneo login` | Authenticate with a Kaneo instance |
| `kaneo logout` | Remove stored credentials |
| `kaneo whoami` | Show current user info |
| `kaneo profile ls` | List connection profiles |
| `kaneo profile use <name>` | Switch active profile |
| `kaneo profile current` | Show current profile details |
| `kaneo profile set-workspace <id>` | Set workspace on profile |

### Workspaces

| Command | Description |
|---------|-------------|
| `kaneo ws ls` | List workspaces |
| `kaneo ws get [id]` | Get workspace details |
| `kaneo ws create <name>` | Create a workspace |
| `kaneo ws update` | Update workspace name/slug/logo |
| `kaneo ws delete <id>` | Delete a workspace |
| `kaneo ws members` | List workspace members |
| `kaneo ws invite <email>` | Invite a member |
| `kaneo ws remove-member <id>` | Remove a member |
| `kaneo ws update-role <id> <role>` | Update member role |
| `kaneo ws leave` | Leave workspace |
| `kaneo ws set-active <id>` | Set active workspace |
| `kaneo ws check-slug <slug>` | Check slug availability |
| `kaneo ws invitations` | List pending invitations |
| `kaneo ws accept-invitation <id>` | Accept invitation |
| `kaneo ws reject-invitation <id>` | Reject invitation |

### Projects

| Command | Description |
|---------|-------------|
| `kaneo proj ls` | List projects |
| `kaneo proj get <id>` | Get project details |
| `kaneo proj create <name>` | Create a project |
| `kaneo proj update <id>` | Update a project |
| `kaneo proj delete <id>` | Delete a project |

### Tasks

| Command | Description |
|---------|-------------|
| `kaneo t ls <project_id>` | List tasks (board view) |
| `kaneo t get <id>` | Get task details |
| `kaneo t create <project_id> <title>` | Create a task |
| `kaneo t status <id> <status>` | Update status |
| `kaneo t priority <id> <priority>` | Update priority |
| `kaneo t assign <id> [user_id]` | Assign/unassign |
| `kaneo t title <id> <title>` | Update title |
| `kaneo t description <id> <desc>` | Update description |
| `kaneo t due-date <id> [date]` | Set/clear due date |
| `kaneo t delete <id>` | Delete a task |
| `kaneo t export <project_id>` | Export tasks as JSON |
| `kaneo t import <project_id> <file>` | Import tasks from JSON |
| `kaneo t upload <task_id> <file>` | Upload image to task |
| `kaneo t asset <id>` | Download attachment |

### Columns

| Command | Description |
|---------|-------------|
| `kaneo col ls <project_id>` | List columns |
| `kaneo col create <project_id> <name>` | Create column |
| `kaneo col update <id>` | Update column |
| `kaneo col reorder <project_id> <ids>` | Reorder columns |
| `kaneo col delete <id>` | Delete column |

### Labels

| Command | Description |
|---------|-------------|
| `kaneo label ls` | List workspace labels |
| `kaneo label task <task_id>` | List labels on task |
| `kaneo label create <name> --color <hex>` | Create label |
| `kaneo label update <id>` | Update label |
| `kaneo label delete <id>` | Delete label |

### Activity & Comments

| Command | Description |
|---------|-------------|
| `kaneo activity ls <task_id>` | List task activity |
| `kaneo activity comment <task_id> <text>` | Add comment |
| `kaneo activity edit-comment <id> <text>` | Edit comment |
| `kaneo activity delete-comment <id>` | Delete comment |

### Notifications

| Command | Description |
|---------|-------------|
| `kaneo notif ls` | List notifications |
| `kaneo notif read <id>` | Mark as read |
| `kaneo notif read-all` | Mark all as read |
| `kaneo notif clear-all` | Clear all |

### Time Tracking

| Command | Description |
|---------|-------------|
| `kaneo time ls <task_id>` | List time entries |
| `kaneo time get <id>` | Get time entry |
| `kaneo time create <task_id> <start>` | Create entry |
| `kaneo time update <id> <start>` | Update entry |

### Other

| Command | Description |
|---------|-------------|
| `kaneo search <query>` | Search tasks, projects, comments |
| `kaneo api-check` | Validate CLI against server API |

## Output Modes

By default, `kaneo` outputs human-readable colored text to stderr.

When stdout is not a TTY (piped), it automatically switches to JSON output. You can also force it:

```bash
# Force JSON output
kaneo --json project ls

# Force human output even when piped
kaneo --human project ls | cat
```

## Configuration

Credentials are stored in `~/.config/kaneo/config.json` (Linux/macOS) or `%APPDATA%\kaneo\config.json` (Windows).

**Environment variables:**

| Variable | Description |
|----------|-------------|
| `KANEO_API_KEY` | API key (overrides config) |
| `KANEO_API_URL` | API base URL |
| `KANEO_WORKSPACE` | Default workspace ID |
| `KANEO_JSON` | Force JSON output (`true`) |

**Global flags:**

| Flag | Description |
|------|-------------|
| `--token <key>` | API key (highest priority) |
| `--api-url <url>` | API base URL |
| `-w <id>` | Workspace ID |
| `--json` | Force JSON output |
| `--human` | Force human output |

## Supported Platforms

| Platform | Architecture |
|----------|-------------|
| Linux | x86_64 |
| macOS | x86_64, Apple Silicon |
| Windows | x86_64 |

## License

[MIT](LICENSE)
