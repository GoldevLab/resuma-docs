//! Live Resuma OS worker demo on the docs homepage.

use resuma::prelude::*;
use resuma_flow::{flow_dashboard_poll, flow_styles};
use serde_json::{json, Value};

#[server]
async fn start_docs_showcase(topic: String, blurb: String) -> Result<Value> {
    let started =
        resuma::exec::FlowEngine::start("docs_showcase", json!({ "topic": topic, "blurb": blurb }))
            .await?;
    Ok(json!({
        "graph_id": started.graph_id.0,
        "access_token": started.access_token.unwrap_or_default(),
    }))
}

/// Interactive worker + live execution graph (Resuma OS).
#[component]
fn ExecShowcaseDemo() -> View {
    let status = resuma::exec::exec_status();

    view! {
        <section class="exec-demo" aria-labelledby="exec-demo-title">
            {flow_styles()}
            <div class="exec-demo-intro">
                <h3 id="exec-demo-title">"Resuma OS — live worker"</h3>
                <p class="exec-demo-lead">
                    "A real "
                    <code>"#[worker]"</code>
                    " runs on this server: durable graph, SSE event stream, pause/resume/cancel. "
                    "No Redis — self-hosted execution in the same binary as your app."
                </p>
            </div>
            <div class="exec-demo-grid">
                <div class="exec-demo-controls">
                    <label class="exec-demo-label" for="exec-topic">
                        "Topic"
                        <input id="exec-topic" type="text" name="exec_topic" value="Resuma OS" />
                    </label>
                    <label class="exec-demo-label" for="exec-blurb">
                        "Text to analyze"
                        <textarea id="exec-blurb" name="exec_blurb" rows="3">"Durable workers with checkpointed graphs, queue recovery, and an ops dashboard — all in Rust."</textarea>
                    </label>
                    <button
                        type="button"
                        class="btn btn-primary"
                        id="exec-start-btn"
                        onClick={js!(async (_event, _state, __resuma) => {
                            const topic = document.getElementById("exec-topic").value;
                            const blurb = document.getElementById("exec-blurb").value;
                            const errEl = document.getElementById("exec-err");
                            const slot = document.getElementById("exec-flow-slot");
                            const btn = document.getElementById("exec-start-btn");
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
                            const panel = document.createElement("div");
                            panel.className = "r-flow-exec";
                            panel.setAttribute("data-r-flow-execution", graphId);
                            panel.innerHTML =
                                "<div class=\"r-flow-exec__panel\">" +
                                "<h3>Execution graph</h3>" +
                                "<div class=\"r-flow-graph\" data-r-flow-graph=\"" + graphId + "\" data-r-flow-graph-live=\"true\" data-r-graph-token=\"" + token + "\">" +
                                "<div class=\"r-flow-graph__track\" data-r-flow-graph-track=\"true\">...</div>" +
                                "<p class=\"r-flow-graph__status\" data-r-flow-graph-status=\"true\">Loading graph...</p>" +
                                "</div></div>" +
                                "<aside class=\"r-flow-exec__side\">" +
                                "<div class=\"r-flow-exec__panel\"><h3>Controls</h3>" +
                                "<div class=\"r-worker-panel\" data-r-worker-panel=\"" + graphId + "\" data-r-graph-token=\"" + token + "\">" +
                                "<div class=\"r-worker-panel__actions\">" +
                                "<button type=\"button\" class=\"r-flow-control r-flow-control--ghost r-flow-control--pause\" data-r-worker-pause=\"true\">Pause</button>" +
                                "<button type=\"button\" class=\"r-flow-control r-flow-control--ghost r-flow-control--resume\" data-r-worker-resume=\"true\">Resume</button>" +
                                "<button type=\"button\" class=\"r-flow-control r-flow-control--danger\" data-r-worker-cancel=\"true\">Cancel</button>" +
                                "<button type=\"button\" class=\"r-flow-control r-flow-control--ghost r-flow-control--replay\" data-r-worker-replay=\"true\">Replay</button>" +
                                "</div>" +
                                "<p class=\"r-worker-panel__status\" data-r-worker-status aria-live=\"polite\"></p>" +
                                "</div></div>" +
                                "<div class=\"r-flow-exec__panel\"><h3>Event stream</h3>" +
                                "<div class=\"r-event-stream\" data-r-event-stream=\"" + graphId + "\" data-r-graph-token=\"" + token + "\">" +
                                "<div class=\"r-event-stream__viewport\" data-r-event-stream-viewport=\"true\">" +
                                "<ul class=\"r-event-stream-list\"></ul>" +
                                "</div></div></div></aside>";
                            slot.appendChild(panel);
                            slot.hidden = false;
                            flow.initFlowWidgets(slot, { flush: false });
                        })}
                    >
                        "Run worker"
                    </button>
                    <p id="exec-err" class="exec-demo-err" role="alert" hidden></p>
                    <p class="exec-demo-hint">
                        <a href="/docs/exec">"Resuma OS docs →"</a>
                    </p>
                </div>
                <div class="exec-demo-dash">
                    {flow_dashboard_poll(4000, Some(status))}
                </div>
            </div>
            <div id="exec-flow-slot" class="exec-flow-slot" hidden></div>
        </section>
    }
}

/// Public entry — resumable boundary so the large Run-worker handler loads from
/// `/_resuma/handler/ExecShowcaseDemo.js` instead of the shared `__page__` chunk.
pub fn exec_showcase_demo() -> View {
    view! { <ExecShowcaseDemo /> }
}
