use crate::site::code_block;
use resuma::prelude::*;

pub fn page(_req: FlowRequest) -> View {
    view! {
        <>
            <h1>"Tools & planner"</h1>
            <p class="lead">
                "Built-in tools callable from workers via " <code>"ctx.tool(...)"</code> ". "
                "The planner chooses execution strategy (single node vs map-reduce) from worker intent."
            </p>

            {crate::site::demos::exec_tools()}

            <h2>"Built-in tools"</h2>
            <table class="docs-table">
                <thead><tr><th>"Name"</th><th>"Purpose"</th></tr></thead>
                <tbody>
                    <tr><td><code>"echo"</code></td><td>"Passthrough args (testing)"</td></tr>
                    <tr><td><code>"fetch"</code></td><td>"HTTP GET/POST with SSRF protection"</td></tr>
                    <tr><td><code>"scrape"</code></td><td>"Search HTML stub (override in prod)"</td></tr>
                    <tr><td><code>"ai"</code></td><td>"OpenAI-compatible chat (" <code>"RESUMA_AI_*"</code> ")"</td></tr>
                </tbody>
            </table>

            <h2>"Call from a worker"</h2>
            {code_block(r#"#[worker(intent = "fetch and summarize with AI")]
async fn pipeline(input: Value, ctx: WorkerContext) -> Result<Value> {
    let page = ctx.tool("fetch", json!({ "url": input["url"] })).await?;
    let summary = ctx.tool("ai", json!({
        "prompt": "Summarize",
        "data": page
    })).await?;
    Ok(summary)
}"#)}

            <h2>"Register custom tools"</h2>
            {code_block(r#"register_tool("my_api", |args| {
    Box::pin(async move {
        // call your API
        Ok(json!({ "ok": true }))
    })
});"#)}

            <h2>"SSRF & fetch limits"</h2>
            <ul>
                <li><code>"RESUMA_FETCH_ALLOWLIST"</code> " — optional host allowlist"</li>
                <li><code>"RESUMA_FETCH_MAX_BYTES"</code> " — default 5 MB response cap"</li>
                <li>"Private IPs, metadata hosts, and redirects are blocked"</li>
            </ul>

            <h2>"Map-reduce parallelism"</h2>
            <p>
                "Heavy intents (e.g. containing \"AI\" with large item lists) trigger map-reduce. "
                "Concurrency is capped by " <code>"RESUMA_NODE_PARALLEL"</code> " (default 4) via a semaphore."
            </p>

            <p><a href="/docs/exec/webhooks">"← Webhooks"</a> " · " <a href="/docs/exec/flow_ui">"Flow UI →"</a></p>
        </>
    }
}
