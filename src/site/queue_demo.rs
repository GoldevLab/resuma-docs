//! Live disk queue demo — enqueue `docs_showcase` and read queue stats.

use resuma::exec::{enqueue, queue_stats, EnqueueResponse, QueueStats};
use resuma::prelude::*;
use serde_json::json;

#[server]
async fn docs_queue_enqueue() -> Result<EnqueueResponse> {
  enqueue(
        "docs",
        "docs_showcase",
        json!({
            "topic": "Queue demo",
            "blurb": "Enqueued from the docs queue page — claimed by the background consumer."
        }),
    )
    .await
}

#[server]
async fn docs_queue_stats() -> Result<QueueStats> {
    Ok(queue_stats("docs"))
}

/// Interactive queue panel for `/docs/exec/queue`.
#[component]
pub fn QueueDemoWidget() -> View {
    view! {
        <div class="queue-demo">
            <p class="demo-muted">
                "Calls "
                <code>"enqueue(\"docs\", \"docs_showcase\", …)"</code>
                " — same disk-backed queue store as production apps."
            </p>
            <div class="demo-row">
                <button type="button" class="btn btn-sm btn-primary" id="queue-enqueue-btn" onClick={js!(async () => {
                    const out = document.getElementById("queue-out");
                    out.textContent = "Enqueueing…";
                    const res = await __resuma.safeAction("docs_queue_enqueue", []);
                    if (!res.ok) { out.textContent = res.error; return; }
                    const v = res.value;
                    out.textContent = "Enqueued message " + v.message_id + " on queue \"" + v.queue + "\"";
                })}>"Enqueue worker job"</button>
                <button type="button" class="btn btn-sm" id="queue-stats-btn" onClick={js!(async () => {
                    const out = document.getElementById("queue-out");
                    out.textContent = "Reading stats…";
                    const res = await __resuma.safeAction("docs_queue_stats", []);
                    if (!res.ok) { out.textContent = res.error; return; }
                    const s = res.value;
                    out.textContent =
                        "docs queue - pending: " + s.pending +
                        ", processing: " + s.processing +
                        ", done: " + s.done +
                        ", failed: " + s.failed;
                })}>"Queue stats"</button>
            </div>
            <pre id="queue-out" class="demo-output queue-demo-out" aria-live="polite"></pre>
        </div>
    }
}
