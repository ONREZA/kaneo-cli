mod api;
mod auth;
mod cli;
mod config;
mod output;

use clap::Parser;
use cli::{Cli, Command};

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let json = !cli.human && (cli.json || !std::io::IsTerminal::is_terminal(&std::io::stdout()));

    let result = run(cli, json).await;

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
    // Commands that don't need auth context
    match &cli.command {
        Command::Login(_) | Command::Logout | Command::Profile(_) => {}
        _ => {}
    }

    let token = cli.token.as_deref();
    let api_url = cli.api_url.as_deref();
    let workspace = cli.workspace.as_deref();

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
        Command::Whoami => {
            let ctx = auth::resolve_context(token, api_url, workspace)?;
            cli::whoami_handler::run(&ctx, json).await?;
        }
        Command::Workspace(args) => {
            let ctx = auth::resolve_context(token, api_url, workspace)?;
            cli::workspace_handler::run(args, &ctx, json).await?;
        }
        Command::Project(args) => {
            let ctx = auth::resolve_context(token, api_url, workspace)?;
            cli::project_handler::run(args, &ctx, json).await?;
        }
        Command::Task(args) => {
            let ctx = auth::resolve_context(token, api_url, workspace)?;
            cli::task_handler::run(args, &ctx, json).await?;
        }
        Command::Column(args) => {
            let ctx = auth::resolve_context(token, api_url, workspace)?;
            cli::column_handler::run(args, &ctx, json).await?;
        }
        Command::Label(args) => {
            let ctx = auth::resolve_context(token, api_url, workspace)?;
            cli::label_handler::run(args, &ctx, json).await?;
        }
        Command::Activity(args) => {
            let ctx = auth::resolve_context(token, api_url, workspace)?;
            cli::activity_handler::run(args, &ctx, json).await?;
        }
        Command::Notification(args) => {
            let ctx = auth::resolve_context(token, api_url, workspace)?;
            cli::notification_handler::run(args, &ctx, json).await?;
        }
        Command::TimeEntry(args) => {
            let ctx = auth::resolve_context(token, api_url, workspace)?;
            cli::time_entry_handler::run(args, &ctx, json).await?;
        }
        Command::Search(args) => {
            let ctx = auth::resolve_context(token, api_url, workspace)?;
            cli::search_handler::run(args, &ctx, json).await?;
        }
        Command::ApiCheck => {
            // api-check only needs the URL, not auth — try resolve but fall back to URL-only
            let ctx = auth::resolve_context(token, api_url, workspace).unwrap_or_else(|_| {
                auth::ResolvedContext {
                    api_url: api_url
                        .unwrap_or("https://cloud.kaneo.app")
                        .to_string(),
                    api_key: String::new(),
                    workspace_id: None,
                }
            });
            cli::api_check_handler::run(&ctx, json).await?;
        }
    }

    Ok(())
}
