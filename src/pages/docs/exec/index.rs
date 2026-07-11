use crate::site::code_block;
use resuma::prelude::*;

pub fn page(_req: FlowRequest) -> View {
    view! {
        <>
            <h1>"Resuma OS (execution layer)"</h1>
            <p class="lead">
                "Durable workers, disk-backed queues, cron scheduler, webhooks, and an ops dashboard — "
                "self-hosted. No Redis or Cloudflare Workers required. "
                "Rate limiting for the whole app (actions, submits, exec) also lives on disk under "
                <code>"{RESUMA_DATA_DIR}/rate-limit/"</code> "."
            </p>

            {crate::site::demos::exec_overview()}

            <h2>"When to use"</h2>
            <ul>
                <li>"Background jobs (enqueue + multi-process consumers)"</li>
                <li>"Cron-triggered workers"</li>
                <li>"AI / fetch pipelines with map-reduce"</li>
                <li>"Ops visibility (graphs, queues, Prometheus metrics)"</li>
            </ul>

            <h2>"On-disk layout"</h2>
            <pre class="docs-pre">{r#".resuma/
├── durable/     graphs, events, checkpoints, tokens
├── queue/       pending → processing → done|failed
├── scheduler/   cron jobs (multi-process safe)
├── rate-limit/  per-IP limits on disk
└── webhooks/    outbound lifecycle hooks"#}</pre>

            <h2>"Bootstrap"</h2>
            <p>
                <code>"init_exec()"</code> " runs automatically when you "
                <code>"FlowApp::serve"</code> " or " <code>"ResumaApp::into_router"</code> ". "
                "Configure data dir with " <code>"RESUMA_DATA_DIR"</code> "."
            </p>
            {code_block(r#"use resuma::prelude::*;

#[worker(intent = "summarize text with AI")]
async fn summarize(input: SummarizeInput) -> Result<Value> {
    Ok(json!({ "summary": input.text }))
}

FlowApp::new()
    .auto_pages("src/pages", PagesRegistry)
    .serve(FlowServeOptions::default())  // init_exec() inside
    .await"#)}

            <h2>"HTTP routes"</h2>
            <table class="docs-table">
                <thead>
                    <tr><th>"Route"</th><th>"Purpose"</th></tr>
                </thead>
                <tbody>
                    <tr><td><code>"POST /_resuma/worker/{name}"</code></td><td>"Start worker graph"</td></tr>
                    <tr><td><code>"POST /_resuma/queue/{name}"</code></td><td>"Enqueue job"</td></tr>
                    <tr><td><code>"GET /_resuma/queue/{name}/stats"</code></td><td>"Queue depth"</td></tr>
                    <tr><td><code>"GET /_resuma/graph/{id}"</code></td><td>"Graph snapshot"</td></tr>
                    <tr><td><code>"GET /_resuma/graph/{id}/events"</code></td><td>"SSE live events"</td></tr>
                    <tr><td><code>"POST .../pause|resume|cancel"</code></td><td>"Graph control"</td></tr>
                    <tr><td><code>"GET /_resuma/status"</code></td><td>"Ops snapshot (API key)"</td></tr>
                    <tr><td><code>"GET /_resuma/metrics"</code></td><td>"Prometheus text"</td></tr>
                    <tr><td><code>"GET|POST /_resuma/scheduler"</code></td><td>"Cron jobs"</td></tr>
                    <tr><td><code>"GET|POST /_resuma/webhooks"</code></td><td>"Webhook targets"</td></tr>
                </tbody>
            </table>

            <h2>"Guides"</h2>
            <div class="template-grid">
                <a href="/docs/exec/workers" class="template-pill template-pill--link">
                    <strong>"Workers"</strong>
                    <span>"#[worker], lifecycle, WorkerContext"</span>
                </a>
                <a href="/docs/exec/queue" class="template-pill template-pill--link">
                    <strong>"Queue"</strong>
                    <span>"Enqueue, claim, multi-process"</span>
                </a>
                <a href="/docs/exec/scheduler" class="template-pill template-pill--link">
                    <strong>"Scheduler"</strong>
                    <span>"Cron jobs on disk"</span>
                </a>
                <a href="/docs/exec/webhooks" class="template-pill template-pill--link">
                    <strong>"Webhooks"</strong>
                    <span>"graph.done / failed / paused"</span>
                </a>
                <a href="/docs/exec/tools" class="template-pill template-pill--link">
                    <strong>"Tools & planner"</strong>
                    <span>"fetch, ai, map-reduce"</span>
                </a>
                <a href="/docs/exec/flow_ui" class="template-pill template-pill--link">
                    <strong>"Flow UI"</strong>
                    <span>"resuma-flow dashboard widgets"</span>
                </a>
                <a href="/docs/exec/ops" class="template-pill template-pill--link">
                    <strong>"Ops & deploy"</strong>
                    <span>"Production template, env vars"</span>
                </a>
                <a href="/docs/exec/security" class="template-pill template-pill--link">
                    <strong>"Exec security"</strong>
                    <span>"API keys, tokens, rate limits"</span>
                </a>
            </div>
        </>
    }
}
