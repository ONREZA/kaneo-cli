use crate::api::ApiClient;
use crate::api::types::SessionResponse;
use crate::auth::ResolvedContext;
use crate::output;

pub async fn run(ctx: &ResolvedContext, json: bool) -> anyhow::Result<()> {
    let client = ApiClient::new(&ctx.api_url, &ctx.api_key)?;

    // better-auth returns JSON `null` on API-key auth (sessions are cookie-based),
    // so parse as Option to avoid "invalid type: null" deserialization errors.
    let session: Option<SessionResponse> = client.get("/auth/get-session").await?;
    let user = session.and_then(|s| s.user);

    if json {
        output::json_output(&serde_json::json!({
            "user": user,
            "workspace": ctx.workspace_id,
            "api": ctx.api_url,
        }));
        return Ok(());
    }

    let bold = console::Style::new().bold();
    let dim = console::Style::new().dim();

    match user {
        Some(u) => {
            eprintln!("  {} {}", bold.apply_to(&u.name), dim.apply_to(&u.email));
            eprintln!("  {} {}", dim.apply_to("id:"), u.id);
        }
        None => {
            eprintln!(
                "  {} {}",
                bold.apply_to("(API-key auth)"),
                dim.apply_to("server did not return a user profile"),
            );
        }
    }
    if let Some(ws) = &ctx.workspace_id {
        eprintln!("  {} {ws}", dim.apply_to("workspace:"));
    }
    eprintln!("  {} {}", dim.apply_to("api:"), ctx.api_url);

    Ok(())
}
