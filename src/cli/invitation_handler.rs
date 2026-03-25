use crate::api::ApiClient;
use crate::api::types::{Invitation, InvitationDetails};
use crate::auth::ResolvedContext;
use crate::cli::{InvitationArgs, InvitationCommand};
use crate::output;

pub async fn run(args: InvitationArgs, ctx: &ResolvedContext, json: bool) -> anyhow::Result<()> {
    let client = ApiClient::new(&ctx.api_url, &ctx.api_key)?;

    match args.command {
        InvitationCommand::Pending => {
            let invitations: Vec<Invitation> = client.get("/invitation/pending").await?;

            if json {
                output::json_output(&invitations);
            } else {
                if invitations.is_empty() {
                    output::warn(false, "No pending invitations");
                    return Ok(());
                }
                let bold = console::Style::new().bold();
                let dim = console::Style::new().dim();
                for inv in &invitations {
                    eprintln!(
                        "  📩 {} → {} {}",
                        bold.apply_to(&inv.workspace_name),
                        dim.apply_to(&inv.email),
                        dim.apply_to(&inv.id),
                    );
                    eprintln!(
                        "    {} {} {}",
                        dim.apply_to("from:"),
                        inv.inviter_name,
                        dim.apply_to(format!("expires: {}", inv.expires_at)),
                    );
                }
            }
        }

        InvitationCommand::Get { id } => {
            let details: InvitationDetails = client.get(&format!("/invitation/{id}")).await?;

            if json {
                output::json_output(&details);
            } else {
                let bold = console::Style::new().bold();
                let dim = console::Style::new().dim();
                if details.valid {
                    if let Some(inv) = &details.invitation {
                        eprintln!("  {}", bold.apply_to("Valid invitation"));
                        eprintln!("  {} {}", dim.apply_to("workspace:"), inv.workspace_name);
                        eprintln!("  {} {}", dim.apply_to("email:"), inv.email);
                        eprintln!("  {} {}", dim.apply_to("from:"), inv.inviter_name);
                        eprintln!("  {} {}", dim.apply_to("status:"), inv.status);
                        eprintln!("  {} {}", dim.apply_to("expires:"), inv.expires_at);
                        if inv.expired {
                            output::warn(false, "This invitation has expired");
                        }
                    }
                } else {
                    let err = details.error.as_deref().unwrap_or("Invalid invitation");
                    output::warn(false, err);
                }
            }
        }
    }

    Ok(())
}
