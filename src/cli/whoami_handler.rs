use crate::api::ApiClient;
use crate::api::types::SessionResponse;
use crate::auth::ResolvedContext;
use crate::output;

pub async fn run(ctx: &ResolvedContext, json: bool) -> anyhow::Result<()> {
    let client = ApiClient::new(&ctx.api_url, &ctx.api_key)?;
    let session: SessionResponse = client.get("/auth/get-session").await?;

    match session.user {
        Some(user) => {
            if json {
                output::json_output(&user);
            } else {
                let bold = console::Style::new().bold();
                let dim = console::Style::new().dim();
                eprintln!(
                    "  {} {}",
                    bold.apply_to(&user.name),
                    dim.apply_to(&user.email)
                );
                eprintln!("  {} {}", dim.apply_to("id:"), user.id);
                if let Some(ws) = &ctx.workspace_id {
                    eprintln!("  {} {ws}", dim.apply_to("workspace:"));
                }
                eprintln!("  {} {}", dim.apply_to("api:"), ctx.api_url);
            }
        }
        None => {
            anyhow::bail!("not authenticated — run `kaneo login`");
        }
    }

    Ok(())
}
