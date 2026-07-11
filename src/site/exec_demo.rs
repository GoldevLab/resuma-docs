//! Live Resuma OS worker demo — variants per docs page.

use resuma::prelude::*;
use resuma_flow::{flow_dashboard_poll, flow_styles_link};
use serde_json::{json, Value};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ExecDemoMode {
    /// `/docs/exec` — worker + ops dashboard.
    Overview,
    /// `/docs/exec/workers` — focus on `#[worker]` lifecycle (no ops dashboard).
    Workers,
}

#[server]
pub async fn start_docs_showcase(topic: String, blurb: String) -> Result<Value> {
    let started =
        resuma::exec::FlowEngine::start("docs_showcase", json!({ "topic": topic, "blurb": blurb }))
            .await?;
    Ok(json!({
        "graph_id": started.graph_id.0,
        "access_token": started.access_token.unwrap_or_default(),
    }))
}

/// Interactive worker + live execution graph (Resuma OS).
fn exec_showcase_demo_view(mode: ExecDemoMode) -> View {
    let status = resuma::exec::exec_status();
    let show_dashboard = mode == ExecDemoMode::Overview;
    let grid_class = if show_dashboard {
        "exec-demo-grid"
    } else {
        "exec-demo-grid exec-demo-grid--single"
    };
    let (title, lead) = match mode {
        ExecDemoMode::Overview => (
            "Resuma OS — live worker",
            view! {
                <p class="exec-demo-lead">
                    "A real "
                    <code>"#[worker]"</code>
                    " runs on this server: durable graph, SSE event stream, pause/resume/cancel. "
                    "No Redis — self-hosted execution in the same binary as your app."
                </p>
            },
        ),
        ExecDemoMode::Workers => (
            "Run docs_showcase worker",
            view! {
                <p class="exec-demo-lead">
                    "Starts a real "
                    <code>"#[worker]"</code>
                    " graph on this server — watch logs, progress, pause/resume/cancel, and the final result payload."
                </p>
            },
        ),
    };

    view! {
        <section class="exec-demo" aria-labelledby="exec-demo-title">
            {flow_styles_link()}
            <div class="exec-demo-intro">
                <h3 id="exec-demo-title">{title.to_string()}</h3>
                {lead}
            </div>
            <div class={grid_class.to_string()}>
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
                        onClick={js!(async () => {
                            const m = await import("/static/client/docs-flow-worker.js");
                            await m.runDocsFlowWorker("exec");
                        })}
                    >
                        "Run worker"
                    </button>
                    <p id="exec-err" class="exec-demo-err" role="alert" hidden></p>
                    <p class="exec-demo-hint">
                        <a href="/docs/exec">"Resuma OS docs →"</a>
                    </p>
                </div>
                {if show_dashboard {
                    view! {
                        <div class="exec-demo-dash">
                            {flow_dashboard_poll(4000, Some(status))}
                        </div>
                    }
                } else {
                    view! { <span hidden aria-hidden="true"></span> }
                }}
            </div>
            <div id="exec-flow-slot" class="exec-flow-slot" hidden></div>
        </section>
    }
}

fn exec_showcase_demo_mode(mode: ExecDemoMode) -> View {
    exec_showcase_demo_view(mode)
}

/// Overview — worker + ops dashboard.
pub fn exec_showcase_demo() -> View {
    exec_showcase_demo_mode(ExecDemoMode::Overview)
}

/// Workers page — worker lifecycle without ops dashboard.
pub fn exec_workers_demo() -> View {
    exec_showcase_demo_mode(ExecDemoMode::Workers)
}
