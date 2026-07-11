//! Shared SSR execution panel for docs demos (CSP-safe dynamic injection).

use resuma::prelude::*;
use resuma::ssr::render_view;
use resuma_flow::flow_execution_panel_auth;
use serde_json::{json, Value};

/// Start `docs_showcase` and return SSR panel HTML in one round-trip (token always matches).
#[server]
pub async fn run_docs_flow_worker(topic: String, blurb: String) -> Result<Value> {
    let started =
        resuma::exec::FlowEngine::start("docs_showcase", json!({ "topic": topic, "blurb": blurb }))
            .await?;
    let graph_id = started.graph_id.0.clone();
    let access_token = started.access_token.clone();
    let panel_html = render_view(&flow_execution_panel_auth(
        graph_id.clone(),
        true,
        access_token.clone(),
    ));
    Ok(json!({
        "graph_id": graph_id,
        "access_token": access_token.unwrap_or_default(),
        "panel_html": panel_html,
    }))
}

#[server]
pub async fn flow_execution_panel_html(graph_id: String, access_token: String) -> Result<String> {
    let token = if access_token.is_empty() {
        None
    } else {
        Some(access_token)
    };
    Ok(render_view(&flow_execution_panel_auth(
        graph_id, true, token,
    )))
}
