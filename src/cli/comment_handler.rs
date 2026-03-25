use crate::api::ApiClient;
use crate::api::types::Comment;
use crate::auth::ResolvedContext;
use crate::cli::{CommentArgs, CommentCommand};
use crate::output;
use serde::Serialize;

pub async fn run(args: CommentArgs, ctx: &ResolvedContext, json: bool) -> anyhow::Result<()> {
    let client = ApiClient::new(&ctx.api_url, &ctx.api_key)?;

    match args.command {
        CommentCommand::List { task_id } => {
            let comments: Vec<Comment> = client.get(&format!("/comment/{task_id}")).await?;

            if json {
                output::json_output(&comments);
            } else {
                if comments.is_empty() {
                    output::warn(false, "No comments on this task");
                    return Ok(());
                }
                let bold = console::Style::new().bold();
                let dim = console::Style::new().dim();
                for c in &comments {
                    let author = c
                        .user
                        .as_ref()
                        .map(|u| u.name.as_str())
                        .unwrap_or("unknown");
                    eprintln!(
                        "  {} {} {}",
                        bold.apply_to(author),
                        dim.apply_to(&c.created_at),
                        dim.apply_to(&c.id),
                    );
                    eprintln!("    {}", c.content);
                }
            }
        }

        CommentCommand::Create { task_id, content } => {
            #[derive(Serialize)]
            struct Body {
                content: String,
            }
            let comment: Comment = client
                .post(&format!("/comment/{task_id}"), &Body { content })
                .await?;

            if json {
                output::json_output(&comment);
            } else {
                output::success(false, &format!("Comment added ({})", comment.id));
            }
        }

        CommentCommand::Update { id, content } => {
            #[derive(Serialize)]
            struct Body {
                content: String,
            }
            let comment: Comment = client
                .put(&format!("/comment/{id}"), &Body { content })
                .await?;

            if json {
                output::json_output(&comment);
            } else {
                output::success(false, &format!("Comment updated ({})", comment.id));
            }
        }

        CommentCommand::Delete { id } => {
            let comment: Comment = client.delete(&format!("/comment/{id}")).await?;

            if json {
                output::json_output(&comment);
            } else {
                output::success(false, &format!("Comment deleted ({})", comment.id));
            }
        }
    }

    Ok(())
}
