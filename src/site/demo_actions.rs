//! Server actions, submits, and loaders backing documentation live demos.

use resuma::prelude::*;
use serde::{Deserialize, Serialize};

pub fn docs_timestamp() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    format!("unix:{secs}")
}

#[server]
pub async fn docs_echo(msg: String) -> String {
    format!("Echo: {msg}")
}

#[server]
pub async fn docs_add(a: i32, b: i32) -> i32 {
    a + b
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocsGreetForm {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocsGreetResult {
    pub message: String,
}

#[submit]
pub async fn docs_greet(
    data: DocsGreetForm,
    _req: &FlowRequest,
) -> std::result::Result<DocsGreetResult, SubmitError> {
    if data.name.trim().is_empty() {
        return Err(SubmitError::new("Invalid input").field("name", "Name required"));
    }
    Ok(DocsGreetResult {
        message: format!("Hello, {}!", data.name.trim()),
    })
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocsSearchData {
    pub query: String,
    pub results: Vec<String>,
}

#[load]
pub async fn docs_search(req: &FlowRequest) -> DocsSearchData {
    let q = req.query_param("q").unwrap_or("").trim().to_string();
    let results = if q.len() >= 2 {
        vec![
            format!("Result A for '{q}'"),
            format!("Result B for '{q}'"),
            format!("Result C for '{q}'"),
        ]
    } else {
        vec![]
    };
    DocsSearchData { query: q, results }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocsCachedData {
    pub value: String,
    pub timestamp: String,
}

#[load(cache = "public, max-age=60")]
pub async fn docs_cached(_req: &FlowRequest) -> DocsCachedData {
    DocsCachedData {
        value: "Cached loader response".into(),
        timestamp: docs_timestamp(),
    }
}

fn docs_delayed_stream_view(data: &str) -> View {
    view! { <p class="demo-output">{data.to_string()}</p> }
}

#[load(stream)]
pub async fn docs_delayed(_req: &FlowRequest) -> String {
    tokio::time::sleep(std::time::Duration::from_millis(600)).await;
    "Streamed after ~600ms delay".into()
}

#[server]
pub async fn docs_loader_stamp() -> String {
    docs_timestamp()
}

#[server]
pub async fn docs_e2e_ping() -> Result<serde_json::Value> {
    Ok(serde_json::json!({
        "ok": true,
        "service": "resuma-docs",
        "timestamp": docs_timestamp(),
    }))
}

#[server]
pub async fn docs_deploy_info() -> Result<serde_json::Value> {
    let env = std::env::var("RESUMA_ENV").unwrap_or_else(|_| "development".into());
    Ok(serde_json::json!({
        "resuma_env": env,
        "trust_proxy": std::env::var("RESUMA_TRUST_PROXY").unwrap_or_default(),
        "site_url": std::env::var("SITE_URL").unwrap_or_else(|_| "https://resuma-docs.fly.dev".into()),
        "addr": std::env::var("RESUMA_ADDR").unwrap_or_else(|_| "127.0.0.1:3000".into()),
        "container_hint": "docker run -p 3000:3000 -e RESUMA_ENV=production resuma-docs",
    }))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocsPrgForm {
    pub item: String,
}

#[submit]
pub async fn docs_prg(
    data: DocsPrgForm,
    _req: &FlowRequest,
) -> std::result::Result<Redirect, SubmitError> {
    if data.item.trim().is_empty() {
        return Err(SubmitError::new("Invalid").field("item", "Item required"));
    }
    Ok(Redirect::to("/docs/cookbook/prg?ok=1"))
}
