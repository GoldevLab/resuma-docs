use crate::site::code_block;
use resuma::prelude::*;

pub fn page(_req: FlowRequest) -> View {
    view! {
        <>
            <h1>"Upgrading to 1.0"</h1>
            <p class="lead">
                "Resuma 1.0 is the stable release — semver applies to the public API. "
                "If you were on 0.4.x, bump dependencies and adjust a few patterns below."
            </p>

            <h2>"1. Bump dependencies"</h2>
            {code_block(r#"[dependencies]
resuma = "1.0"

# CLI (optional):
# cargo install resuma --force"#)}

            <h2>"2. js! async handlers"</h2>
            <p>
                "Since 1.0.1, " <code>"js!{ async (...) => { ... } }"</code> " is emitted verbatim. "
                "Use the three-argument signature the runtime expects:"
            </p>
            {code_block(r#"onClick={js!(async (_event, _state, __resuma) => {
    const res = await __resuma.safeAction("save", [draft]);
    if (res.ok) state.status.set("Saved");
    else state.error.set(res.error);
})}"#)}
            <p><a href="/docs/components/js">"js! guide →"</a></p>

            <h2>"3. New in 1.0"</h2>
            <ul>
                <li><code>"#[derive(Store)]"</code> " — typed field accessors on stores (" <a href="/docs/components/store">"Store"</a>")"</li>
                <li><code>"&lt;For each={signal} let:item&gt;"</code> " — keyed reactive lists (" <a href="/docs/components/control_flow">"Control flow"</a>")"</li>
                <li><code>"&lt;Match&gt;"</code> " / " <code>"&lt;When&gt;"</code> " — reactive branching"</li>
                <li><code>"Path&lt;T&gt;"</code> " / " <code>"Query&lt;T&gt;"</code> " — typed extractors for " <code>"#[load]"</code> " / " <code>"#[submit]"</code></li>
                <li>"Loader invalidation — " <code>"invalidate_href"</code> ", " <code>"__resuma.invalidate()"</code> " (" <a href="/docs/cookbook/loader_invalidation">"cookbook"</a>")"</li>
                <li><code>"NavLink"</code> " hover prefetch for SPA navigation"</li>
                <li><code>"resuma new --template production"</code> " — Dockerfile + fly.toml + security stub"</li>
                <li><code>"resuma build --static-export"</code> " — HTML export from " <code>"src/pages/"</code></li>
            </ul>

            <h2>"4. CLI renames"</h2>
            <ul>
                <li><code>"resuma build --static-export"</code> " (not " <code>"--static"</code>")"</li>
                <li><code>"FlowApp::auto_pages(...)"</code> " (not " <code>"pages_from_dir"</code>")"</li>
            </ul>

            <h2>"5. Client components & CSP"</h2>
            <p>
                "Production CSP uses per-request nonces. " <code>"ClientComponent"</code> " module scripts include the nonce automatically (1.0.2+). "
                <a href="/docs/components/client">"Client components →"</a>
            </p>

            <h2>"Stability policy"</h2>
            <p>
                "MSRV Rust 1.91+, runtime gzip budgets (loader ≤ 1 KiB, core ≤ 5.7 KiB), semver for public API. "
                "Full policy: "
                <a href="https://github.com/GoldevLab/resuma/blob/main/docs/STABILITY.md" target="_blank" rel="noopener">"STABILITY.md"</a> " on GitHub."
            </p>

            <h2>"From other frameworks"</h2>
            <ul>
                <li><a href="https://github.com/GoldevLab/resuma/blob/main/docs/MIGRATION_LEPTOS.md" target="_blank" rel="noopener">"Leptos → Resuma"</a></li>
                <li><a href="https://github.com/GoldevLab/resuma/blob/main/docs/MIGRATION_QWIK.md" target="_blank" rel="noopener">"Qwik → Resuma"</a></li>
            </ul>
        </>
    }
}
