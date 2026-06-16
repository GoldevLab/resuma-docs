use crate::site::code_block;
use resuma::prelude::*;

pub fn page(_req: FlowRequest) -> View {
    view! {
        <>
            <h1>"Error boundaries"</h1>
            <p class="lead">
                "Handle failed loaders and server results in the UI — inspired by Leptos "
                <code>"&lt;ErrorBoundary&gt;"</code> ", adapted for Resuma's SSR-first model."
            </p>

            <h2>"Loader boundaries"</h2>
            <p>
                "Loaders run on the server before paint. Use " <code>"load_boundary"</code> " to branch on "
                <code>"LoadValue::Ok | Err | Pending"</code> ":"
            </p>
            {code_block(r#"load_boundary(
    use_user_load(),
    |user| view! {
        <h1>"Hello, " {user.name.clone()}</h1>
    },
    |err| view! {
        <div class="error-banner">
            "Could not load profile: " {err.message.clone()}
        </div>
    },
    || view! { <p aria-busy="true">"Loading profile…"</p> },
)"#)}

            <h2>"Result fallback"</h2>
            <p>"For synchronous " <code>"Result&lt;View, E&gt;"</code> " branches inside a component:"</p>
            {code_block(r#"let panel = match build_panel() {
    Ok(v) => v,
    Err(e) => view! { <p class="error">{e}</p> },
};

// Or with the helper:
error_boundary(build_panel(), |msg| view! {
    <p class="error">{msg}</p>
})"#)}

            <h2>"Server actions & client errors"</h2>
            <p>
                "Use " <code>"__resuma.safeAction(name, args)"</code> " in " <code>"js!"</code> " for Result-style "
                "handling without try/catch — like Leptos error boundaries for RPC:"
            </p>
            {code_block(r#"onClick={js!(async (_event, _state, __resuma) => {
    const res = await __resuma.safeAction("current_time", [input]);
    if (res.ok) {
        okEl.textContent = res.value;
    } else {
        errEl.textContent = res.error;
    }
})}"#)}

            <h2>"Progressive enhancement"</h2>
            <p>
                "Forms with " <code>"&lt;Form submit={…}&gt;"</code> " still work without JS. "
                "Field errors render via " <code>".resuma-field-error"</code> " when the runtime intercepts the POST."
            </p>
        </>
    }
}
