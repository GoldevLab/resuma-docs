use crate::site::code_block;
use resuma::prelude::*;

pub fn page(_req: FlowRequest) -> View {
    view! {
        <>
            <h1>"Webhooks"</h1>
            <p class="lead">
                "Outbound HTTP callbacks when graphs complete, fail, or pause. "
                "HMAC-signed payloads for verification."
            </p>

            {crate::site::demos::exec_webhooks()}

            <h2>"Events"</h2>
            <ul>
                <li><code>"graph.done"</code> " — successful completion"</li>
                <li><code>"graph.failed"</code> " — worker error or operator cancel"</li>
                <li><code>"graph.paused"</code> " — cooperative pause"</li>
            </ul>

            <h2>"Register via env (boot)"</h2>
            {code_block(r#"RESUMA_WEBHOOK_URLS=https://hooks.example.com/resuma,https://backup.example/hooks
RESUMA_WEBHOOK_SECRET=<openssl rand -hex 32>"#)}

            <h2>"Register via HTTP"</h2>
            {code_block(r#"POST /_resuma/webhooks
Authorization: Bearer $RESUMA_EXEC_API_KEY

{ "url": "https://hooks.example.com/resuma", "events": ["graph.done", "graph.failed"] }"#)}

            <h2>"Payload & signature"</h2>
            {code_block(r#"POST https://your-endpoint
Content-Type: application/json
X-Resuma-Signature: sha256=<hmac-sha256-hex>

{
  "event": "graph.done",
  "graph_id": "g_...",
  "worker": "summarize",
  "status": "done",
  "timestamp_ms": 1710000000000,
  "duration_ms": 1234,
  "result": { ... }
}"#)}
            <p>
                "Verify with " <code>"RESUMA_WEBHOOK_SECRET"</code> ". Outbound URLs are SSRF-checked; "
                "HTTP redirects are disabled."
            </p>

            <p><a href="/docs/exec/scheduler">"← Scheduler"</a> " · " <a href="/docs/exec/tools">"Tools →"</a></p>
        </>
    }
}
