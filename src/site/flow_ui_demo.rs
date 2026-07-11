//! Live resuma-flow widget demo — ops dashboard + execution panel.

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
                "; the execution panel is SSR from "
                <code>"flow_execution_auth"</code>
                " after you start a worker."
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

            <div class="flow-ui-demo-section">
                <h4 class="flow-ui-demo-heading">"Execution panel"</h4>
                <p class="demo-muted flow-ui-demo-hint">
                    "Run "
                    <code>"docs_showcase"</code>
                    " — graph, pause/resume/cancel, and SSE event stream mount below."
                </p>
                <div class="exec-demo-controls flow-ui-demo-controls">
                    <label class="exec-demo-label" for="flow-ui-topic">
                        "Topic"
                        <input id="flow-ui-topic" type="text" name="flow_ui_topic" value="Resuma OS" />
                    </label>
                    <label class="exec-demo-label" for="flow-ui-blurb">
                        "Text to analyze"
                        <textarea id="flow-ui-blurb" name="flow_ui_blurb" rows="3">"Durable workers with checkpointed graphs, queue recovery, and an ops dashboard — all in Rust."</textarea>
                    </label>
                    <button
                        type="button"
                        class="btn btn-primary"
                        id="flow-ui-start-btn"
                        onClick={js!(async () => {
                            const m = await import("/static/client/docs-flow-worker.js");
                            await m.runDocsFlowWorker("flow-ui");
                        })}
                    >
                        "Run worker"
                    </button>
                    <p id="flow-ui-err" class="exec-demo-err" role="alert" hidden></p>
                    {crate::site::exec_guide::worker_try_it_guide("flow-ui")}
                </div>
                {crate::site::exec_guide::worker_panel_placeholder("flow-ui-flow-placeholder")}
                <div id="flow-ui-slot" class="exec-flow-slot" hidden></div>
            </div>
        </div>
    }
}
