use anyhow::{Context, bail};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use serde::de::DeserializeOwned;
use serde::Serialize;

pub struct ApiClient {
    client: reqwest::Client,
    base_url: String,
}

impl ApiClient {
    pub fn new(base_url: &str, api_key: &str) -> anyhow::Result<Self> {
        let mut headers = HeaderMap::new();
        let auth_value = format!("Bearer {api_key}");
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&auth_value).context("invalid API key characters")?,
        );

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .context("failed to build HTTP client")?;

        let base_url = base_url.trim_end_matches('/').to_string();

        Ok(Self { client, base_url })
    }

    pub fn anonymous(base_url: &str) -> anyhow::Result<Self> {
        let client = reqwest::Client::builder()
            .build()
            .context("failed to build HTTP client")?;

        let base_url = base_url.trim_end_matches('/').to_string();

        Ok(Self { client, base_url })
    }

    fn url(&self, path: &str) -> String {
        format!("{}/api{path}", self.base_url)
    }

    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> anyhow::Result<T> {
        let url = self.url(path);
        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .with_context(|| format!("GET {url}"))?;

        handle_response(resp, "GET", &url).await
    }

    pub async fn get_query<T: DeserializeOwned, Q: Serialize>(
        &self,
        path: &str,
        query: &Q,
    ) -> anyhow::Result<T> {
        let url = self.url(path);
        let resp = self
            .client
            .get(&url)
            .query(query)
            .send()
            .await
            .with_context(|| format!("GET {url}"))?;

        handle_response(resp, "GET", &url).await
    }

    pub async fn post<B: Serialize, T: DeserializeOwned>(
        &self,
        path: &str,
        body: &B,
    ) -> anyhow::Result<T> {
        let url = self.url(path);
        let resp = self
            .client
            .post(&url)
            .json(body)
            .send()
            .await
            .with_context(|| format!("POST {url}"))?;

        handle_response(resp, "POST", &url).await
    }

    pub async fn put<B: Serialize, T: DeserializeOwned>(
        &self,
        path: &str,
        body: &B,
    ) -> anyhow::Result<T> {
        let url = self.url(path);
        let resp = self
            .client
            .put(&url)
            .json(body)
            .send()
            .await
            .with_context(|| format!("PUT {url}"))?;

        handle_response(resp, "PUT", &url).await
    }

    pub async fn patch<B: Serialize, T: DeserializeOwned>(
        &self,
        path: &str,
        body: &B,
    ) -> anyhow::Result<T> {
        let url = self.url(path);
        let resp = self
            .client
            .patch(&url)
            .json(body)
            .send()
            .await
            .with_context(|| format!("PATCH {url}"))?;

        handle_response(resp, "PATCH", &url).await
    }

    pub async fn patch_empty<T: DeserializeOwned>(&self, path: &str) -> anyhow::Result<T> {
        let url = self.url(path);
        let resp = self
            .client
            .patch(&url)
            .send()
            .await
            .with_context(|| format!("PATCH {url}"))?;

        handle_response(resp, "PATCH", &url).await
    }

    pub async fn get_bytes(&self, path: &str) -> anyhow::Result<(Vec<u8>, String)> {
        let url = self.url(path);
        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .with_context(|| format!("GET {url}"))?;

        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            bail!("GET {url} → {status}: {body}");
        }

        let content_type = resp
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("application/octet-stream")
            .to_string();

        let bytes = resp
            .bytes()
            .await
            .with_context(|| format!("reading bytes from GET {url}"))?;

        Ok((bytes.to_vec(), content_type))
    }

    pub async fn delete<T: DeserializeOwned>(&self, path: &str) -> anyhow::Result<T> {
        let url = self.url(path);
        let resp = self
            .client
            .delete(&url)
            .send()
            .await
            .with_context(|| format!("DELETE {url}"))?;

        handle_response(resp, "DELETE", &url).await
    }

    pub async fn delete_json<B: Serialize, T: DeserializeOwned>(
        &self,
        path: &str,
        body: &B,
    ) -> anyhow::Result<T> {
        let url = self.url(path);
        let resp = self
            .client
            .delete(&url)
            .json(body)
            .send()
            .await
            .with_context(|| format!("DELETE {url}"))?;

        handle_response(resp, "DELETE", &url).await
    }
}

/// Upload bytes to an external URL (e.g. S3 presigned URL) — no auth headers.
pub async fn upload_to_presigned_url(
    url: &str,
    data: Vec<u8>,
    content_type: &str,
) -> anyhow::Result<()> {
    let client = reqwest::Client::new();
    let resp = client
        .put(url)
        .header("content-type", content_type)
        .body(data)
        .send()
        .await
        .with_context(|| format!("PUT {url}"))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        bail!("S3 upload failed: {status}: {body}");
    }

    Ok(())
}

async fn handle_response<T: DeserializeOwned>(
    resp: reqwest::Response,
    method: &str,
    url: &str,
) -> anyhow::Result<T> {
    let status = resp.status();

    if !status.is_success() {
        let body = resp
            .text()
            .await
            .unwrap_or_else(|_| "<unreadable body>".into());

        // Try to extract a message from JSON error response
        if let Ok(val) = serde_json::from_str::<serde_json::Value>(&body) {
            if let Some(msg) = val.get("message").and_then(|m| m.as_str()) {
                bail!("{method} {url} → {status}: {msg}");
            }
        }

        bail!("{method} {url} → {status}: {body}");
    }

    let body = resp
        .text()
        .await
        .with_context(|| format!("reading response body from {method} {url}"))?;

    serde_json::from_str::<T>(&body)
        .with_context(|| format!("parsing response from {method} {url}: {body}"))
}
