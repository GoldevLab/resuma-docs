use crate::site::code_block;
use resuma::prelude::*;

pub fn page(_req: FlowRequest) -> View {
    view! {
        <>
            <h1>"Disk queue"</h1>
            <p class="lead">
                "Durable job queue on local disk — atomic claim via rename, no Redis. "
                "Any Resuma process can enqueue; any process can claim pending jobs."
            </p>

            {crate::site::demos::exec_queue()}

            <h2>"Layout"</h2>
            <pre class="docs-pre">{r#".resuma/queue/{name}/
  pending/      ← new jobs
  processing/   ← claimed by one process
  done/         ← graph finished successfully
  failed/       ← start failed or graph failed/cancelled"#}</pre>

            <h2>"Enqueue from Rust"</h2>
            {code_block(r#"use resuma::prelude::*;

let resp = enqueue("default", "summarize", json!({ "text": "..." })).await?;
// resp.message_id, resp.queue"#)}

            <h2>"Enqueue from HTTP"</h2>
            {code_block(r#"POST /_resuma/queue/default
Authorization: Bearer $RESUMA_EXEC_API_KEY
Content-Type: application/json

{ "worker": "summarize", "input": { "text": "..." } }"#)}

            <h2>"Stats"</h2>
            {code_block(r#"GET /_resuma/queue/default/stats
    // { "pending": 2, "processing": 1, "done": 10, "failed": 0 }"#)}

            <h2>"Multi-process"</h2>
            <p>
                "Each process runs a background consumer when " <code>"init_exec()"</code> " runs "
                "(automatic on " <code>"FlowApp::serve"</code> "). Claim uses "
                <code>"rename(pending → processing)"</code> " — only one winner per job. "
                "Tune poll interval with " <code>"RESUMA_QUEUE_POLL_MS"</code> " (default 200)."
            </p>

            <p><a href="/docs/exec">"← Overview"</a> " · " <a href="/docs/exec/scheduler">"Scheduler →"</a></p>
        </>
    }
}
