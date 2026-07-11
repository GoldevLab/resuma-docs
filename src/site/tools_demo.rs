//! Live tools demo — dispatch built-in `echo` via the exec tool registry.

use resuma::exec::dispatch_tool;
use resuma::prelude::*;
use serde_json::Value;

#[server]
async fn docs_tool_echo(input: Value) -> Result<Value> {
    dispatch_tool("echo", input).await
}

/// Interactive tools panel for `/docs/exec/tools`.
#[component]
pub fn ToolsDemoWidget() -> View {
    view! {
        <div class="tools-demo">
            <p class="demo-muted">
                "Calls "
                <code>"dispatch_tool(\"echo\", …)"</code>
                " on the real registry — same path workers use with "
                <code>"ctx.tool(\"echo\", …)"</code>
                "."
            </p>
            <label class="exec-demo-label" for="tool-json">
                "JSON args"
                <textarea id="tool-json" name="tool_json" rows="3">{r#"{"message": "Hello from docs"}"#}</textarea>
            </label>
            <button
                type="button"
                class="btn btn-sm btn-primary"
                id="tool-echo-btn"
                onClick={js!(async () => {
                    const out = document.getElementById("tool-out");
                    const raw = document.getElementById("tool-json").value;
                    out.textContent = "Calling echo…";
                    let args;
                    try {
                        args = JSON.parse(raw);
                    } catch (e) {
                        out.textContent = "Invalid JSON: " + String(e);
                        return;
                    }
                    const res = await __resuma.safeAction("docs_tool_echo", [args]);
                    out.textContent = res.ok ? JSON.stringify(res.value, null, 2) : res.error;
                })}
            >
                "Call echo tool"
            </button>
            <pre id="tool-out" class="demo-output tools-demo-out" aria-live="polite"></pre>
        </div>
    }
}
