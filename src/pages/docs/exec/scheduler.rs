use crate::site::code_block;
use resuma::prelude::*;

pub fn page(_req: FlowRequest) -> View {
    view! {
        <>
            <h1>"Cron scheduler"</h1>
            <p class="lead">
                "Disk-backed cron jobs that enqueue workers on a schedule. "
                "No external cron daemon — Resuma ticks every "
                <code>"RESUMA_SCHEDULER_TICK_SECS"</code> " (default 30s)."
            </p>

            <h2>"Create a schedule"</h2>
            {code_block(r#"POST /_resuma/scheduler
Authorization: Bearer $RESUMA_EXEC_API_KEY

{
  "name": "nightly-enrich",
  "cron": "0 2 * * *",
  "worker": "enrich",
  "queue": "default",
  "input": { "url": "https://example.com" }
}"#)}

            <h2>"Cron expressions"</h2>
            <ul>
                <li><code>"@hourly"</code> ", " <code>"@daily"</code> ", " <code>"@weekly"</code> " — presets"</li>
                <li><code>"0 */6 * * *"</code> " — standard 5-field cron"</li>
            </ul>

            <h2>"HTTP API"</h2>
            <table class="docs-table">
                <thead><tr><th>"Method"</th><th>"Path"</th><th>"Purpose"</th></tr></thead>
                <tbody>
                    <tr><td><code>"GET"</code></td><td><code>"/_resuma/scheduler"</code></td><td>"List jobs"</td></tr>
                    <tr><td><code>"POST"</code></td><td><code>"/_resuma/scheduler"</code></td><td>"Create job"</td></tr>
                    <tr><td><code>"DELETE"</code></td><td><code>"/_resuma/scheduler/{id}"</code></td><td>"Remove job"</td></tr>
                    <tr><td><code>"POST"</code></td><td><code>"/_resuma/scheduler/tick"</code></td><td>"Fire due jobs once (admin)"</td></tr>
                </tbody>
            </table>

            <h2>"Multi-process safety"</h2>
            <p>
                "Due jobs are claimed with an atomic rename into "
                <code>"scheduler/jobs/firing/"</code> " before enqueue — only one process fires each tick. "
                "Stuck claims are recovered on scheduler start."
            </p>

            <p><a href="/docs/exec/queue">"← Queue"</a> " · " <a href="/docs/exec/webhooks">"Webhooks →"</a></p>
        </>
    }
}
