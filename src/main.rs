mod api;
mod auth;
mod cli;
mod config;
mod output;
mod upgrade;

use clap::Parser;
use cli::{Cli, Command};

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let json = !cli.human && (cli.json || !std::io::IsTerminal::is_terminal(&std::io::stdout()));

    // Spawn background version check if cache is stale (non-blocking)
    let is_upgrade_cmd = matches!(cli.command, Command::Upgrade(_));
    if !json && !is_upgrade_cmd && upgrade::should_check_version() {
        upgrade::spawn_version_check();
    }

    // Check cached version hint (instant, no network)
    let update_hint = if !json && !is_upgrade_cmd {
        upgrade::check_cached_update()
    } else {
        None
    };

    let result = run(cli, json).await;

    // Show update hint after command output
    if let Some(version) = update_hint {
        let dim = console::Style::new().dim();
        let yellow = console::Style::new().yellow();
        eprintln!(
            "\n  {} {} → {} {}",
            yellow.apply_to("Update available:"),
            dim.apply_to(concat!("v", env!("CARGO_PKG_VERSION"))),
            yellow.apply_to(format!("v{version}")),
            dim.apply_to("(run `kaneo upgrade`)"),
        );
    }

    if let Err(ref e) = result {
        if json {
            let err = serde_json::json!({ "error": format!("{e:#}") });
            println!("{}", serde_json::to_string(&err).unwrap());
        } else {
            output::error_msg(&format!("{e:#}"));
        }
        std::process::exit(1);
    }
}

async fn run(cli: Cli, json: bool) -> anyhow::Result<()> {
    let token = cli.token.as_deref();
    let api_url = cli.api_url.as_deref();
    let workspace = cli.workspace.as_deref();
    let project = cli.project.as_deref();

    match cli.command {
        Command::Login(args) => {
            cli::login_handler::run(args, json)?;
        }
        Command::Logout => {
            cli::login_handler::logout(json)?;
        }
        Command::Profile(args) => {
            cli::profile_handler::run(args, json)?;
        }
        Command::Link(args) => {
            let config = auth::LocalConfig {
                workspace: args.workspace,
                project: args.project,
            };
            let cwd = std::env::current_dir()?;
            auth::write_local_config(&cwd, &config)?;
            if json {
                output::json_output(&serde_json::json!({
                    "linked": true,
                    "path": cwd.join(".kaneo.json").display().to_string(),
                    "workspace": config.workspace,
                    "project": config.project,
                }));
            } else {
                output::success(
                    false,
                    &format!("Linked {}", cwd.join(".kaneo.json").display()),
                );
            }
        }
        Command::Unlink => {
            let cwd = std::env::current_dir()?;
            let path = cwd.join(".kaneo.json");
            if path.exists() {
                std::fs::remove_file(&path)?;
                if json {
                    output::json_output(&serde_json::json!({ "unlinked": true }));
                } else {
                    output::success(false, "Removed .kaneo.json");
                }
            } else if json {
                output::json_output(&serde_json::json!({ "unlinked": false }));
            } else {
                output::warn(false, "No .kaneo.json in current directory");
            }
        }
        Command::Context => {
            let ctx = auth::resolve_context(token, api_url, workspace, project)?;
            if json {
                output::json_output(&serde_json::json!({
                    "api_url": ctx.api_url,
                    "workspace_id": ctx.workspace_id,
                    "project_id": ctx.project_id,
                }));
            } else {
                let dim = console::Style::new().dim();
                let bold = console::Style::new().bold();
                eprintln!("  {} {}", dim.apply_to("api:"), bold.apply_to(&ctx.api_url));
                eprintln!(
                    "  {} {}",
                    dim.apply_to("workspace:"),
                    ctx.workspace_id.as_deref().unwrap_or("(none)")
                );
                eprintln!(
                    "  {} {}",
                    dim.apply_to("project:"),
                    ctx.project_id.as_deref().unwrap_or("(none)")
                );
            }
        }
        Command::Whoami => {
            let ctx = auth::resolve_context(token, api_url, workspace, project)?;
            cli::whoami_handler::run(&ctx, json).await?;
        }
        Command::Workspace(args) => {
            let ctx = auth::resolve_context(token, api_url, workspace, project)?;
            cli::workspace_handler::run(args, &ctx, json).await?;
        }
        Command::Project(args) => {
            let ctx = auth::resolve_context(token, api_url, workspace, project)?;
            cli::project_handler::run(args, &ctx, json).await?;
        }
        Command::Task(args) => {
            let ctx = auth::resolve_context(token, api_url, workspace, project)?;
            cli::task_handler::run(args, &ctx, json).await?;
        }
        Command::Column(args) => {
            let ctx = auth::resolve_context(token, api_url, workspace, project)?;
            cli::column_handler::run(args, &ctx, json).await?;
        }
        Command::Label(args) => {
            let ctx = auth::resolve_context(token, api_url, workspace, project)?;
            cli::label_handler::run(args, &ctx, json).await?;
        }
        Command::Activity(args) => {
            let ctx = auth::resolve_context(token, api_url, workspace, project)?;
            cli::activity_handler::run(args, &ctx, json).await?;
        }
        Command::Notification(args) => {
            let ctx = auth::resolve_context(token, api_url, workspace, project)?;
            cli::notification_handler::run(args, &ctx, json).await?;
        }
        Command::TimeEntry(args) => {
            let ctx = auth::resolve_context(token, api_url, workspace, project)?;
            cli::time_entry_handler::run(args, &ctx, json).await?;
        }
        Command::Search(args) => {
            let ctx = auth::resolve_context(token, api_url, workspace, project)?;
            cli::search_handler::run(args, &ctx, json).await?;
        }
        Command::ApiCheck => {
            let ctx =
                auth::resolve_context(token, api_url, workspace, project).unwrap_or_else(|_| {
                    auth::ResolvedContext {
                        api_url: api_url.unwrap_or("https://cloud.kaneo.app").to_string(),
                        api_key: String::new(),
                        workspace_id: None,
                        project_id: None,
                    }
                });
            cli::api_check_handler::run(&ctx, json).await?;
        }
        Command::Upgrade(args) => {
            upgrade::run(args).await?;
        }
    }

    Ok(())
}
