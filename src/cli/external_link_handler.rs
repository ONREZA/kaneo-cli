use crate::api::ApiClient;
use crate::api::types::ExternalLink;
use crate::auth::ResolvedContext;
use crate::cli::resolve::resolve_task_id;
use crate::cli::{ExternalLinkArgs, ExternalLinkCommand};
use crate::output;

pub async fn run(args: ExternalLinkArgs, ctx: &ResolvedContext, json: bool) -> anyhow::Result<()> {
    let client = ApiClient::new(&ctx.api_url, &ctx.api_key)?;

    match args.command {
        ExternalLinkCommand::Task { task_id } => {
            let task_id = resolve_task_id(&task_id, ctx, &client).await?;
            let links: Vec<ExternalLink> = client
                .get(&format!("/external-link/task/{task_id}"))
                .await?;

            if json {
                output::json_output(&links);
            } else {
                if links.is_empty() {
                    output::warn(false, "No external links on this task");
                    return Ok(());
                }
                let bold = console::Style::new().bold();
                let dim = console::Style::new().dim();
                for link in &links {
                    let title = link.title.as_deref().unwrap_or(&link.external_id);
                    eprintln!(
                        "  🔗 {} {} {}",
                        bold.apply_to(title),
                        dim.apply_to(&link.url),
                        dim.apply_to(&link.resource_type),
                    );
                }
            }
        }
    }

    Ok(())
}
