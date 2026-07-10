use crate::site::code_block;
use resuma::prelude::*;

pub fn page(_req: FlowRequest) -> View {
    view! {
        <>
            <h1>"Workers"</h1>
            <p class="lead">
                "Workers are async Rust functions registered at compile time via "
                <code>"#[worker]"</code> ". Each run becomes an execution graph with durable checkpoints."
            </p>

            {crate::site::demos::exec_workers()}

            <h2>"Define a worker"</h2>
            {code_block(r#"use resuma::prelude::*;
use serde::Deserialize;

#[derive(Deserialize)]
struct EnrichInput {
    url: String,
}

#[worker(intent = "fetch URL and enrich with AI")]
async fn enrich(input: EnrichInput, ctx: WorkerContext) -> Result<Value> {
    ctx.log("fetching");
    let page = ctx.tool("fetch", json!({ "url": input.url })).await?;
    ctx.progress(50);
    let summary = ctx.tool("ai", json!({
        "prompt": "Extract key facts",
        "data": page
    })).await?;
    Ok(summary)
}"#)}

            <h2>"WorkerContext API"</h2>
            <table class="docs-table">
                <thead><tr><th>"Method"</th><th>"Purpose"</th></tr></thead>
                <tbody>
                    <tr><td><code>"ctx.log(msg)"</code></td><td>"Emit log event (SSE + replay)"</td></tr>
                    <tr><td><code>"ctx.progress(n)"</code></td><td>"0–100 progress hint"</td></tr>
                    <tr><td><code>"ctx.tool(name, args)"</code></td><td>"Call registered tool"</td></tr>
                    <tr><td><code>"ctx.state_set(k, v)"</code></td><td>"Checkpoint key/value"</td></tr>
                    <tr><td><code>"ctx.check_cancelled()"</code></td><td>"Cooperative pause/cancel"</td></tr>
                </tbody>
            </table>

            <h2>"Manual registration"</h2>
            {code_block(r#"WorkerRegistry::new()
    .register(
        "my_worker",
        WorkerMeta {
            intent: "process items".into(),
            resources: Resources::auto(),
        },
        |input, ctx| Box::pin(async move { Ok(input) }),
    )
    .install();"#)}

            <h2>"Start a graph"</h2>
            {code_block(r#"let started = FlowEngine::start("enrich", json!({ "url": "https://example.com" })).await?;
// started.graph_id, started.access_token, started.plan"#)}

            <h2>"HTTP & queue"</h2>
            {code_block(r#"POST /_resuma/worker/enrich
Authorization: Bearer $RESUMA_EXEC_API_KEY
{ "input": { "url": "https://example.com" } }

POST /_resuma/queue/default
{ "worker": "enrich", "input": { "url": "..." } }"#)}

            <h2>"Graph lifecycle"</h2>
            <table class="docs-table">
                <thead><tr><th>"Status"</th><th>"Meaning"</th></tr></thead>
                <tbody>
                    <tr><td><code>"running"</code></td><td>"Worker executing"</td></tr>
                    <tr><td><code>"paused"</code></td><td>"Cooperative stop — resumable"</td></tr>
                    <tr><td><code>"done"</code></td><td>"Success"</td></tr>
                    <tr><td><code>"failed"</code></td><td>"Error or operator cancel"</td></tr>
                </tbody>
            </table>
            <ul>
                <li><code>"POST /_resuma/graph/{id}/pause"</code> " — pause (resumable)"</li>
                <li><code>"POST /_resuma/graph/{id}/resume"</code> " — continue from checkpoint"</li>
                <li><code>"POST /_resuma/graph/{id}/cancel"</code> " — permanent abort"</li>
            </ul>

            <p>
                <a href="/docs/exec">"← Overview"</a> " · "
                <a href="/docs/exec/queue">"Queue →"</a> " · "
                <a href="/docs/exec/flow_ui">"Flow UI →"</a>
            </p>
        </>
    }
}
