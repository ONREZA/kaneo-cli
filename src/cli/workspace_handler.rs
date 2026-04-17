use crate::api::ApiClient;
use crate::auth::ResolvedContext;
use crate::cli::resolve::resolve_workspace;
use crate::cli::{WorkspaceArgs, WorkspaceCommand};
use crate::output;
use serde::Serialize;

pub async fn run(args: WorkspaceArgs, ctx: &ResolvedContext, json: bool) -> anyhow::Result<()> {
    let client = ApiClient::new(&ctx.api_url, &ctx.api_key)?;

    match args.command {
        WorkspaceCommand::List => {
            let result: serde_json::Value = client.get("/auth/organization/list").await?;

            if json {
                output::json_output(&result);
            } else {
                let orgs = result.as_array();
                match orgs {
                    Some(orgs) if !orgs.is_empty() => {
                        let bold = console::Style::new().bold();
                        let dim = console::Style::new().dim();
                        for org in orgs {
                            let name = org.get("name").and_then(|v| v.as_str()).unwrap_or("?");
                            let id = org.get("id").and_then(|v| v.as_str()).unwrap_or("");
                            let slug = org.get("slug").and_then(|v| v.as_str()).unwrap_or("");
                            eprintln!(
                                "  {} {} {}",
                                bold.apply_to(name),
                                dim.apply_to(slug),
                                dim.apply_to(id),
                            );
                        }
                    }
                    _ => output::warn(false, "No workspaces found"),
                }
            }
        }

        WorkspaceCommand::Get { id } => {
            let ws_id = resolve_workspace(id.as_deref(), ctx)?;

            let result: serde_json::Value = client
                .get(&format!(
                    "/auth/organization/get-full-organization?organizationId={ws_id}"
                ))
                .await?;

            if json {
                output::json_output(&result);
            } else {
                print_org_details(&result);
            }
        }

        WorkspaceCommand::Create { name, slug, logo } => {
            #[derive(Serialize)]
            struct Body {
                name: String,
                #[serde(skip_serializing_if = "Option::is_none")]
                slug: Option<String>,
                #[serde(skip_serializing_if = "Option::is_none")]
                logo: Option<String>,
            }
            let result: serde_json::Value = client
                .post("/auth/organization/create", &Body { name, slug, logo })
                .await?;

            if json {
                output::json_output(&result);
            } else {
                let name = result.get("name").and_then(|v| v.as_str()).unwrap_or("?");
                let id = result.get("id").and_then(|v| v.as_str()).unwrap_or("");
                output::success(false, &format!("Created workspace '{name}' ({id})"));
            }
        }

        WorkspaceCommand::Update {
            id,
            name,
            slug,
            logo,
        } => {
            let ws_id = resolve_workspace(id.as_deref(), ctx)?.to_string();

            let mut data = serde_json::Map::new();
            if let Some(n) = &name {
                data.insert("name".into(), serde_json::json!(n));
            }
            if let Some(s) = &slug {
                data.insert("slug".into(), serde_json::json!(s));
            }
            if let Some(l) = &logo {
                data.insert("logo".into(), serde_json::json!(l));
            }

            let body = serde_json::json!({
                "organizationId": ws_id,
                "data": data,
            });

            let result: serde_json::Value = client.post("/auth/organization/update", &body).await?;

            if json {
                output::json_output(&result);
            } else {
                output::success(false, "Workspace updated");
            }
        }

        WorkspaceCommand::Delete { id } => {
            let body = serde_json::json!({ "organizationId": id });
            let result: serde_json::Value = client.post("/auth/organization/delete", &body).await?;

            if json {
                output::json_output(&result);
            } else {
                output::success(false, &format!("Deleted workspace {id}"));
            }
        }

        WorkspaceCommand::Me => {
            let member: crate::api::types::ActiveMember =
                client.get("/auth/organization/get-active-member").await?;

            if json {
                output::json_output(&member);
            } else {
                let bold = console::Style::new().bold();
                let dim = console::Style::new().dim();
                eprintln!("  {} {}", dim.apply_to("member id:"), member.id);
                eprintln!("  {} {}", dim.apply_to("user id:"), member.user_id);
                eprintln!(
                    "  {} {}",
                    dim.apply_to("workspace:"),
                    member.organization_id
                );
                eprintln!(
                    "  {} {}",
                    dim.apply_to("role:"),
                    bold.apply_to(&member.role)
                );
            }
        }

        WorkspaceCommand::Members => {
            let ws_id = resolve_workspace(None, ctx)?;

            let members: Vec<crate::api::types::WorkspaceMemberInfo> =
                client.get(&format!("/workspace/{ws_id}/members")).await?;

            if json {
                output::json_output(&members);
            } else {
                print_members(&members);
            }
        }

        WorkspaceCommand::Invite { email, role } => {
            let ws_id = resolve_workspace(None, ctx)?;

            let body = serde_json::json!({
                "organizationId": ws_id,
                "email": email,
                "role": role,
            });

            let result: serde_json::Value = client
                .post("/auth/organization/invite-member", &body)
                .await?;

            if json {
                output::json_output(&result);
            } else {
                output::success(false, &format!("Invited {email} as {role}"));
            }
        }

        WorkspaceCommand::RemoveMember { user_id } => {
            let ws_id = resolve_workspace(None, ctx)?;

            let body = serde_json::json!({
                "organizationId": ws_id,
                "memberIdOrEmail": user_id,
            });

            let result: serde_json::Value = client
                .post("/auth/organization/remove-member", &body)
                .await?;

            if json {
                output::json_output(&result);
            } else {
                output::success(false, &format!("Removed member {user_id}"));
            }
        }

        WorkspaceCommand::UpdateRole { user_id, role } => {
            let ws_id = resolve_workspace(None, ctx)?;

            let body = serde_json::json!({
                "organizationId": ws_id,
                "memberId": user_id,
                "role": role,
            });

            let result: serde_json::Value = client
                .post("/auth/organization/update-member-role", &body)
                .await?;

            if json {
                output::json_output(&result);
            } else {
                output::success(false, &format!("Updated {user_id} role → {role}"));
            }
        }

        WorkspaceCommand::Leave { id } => {
            let ws_id = resolve_workspace(id.as_deref(), ctx)?;

            let body = serde_json::json!({ "organizationId": ws_id });
            let result: serde_json::Value = client.post("/auth/organization/leave", &body).await?;

            if json {
                output::json_output(&result);
            } else {
                output::success(false, &format!("Left workspace {ws_id}"));
            }
        }

        WorkspaceCommand::SetActive { id } => {
            let ws_id = match id {
                Some(id) => id,
                None if output::is_interactive() => select_workspace(&client).await?,
                None => anyhow::bail!("workspace ID required (or run interactively in a terminal)"),
            };

            let body = serde_json::json!({ "organizationId": ws_id });
            let result: serde_json::Value =
                client.post("/auth/organization/set-active", &body).await?;

            if json {
                output::json_output(&result);
            } else {
                output::success(false, &format!("Active workspace → {ws_id}"));
            }
        }

        WorkspaceCommand::CheckSlug { slug } => {
            let body = serde_json::json!({ "slug": slug });
            let result: serde_json::Value =
                client.post("/auth/organization/check-slug", &body).await?;

            if json {
                output::json_output(&result);
            } else {
                let available = result
                    .get("status")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);
                if available {
                    output::success(false, &format!("Slug '{slug}' is available"));
                } else {
                    output::warn(false, &format!("Slug '{slug}' is taken"));
                }
            }
        }

        WorkspaceCommand::Invitations => {
            let ws_id = resolve_workspace(None, ctx)?;

            let result: serde_json::Value = client
                .get(&format!(
                    "/auth/organization/list-invitations?organizationId={ws_id}"
                ))
                .await?;

            if json {
                output::json_output(&result);
            } else {
                let invitations = result.as_array();
                match invitations {
                    Some(inv) if !inv.is_empty() => {
                        let bold = console::Style::new().bold();
                        let dim = console::Style::new().dim();
                        for i in inv {
                            let email = i.get("email").and_then(|v| v.as_str()).unwrap_or("?");
                            let role = i.get("role").and_then(|v| v.as_str()).unwrap_or("member");
                            let status = i
                                .get("status")
                                .and_then(|v| v.as_str())
                                .unwrap_or("pending");
                            let id = i.get("id").and_then(|v| v.as_str()).unwrap_or("");
                            eprintln!(
                                "  {} {} {} {}",
                                bold.apply_to(email),
                                dim.apply_to(format!("[{role}]")),
                                dim.apply_to(status),
                                dim.apply_to(id),
                            );
                        }
                    }
                    _ => output::warn(false, "No pending invitations"),
                }
            }
        }

        WorkspaceCommand::CancelInvitation { id } => {
            let body = serde_json::json!({ "invitationId": id });
            let result: serde_json::Value = client
                .post("/auth/organization/cancel-invitation", &body)
                .await?;

            if json {
                output::json_output(&result);
            } else {
                output::success(false, &format!("Cancelled invitation {id}"));
            }
        }

        WorkspaceCommand::AcceptInvitation { id } => {
            let body = serde_json::json!({ "invitationId": id });
            let result: serde_json::Value = client
                .post("/auth/organization/accept-invitation", &body)
                .await?;

            if json {
                output::json_output(&result);
            } else {
                output::success(false, &format!("Accepted invitation {id}"));
            }
        }

        WorkspaceCommand::RejectInvitation { id } => {
            let body = serde_json::json!({ "invitationId": id });
            let result: serde_json::Value = client
                .post("/auth/organization/reject-invitation", &body)
                .await?;

            if json {
                output::json_output(&result);
            } else {
                output::success(false, &format!("Rejected invitation {id}"));
            }
        }
    }

    Ok(())
}

