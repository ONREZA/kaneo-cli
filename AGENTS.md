# kaneo-cli — LLM Agent Contract

## Overview

`kaneo` is a CLI for [Kaneo](https://kaneo.app/) project management. It supports both machine-readable (JSON) and human-readable output modes.

## Output Modes

### JSON mode (for agents/pipes)
- Activated: `--json` flag, `KANEO_JSON=true` env, or when stdout is not a TTY
- Single JSON object to stdout
- Errors: `{"error": "message"}` with exit code 1

### Human mode (for terminals)
- Activated: default when stdout is a TTY, or `--human` flag
- Colored output to stderr
- Errors: styled error messages to stderr with exit code 1

## Global Flags & Environment

| Flag | Env | Description |
|------|-----|-------------|
| `--json` | `KANEO_JSON` | Force JSON output |
| `--human` | — | Force human output |
| `--token <key>` | `KANEO_API_KEY` | API key |
| `--api-url <url>` | `KANEO_API_URL` | API base URL |
| `-w <id>` | `KANEO_WORKSPACE` | Workspace ID |

## Auth Resolution Order

1. `--token` flag
2. `KANEO_API_KEY` env var
3. `~/.config/kaneo/config.json` active profile

## Module Structure

```
src/
├── main.rs              — Entry point, command routing
├── api/
│   ├── client.rs        — HTTP client (ApiClient)
│   └── types.rs         — API types (Rust structs matching Kaneo schemas)
├── auth/
│   └── mod.rs           — Auth config, profiles, context resolution
├── cli/
│   ├── mod.rs           — Clap definitions (all commands/subcommands)
│   ├── task_handler.rs
│   ├── project_handler.rs
│   ├── workspace_handler.rs
│   ├── column_handler.rs
│   ├── label_handler.rs
│   ├── activity_handler.rs
│   ├── notification_handler.rs
│   ├── time_entry_handler.rs
│   ├── search_handler.rs
│   ├── login_handler.rs
│   ├── profile_handler.rs
│   ├── whoami_handler.rs
│   └── api_check_handler.rs
├── config/              — (reserved)
└── output.rs            — Output helpers (json, success, warn, error, status)
```

## API Details

- **Base URL**: configurable, default `https://cloud.kaneo.app`
- **Auth**: Bearer token in Authorization header
- **Workspace CRUD**: Uses better-auth Organization plugin (`/api/auth/organization/*`)
- **All other resources**: REST under `/api/<resource>`
- **File upload**: 3-step presigned S3 URL flow
- **OpenAPI**: Available at `/api/openapi` (public, no auth)

## Adding a New Command

1. Add subcommand to enum in `src/cli/mod.rs`
2. Implement handler in `src/cli/<resource>_handler.rs`
3. Route in `src/main.rs` if new top-level command
4. Add operationId mapping in `src/cli/api_check_handler.rs`
