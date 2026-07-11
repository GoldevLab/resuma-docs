//! Live resuma-flow widget demo — ops dashboard + execution panel.

use resuma::prelude::*;
use resuma_flow::{flow_dashboard_poll, flow_styles};

/// Interactive Flow UI panel for `/docs/exec/flow_ui`.
pub fn flow_ui_demo() -> View {
    let status = resuma::exec::exec_status();

    view! {
        <div class="flow-ui-demo">
            {flow_styles()}
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
                        onClick={js!(async (_event, _state, __resuma) => {
                            const topic = document.getElementById("flow-ui-topic").value;
                            const blurb = document.getElementById("flow-ui-blurb").value;
                            const errEl = document.getElementById("flow-ui-err");
                            const slot = document.getElementById("flow-ui-slot");
                            const btn = document.getElementById("flow-ui-start-btn");
                            errEl.hidden = true;
                            btn.disabled = true;
                            const res = await __resuma.safeAction("start_docs_showcase", [topic, blurb]);
                            btn.disabled = false;
                            if (!res.ok) {
                                errEl.textContent = res.error;
                                errEl.hidden = false;
                                return;
                            }
                            const graphId = res.value.graph_id;
                            const token = res.value.access_token || "";
                            if (!graphId) {
                                errEl.textContent = "Worker started but no graph id was returned.";
                                errEl.hidden = false;
                                return;
                            }
                            const prev = slot.querySelector("[data-r-flow-execution]");
                            if (window.__resumaCoreReady) await window.__resumaCoreReady;
                            let flow;
                            try {
                                flow = await import("/_resuma/flow.js");
                            } catch (e) {
                                errEl.textContent = "Could not load Flow widgets: " + String(e);
                                errEl.hidden = false;
                                return;
                            }
                            if (prev) flow.disconnectFlowWidgets(prev);
                            slot.innerHTML = "";
                            const panelRes = await __resuma.safeAction("flow_execution_panel_html", [graphId, token]);
                            if (!panelRes.ok) {
                                errEl.textContent = panelRes.error;
                                errEl.hidden = false;
                                return;
                            }
                            slot.innerHTML = panelRes.value;
                            slot.querySelectorAll("style[data-r-flow-styles]").forEach((n) => n.remove());
                            slot.hidden = false;
                            flow.initFlowWidgets(slot, { flush: false });
                        })}
                    >
                        "Run worker"
                    </button>
                    <p id="flow-ui-err" class="exec-demo-err" role="alert" hidden></p>
                </div>
                <div id="flow-ui-slot" class="exec-flow-slot" hidden></div>
            </div>
        </div>
    }
}
