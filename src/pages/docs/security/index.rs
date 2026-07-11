use crate::site::code_block;
use resuma::prelude::*;

pub fn page(_req: FlowRequest) -> View {
    view! {
        <>
            <h1>"Security"</h1>
            <p class="lead">
                "Production-grade defaults built in. Harden your app using "
                <a href="/docs/security/todo">"examples/todo"</a> " as the reference implementation."
            </p>

            {crate::site::demos::security_overview()}

            <h2>"Built in (no setup)"</h2>
            <ul>
                <li><strong>"CSRF"</strong>" — " <code>"X-Resuma-CSRF"</code> " on actions and submits"</li>
                <li><strong>"Headers"</strong>" — CSP nonces, HSTS, X-Frame-Options, COOP, CORP"</li>
                <li><strong>"Rate limiting"</strong>" — per-IP on actions, submits, and exec routes; disk-backed in prod (" <code>"RESUMA_RATE_BACKEND=disk"</code> ", no Redis)"</li>
                <li><strong>"Origin check"</strong>" — blocks cross-origin POST abuse"</li>
                <li><strong>"SSR safety"</strong>" — escaped HTML + sanitized JSON state + JSON-LD"</li>
                <li><strong>"Client bundles"</strong>" — " <code>"script-src 'self'"</code> "; TypeScript via " <code>"ClientComponent"</code></li>
            </ul>

            <h2>"Trust boundaries"</h2>
            <table class="docs-table">
                <thead>
                    <tr><th>"API"</th><th>"Escaping"</th><th>"Notes"</th></tr>
                </thead>
                <tbody>
                    <tr><td><code>"view!"</code>" text/attrs"</td><td>"Auto"</td><td>"Default for UI"</td></tr>
                    <tr><td><code>"View::raw()"</code></td><td>"None"</td><td>"Trusted static HTML only"</td></tr>
                    <tr><td><code>"with_head()"</code></td><td>"Partial"</td><td>"Inline " <code>"&lt;style&gt;"</code> " / " <code>"&lt;script&gt;"</code> " get CSP nonces; external " <code>"src"</code> " scripts use " <code>"'self'"</code></td></tr>
                    <tr><td><code>"ClientComponent"</code></td><td>"Attrs escaped"</td><td>"Ids restricted to " <code>"[a-zA-Z0-9_-]"</code></td></tr>
                    <tr><td>"Resumability payload"</td><td>"Sanitized"</td><td>"Blocks " <code>"&lt;/script&gt;"</code> " breakouts"</td></tr>
                </tbody>
            </table>

            <h2>"Rate limiting"</h2>
            <p>
                "Per-IP sliding window on actions, submits, and exec routes. "
                "Defaults: 120 action RPC/min, 60 submits/min — tune with "
                <code>"RESUMA_RATE_ACTIONS"</code> " and " <code>"RESUMA_RATE_SUBMITS"</code>". "
                <strong>"No Redis or external datastore"</strong> " — Resuma ships built-in backends:"
            </p>
            <table class="docs-table">
                <thead>
                    <tr><th>"Backend"</th><th>"When"</th><th>"Storage"</th></tr>
                </thead>
                <tbody>
                    <tr>
                        <td><code>"memory"</code></td>
                        <td>"Local dev (default)"</td>
                        <td>"In-process HashMap"</td>
                    </tr>
                    <tr>
                        <td><code>"disk"</code></td>
                        <td><code>"RESUMA_ENV=production"</code> " (auto)"</td>
                        <td><code>"{RESUMA_DATA_DIR}/rate-limit/"</code> " — file locks, multi-process on same volume"</td>
                    </tr>
                </tbody>
            </table>
            <p>
                "Set " <code>"RESUMA_RATE_BACKEND=memory|disk"</code> " to override. "
                "Mount a persistent volume at " <code>"RESUMA_DATA_DIR"</code> " on Fly/Docker so limits survive restarts. "
                "For multi-region or many machines without a shared volume, add edge rate limiting (nginx "
                <code>"limit_req"</code> ", Fly proxy, Cloudflare) — still no Redis in your app. "
                "Exec routes have separate limits — " <a href="/docs/exec/security">"Exec security"</a>"."
            </p>

            <h2>"Guides"</h2>
            <div class="template-grid">
                <a href="/docs/security/todo" class="template-pill template-pill--link">
                    <strong>"Todo example"</strong>
                    <span>"Start here — full backend reference (main + security + todo_store)"</span>
                </a>
                <a href="/docs/exec/security" class="template-pill template-pill--link">
                    <strong>"Exec security"</strong>
                    <span>"API keys, graph tokens, worker rate limits"</span>
                </a>
                <a href="/docs/security/environment" class="template-pill template-pill--link">
                    <strong>"Environment variables"</strong>
                    <span>"Local vs prod · RESUMA_ENV · Fly secrets · resuma doctor"</span>
                </a>
                <a href="/docs/security/configure" class="template-pill template-pill--link">
                    <strong>"Configure server"</strong>
                    <span>"SecurityConfig in Rust · CSP · Fly/Docker checklist"</span>
                </a>
                <a href="/docs/security/server_actions" class="template-pill template-pill--link">
                    <strong>"Secure #[server] actions"</strong>
                    <span>"Validation · Result errors · action middleware"</span>
                </a>
                <a href="/docs/security/middleware" class="template-pill template-pill--link">
                    <strong>"Auth middleware"</strong>
                    <span>"Flow #[middleware] vs ResumaApp action pipeline"</span>
                </a>
                <a href="/docs/security/backend_patterns" class="template-pill template-pill--link">
                    <strong>"Backend patterns"</strong>
                    <span>"Pattern mapping table"</span>
                </a>
                <a href="/docs/security/authorization" class="template-pill template-pill--link">
                    <strong>"Authorization & RLS"</strong>
                    <span>"Row-level checks · Postgres RLS"</span>
                </a>
            </div>

            <h2>"Quick start (ResumaApp)"</h2>
            {code_block(r#"mod security;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    security::install();
    ResumaApp::new()
        .page("/", || Home::render(HomeProps::default()))
        .serve(security::serve_options())
        .await
}"#)}
        </>
    }
}
