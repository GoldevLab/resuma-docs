//! Shared SSR execution panel for docs demos (no inline `flow_styles` — uses SITE_CSS).

use resuma::prelude::*;
use resuma::ssr::render_view;
use resuma_flow::{event_stream_auth, flow_graph_auth, worker_panel_auth};

/// `flow_execution_auth` layout without embedded stylesheet (CSP-safe when injected via JS).
pub fn flow_execution_panel_view(graph_id: String, access_token: Option<String>) -> View {
    let gid = graph_id.clone();
    let token = access_token.clone();

    view! {
        <div class="r-flow-exec" data-r-flow-execution={graph_id}>
            <div class="r-flow-exec__panel">
                <h3>"Execution graph"</h3>
                {flow_graph_auth(gid.clone(), true, token.clone())}
            </div>
            <aside class="r-flow-exec__side">
                <div class="r-flow-exec__panel">
                    <h3>"Controls"</h3>
                    {worker_panel_auth(gid.clone(), token.clone())}
                </div>
                <div class="r-flow-exec__panel">
                    <h3>"Event stream"</h3>
                    {event_stream_auth(gid, token)}
                </div>
            </aside>
        </div>
    }
}

#[server]
pub async fn flow_execution_panel_html(graph_id: String, access_token: String) -> Result<String> {
    let token = if access_token.is_empty() {
        None
    } else {
        Some(access_token)
    };
    Ok(render_view(&flow_execution_panel_view(graph_id, token)))
}
