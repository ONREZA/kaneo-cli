//! Integration test: validates that every route in `api::routes::ALL_ROUTES`
//! exists in the Kaneo OpenAPI specification.
//!
//! Runs against a Kaneo instance. Requires network access.
//!
//! ```
//! cargo test --test validate_routes -- --ignored
//! ```

use kaneo::api::routes::ALL_ROUTES;

/// Normalise a path pattern like `/task/{id}` → `/task/{id}` (OpenAPI uses `{param}`).
/// Both our constants and the OpenAPI spec use the same `{param}` format,
/// but we strip query strings since the spec doesn't include them in paths.
fn normalise_path(path: &str) -> String {
    path.split('?').next().unwrap_or(path).to_string()
}

#[tokio::test]
#[ignore] // requires network — run with: cargo test --test validate_routes -- --ignored
async fn all_routes_exist_in_openapi_spec() {
    let base_url =
        std::env::var("KANEO_TEST_URL").unwrap_or_else(|_| "https://kaneo.onreza.ru".to_string());
    let openapi_url = format!("{base_url}/api/openapi");

    let resp = reqwest::get(&openapi_url)
        .await
        .expect("failed to fetch OpenAPI spec");

    assert!(
        resp.status().is_success(),
        "OpenAPI endpoint returned {}",
        resp.status()
    );

    let spec: serde_json::Value = resp.json().await.expect("failed to parse OpenAPI spec");
    let paths = spec
        .get("paths")
        .and_then(|v| v.as_object())
        .expect("OpenAPI spec missing 'paths' object");

    // Build a set of (METHOD, PATH) from the spec
    let mut spec_routes: std::collections::HashSet<(String, String)> =
        std::collections::HashSet::new();
    for (path, methods) in paths {
        if let Some(methods) = methods.as_object() {
            for method in methods.keys() {
                spec_routes.insert((method.to_uppercase(), path.clone()));
            }
        }
    }

    let mut missing = Vec::new();

    for (method, path) in ALL_ROUTES {
        let norm_path = normalise_path(path);
        if !spec_routes.contains(&(method.to_string(), norm_path.clone())) {
            missing.push(format!("{method} {norm_path}"));
        }
    }

    assert!(
        missing.is_empty(),
        "Routes missing from OpenAPI spec at {base_url}:\n  {}",
        missing.join("\n  ")
    );

    eprintln!(
        "  ✓ All {} CLI routes validated against {base_url}",
        ALL_ROUTES.len()
    );
}
