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

            <h2>"Built in (no setup)"</h2>
            <ul>
                <li><strong>"CSRF"</strong>" — " <code>"X-Resuma-CSRF"</code> " on actions and submits"</li>
                <li><strong>"Headers"</strong>" — CSP nonces, HSTS, X-Frame-Options, COOP, CORP"</li>
                <li><strong>"Rate limiting"</strong>" — per-IP on " <code>"/_resuma/action/*"</code></li>
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
            <p>"Per-IP sliding window in memory (resets on process restart). Defaults: 120 action RPC/min, 60 submits/min. "
                "Tune with " <code>"RESUMA_RATE_ACTIONS"</code> " / " <code>"RESUMA_RATE_SUBMITS"</code> ". "
                "For multi-instance deployments, add edge rate limiting (Fly, Cloudflare, nginx) in front of Resuma."</p>

            <h2>"Guides"</h2>
            <div class="template-grid">
                <a href="/docs/security/todo" class="template-pill" style="text-decoration: none;">
                    <strong>"Todo example"</strong>
                    <span>"Start here — full backend reference (main + security + todo_store)"</span>
                </a>
                <a href="/docs/security/configure" class="template-pill" style="text-decoration: none;">
                    <strong>"Configure server"</strong>
                    <span>"SecurityConfig · env vars · Fly/Docker"</span>
                </a>
                <a href="/docs/security/server_actions" class="template-pill" style="text-decoration: none;">
                    <strong>"Secure #[server] actions"</strong>
                    <span>"Validation · Result errors · action middleware"</span>
                </a>
                <a href="/docs/security/middleware" class="template-pill" style="text-decoration: none;">
                    <strong>"Auth middleware"</strong>
                    <span>"Flow #[middleware] vs ResumaApp action pipeline"</span>
                </a>
                <a href="/docs/security/backend_patterns" class="template-pill" style="text-decoration: none;">
                    <strong>"Backend patterns"</strong>
                    <span>"Pattern mapping table"</span>
                </a>
                <a href="/docs/security/authorization" class="template-pill" style="text-decoration: none;">
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
