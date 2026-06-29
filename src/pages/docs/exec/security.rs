use crate::site::code_block;
use resuma::prelude::*;

pub fn page(_req: FlowRequest) -> View {
    view! {
        <>
            <h1>"Exec security"</h1>
            <p class="lead">
                "Production hardening for " <code>"/_resuma/worker"</code> ", queues, graphs, scheduler, and metrics. "
                "Complements " <a href="/docs/security">"app security"</a> " (CSRF, CSP, actions)."
            </p>

            <h2>"Required in production"</h2>
            {code_block(r#"RESUMA_ENV=production
RESUMA_EXEC_API_KEY=<openssl rand -hex 32>   # admin routes
RESUMA_OPS_SESSION=<openssl rand -hex 32>    # /ops cookie value
RESUMA_DATA_DIR=/data/resuma
RESUMA_RATE_BACKEND=disk"#)}

            <h2>"Auth matrix"</h2>
            <table class="docs-table">
                <thead><tr><th>"Route"</th><th>"Auth"</th><th>"CSRF / origin"</th></tr></thead>
                <tbody>
                    <tr><td><code>"POST /_resuma/worker/*"</code></td><td>"API key"</td><td>"Yes"</td></tr>
                    <tr><td><code>"GET /_resuma/status"</code></td><td>"API key"</td><td>"No"</td></tr>
                    <tr><td><code>"GET /_resuma/graph/*/events"</code></td><td>"Graph token or API key"</td><td>"Reads: origin in dev"</td></tr>
                    <tr><td><code>"POST .../pause|resume|cancel"</code></td><td>"Graph token or API key"</td><td>"Always"</td></tr>
                    <tr><td><code>"exec_status" action</code></td><td>"Session or API key"</td><td>"Action pipeline CSRF"</td></tr>
                </tbody>
            </table>

            <h2>"Graph tokens"</h2>
            <p>
                "Each started graph returns " <code>"access_token"</code> " in "
                <code>"StartWorkerResponse"</code> ". Pass to Flow UI components for SSE and controls. "
                "Tokens are persisted under " <code>".resuma/durable/tokens/"</code> " and validated in constant time."
            </p>

            <h2>"Rate limits (exec)"</h2>
            <ul>
                <li><code>"RESUMA_RATE_EXEC_WORKERS"</code> " — default 30/min (worker + queue POST)"</li>
                <li><code>"RESUMA_RATE_EXEC_GRAPH"</code> " — default 180/min (reads, SSE)"</li>
                <li><code>"RESUMA_RATE_EXEC_CONTROL"</code> " — default 60/min (pause/resume/cancel)"</li>
            </ul>

            <h2>"Input validation"</h2>
            <ul>
                <li><code>"RESUMA_EXEC_MAX_INPUT"</code> " — JSON body size (default 512 KiB)"</li>
                <li><code>"RESUMA_EXEC_MAX_DEPTH"</code> " — nesting depth (default 32)"</li>
            </ul>

            <p><a href="/docs/exec/flow_ui">"← Flow UI"</a> " · " <a href="/docs/security/configure">"Server security →"</a></p>
        </>
    }
}
