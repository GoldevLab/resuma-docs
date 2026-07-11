//! Shared SSR execution panel for docs demos (CSP-safe dynamic injection).

use resuma::prelude::*;
use resuma::ssr::render_view;
use resuma_flow::flow_execution_panel_auth;

#[server]
pub async fn flow_execution_panel_html(graph_id: String, access_token: String) -> Result<String> {
    let token = if access_token.is_empty() {
        None
    } else {
        Some(access_token)
    };
    Ok(render_view(&flow_execution_panel_auth(graph_id, true, token)))
}
