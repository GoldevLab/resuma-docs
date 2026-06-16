//! Live server-function demo (Leptos promo pattern): RPC from browser, errors without reload.

use resuma::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};

static DEMO_CALLS: AtomicUsize = AtomicUsize::new(0);

#[server]
async fn demo_server_time(input: String) -> Result<String> {
    let n = DEMO_CALLS.fetch_add(1, Ordering::Relaxed) + 1;
    eprintln!("[demo_server_time] input={input:?} call=#{n}");
    if n.is_multiple_of(3) {
        return Err(ResumaError::Other(format!(
            "Simulated server error on call #{n} (every 3rd call fails)"
        )));
    }
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    Ok(format!(
        "Server unix time: {secs}s · call #{n} · you sent: \"{input}\""
    ))
}

/// Interactive widget: server function + error boundary pattern (see Leptos promo).
pub fn server_function_demo() -> View {
    view! {
        <section class="server-demo" aria-labelledby="server-demo-title">
            <h3 id="server-demo-title">"Server function demo"</h3>
            <p class="server-demo-lead">
                "Rust runs on the server only. Click reload — no page refresh. Every 3rd call returns an error "
                "(check your server logs)."
            </p>
            <div class="server-demo-row">
                <label class="server-demo-label" for="demo-input">
                    "Input logged on server"
                    <input id="demo-input" type="text" name="demo_input" value="hello from browser" />
                </label>
                <button
                    type="button"
                    class="btn btn-primary"
                    id="demo-fetch-btn"
                    onClick={js!(async (_event, _state, __resuma) => {
                        const input = document.getElementById("demo-input").value;
                        const okEl = document.getElementById("demo-ok");
                        const errEl = document.getElementById("demo-err");
                        okEl.hidden = true;
                        errEl.hidden = true;
                        const res = await __resuma.safeAction("demo_server_time", [input]);
                        if (res.ok) {
                            okEl.textContent = res.value;
                            okEl.hidden = false;
                        } else {
                            errEl.textContent = res.error;
                            errEl.hidden = false;
                        }
                    })}
                >
                    "Reload from server"
                </button>
            </div>
            <p id="demo-ok" class="server-demo-ok"></p>
            <p id="demo-err" class="server-demo-err" role="alert"></p>
        </section>
    }
}
