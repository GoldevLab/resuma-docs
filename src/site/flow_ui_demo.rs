//! Live resuma-flow widget demo — ops dashboard (execution panel: see `/docs/exec`).

use resuma::prelude::*;
use resuma_flow::{flow_dashboard_poll, flow_styles_link};

/// Interactive Flow UI panel for `/docs/exec/flow_ui`.
pub fn flow_ui_demo() -> View {
    let status = resuma::exec::exec_status();

    view! {
        <div class="flow-ui-demo">
            {flow_styles_link()}
            <p class="demo-muted">
                "Real "
                <code>"resuma-flow"</code>
                " widgets on this server — ops cards poll "
                <code>"exec_status"</code>
                ". For the full execution panel (graph, controls, SSE stream), use the "
                <a href="/docs/exec">"Resuma OS overview"</a>
                " demo."
            </p>

            <div class="flow-ui-demo-section">
                <h4 class="flow-ui-demo-heading">"Ops dashboard"</h4>
                <p class="demo-muted flow-ui-demo-hint">
                    <code>"flow_dashboard_poll(5000, Some(exec_status()))"</code>
                </p>
                <div class="flow-ui-demo-dash">
                    {flow_dashboard_poll(5000, Some(status))}
                </div>
            </div>

            <p class="demo-muted flow-ui-demo-hint">
                "Other widgets: "
                <code>"flow_graph_auth"</code>
                ", "
                <code>"event_stream_auth"</code>
                ", "
                <code>"flow_execution_panel_auth"</code>
                " — mounted dynamically after "
                <code>"FlowEngine::start"</code>
                ". See "
                <a href="/docs/exec">"Resuma OS live demo"</a>
                "."
            </p>
        </div>
    }
}
