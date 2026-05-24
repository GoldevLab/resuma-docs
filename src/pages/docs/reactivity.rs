use crate::site::code_block;
use resuma::prelude::*;

pub fn page(_req: FlowRequest) -> View {
    view! {
        <>
            <h1>"Reactivity internals"</h1>
            <p class="lead">
                "How Resuma differs from hydration frameworks: components run once on the server; "
                "the browser resumes signals and lazy handler chunks."
            </p>

            <h2>"Lifecycle overview"</h2>
            <ol>
                <li><strong>"SSR render"</strong> " — " <code>"#[component]"</code> " functions build a " <code>"View"</code> " tree."</li>
                <li><strong>"Capture"</strong> " — signals, handlers, contexts, and effects are recorded in " <code>"RenderContext"</code> "."</li>
                <li><strong>"Serialize"</strong> " — " <code>"ResumePayload"</code> " is embedded in " <code>"&lt;script type=\"resuma/state\"&gt;"</code> "."</li>
                <li><strong>"Resume"</strong> " — runtime reconstructs signal cells and binds " <code>"data-r-bind:*"</code> "."</li>
                <li><strong>"Interact"</strong> " — first click fetches " <code>"/_resuma/handler/{Component}.js"</code> " and runs the handler."</li>
            </ol>

            <h2>"Signal lifecycle"</h2>
            {code_block(r#"// Server: allocate id + initial value
let n = use_signal(0);

// Serialized: { id: "s1", value: 0 }
// Client: SignalCell with .set/.update + subscribers on DOM nodes

// Handler closure captures signal id — not the Rust Signal handle
onClick={move |_| n.update(|v| *v += 1)}"#)}

            <h2>"Effects"</h2>
            <ul>
                <li><code>"use_effect"</code> " / " <code>"use_computed"</code> " — SSR dependency tracking only."</li>
                <li><code>"effect!"</code> " / " <code>"computed!"</code> " — rs2js translates to client-replayable JS."</li>
                <li><code>"debounce!"</code> " — debounced client effect registration."</li>
            </ul>

            <h2>"No VDOM diff"</h2>
            <p>
                "Like Leptos/Solid, updates target DOM nodes bound to signals. Unlike Leptos CSR/WASM, "
                "Resuma never re-runs the component tree in the browser — handlers patch the DOM directly."
            </p>

            <h2>"Further reading"</h2>
            <ul>
                <li><a href="/docs/architecture">"Architecture"</a></li>
                <li><a href="/docs/components/signals">"Signals"</a></li>
                <li><a href="/docs/components/effects">"Effects"</a></li>
                <li><a href="/docs/components/handlers">"Handlers"</a></li>
            </ul>
        </>
    }
}