async fn select_workspace(client: &ApiClient) -> anyhow::Result<String> {
    let orgs: Vec<serde_json::Value> = client.get("/auth/organization/list").await?;

    if orgs.is_empty() {
        anyhow::bail!("no workspaces found");
    }

    let labels: Vec<String> = orgs
        .iter()
        .map(|o| {
            let name = o.get("name").and_then(|v| v.as_str()).unwrap_or("?");
            let slug = o.get("slug").and_then(|v| v.as_str()).unwrap_or("");
            format!("{name} ({slug})")
        })
        .collect();

    let idx = output::select("Workspace", &labels)?;

    orgs[idx]
        .get("id")
        .and_then(|v| v.as_str())
        .map(String::from)
        .ok_or_else(|| anyhow::anyhow!("workspace missing id"))
}

fn print_org_details(result: &serde_json::Value) {
    let bold = console::Style::new().bold();
    let dim = console::Style::new().dim();
    let name = result.get("name").and_then(|v| v.as_str()).unwrap_or("?");
    let id_val = result.get("id").and_then(|v| v.as_str()).unwrap_or("");
    let slug = result.get("slug").and_then(|v| v.as_str()).unwrap_or("");
    eprintln!("  {}", bold.apply_to(name));
    eprintln!("  {} {id_val}", dim.apply_to("id:"));
    eprintln!("  {} {slug}", dim.apply_to("slug:"));

    if let Some(members) = result.get("members").and_then(|v| v.as_array()) {
        eprintln!("  {} {}", dim.apply_to("members:"), members.len());
    }
}

fn print_members(members: &[crate::api::types::WorkspaceMemberInfo]) {
    if members.is_empty() {
        output::warn(false, "No members found");
        return;
    }
    let bold = console::Style::new().bold();
    let dim = console::Style::new().dim();
    for m in members {
        eprintln!(
            "  {} {} {}",
            bold.apply_to(&m.name),
            dim.apply_to(&m.email),
            dim.apply_to(format!("[{}]", m.role)),
        );
    }
}
