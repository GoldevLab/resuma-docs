use crate::site::code_block;
use resuma::prelude::*;

pub fn page(_req: FlowRequest) -> View {
    view! {
        <>
            <h1>"Control flow & iteration"</h1>
            <p class="lead">
                "Resuma uses plain Rust inside " <code>"view!"</code> " — no separate "
                <code>"&lt;Show&gt;"</code> " / " <code>"&lt;For&gt;"</code> " components. "
                "That keeps templates predictable and fully type-checked."
            </p>

            <h2>"Conditional UI"</h2>
            <p>"Use Rust " <code>"if"</code> " / " <code>"match"</code> ", or the " <code>"&lt;Show&gt;"</code> " helper (Leptos-style):"</p>
            {code_block(r#"let logged_in = use_signal(false);

view! {
    <>
        <Show when={logged_in.get()}>
            <p>"Welcome back!"</p>
        </Show>
        <Show when={!logged_in.get()} fallback={view! { <a href="/login">"Sign in"</a> }}>
            <span></span>
        </Show>
    </>
}"#)}

            <h2>"Match on enums"</h2>
            {code_block(r#"enum Tab { Docs, Examples }

let tab = use_signal(Tab::Docs);

view! {
    <nav>
        {match tab.get() {
            Tab::Docs => view! { <span class="active">"Docs"</span> },
            Tab::Examples => view! { <span class="active">"Examples"</span> },
        }}
    </nav>
}"#)}

            <h2>"Lists and iteration"</h2>
            {code_block(r#"let items = use_signal(vec!["Rust", "Resuma", "Flow"]);

view! {
    <ul>
        {items.get().iter().map(|label| {
            view! { <li>{label.to_string()}</li> }
        }).collect::<Vec<_>>()}
    </ul>
}"#)}

            <h2>"Loaders and boundaries"</h2>
            <p>
                "For async data, prefer " <code>"load_boundary"</code> " over manual "
                <code>"match use_*_load()"</code> " when you want explicit pending/error UI:"
            </p>
            {code_block(r#"load_boundary(
    use_items_load(),
    |items| view! { <ul>{/* render items */}</ul> },
    |err| view! { <p class="error">{err.message.clone()}</p> },
    || view! { <p>"Loading…"</p> },
)"#)}

            <h2>"vs Leptos"</h2>
            <p>
                "Leptos provides " <code>"&lt;Show&gt;"</code> ", " <code>"&lt;For&gt;"</code> ", and "
                <code>"&lt;Suspense&gt;"</code> ". Resuma maps these to Rust control flow plus "
                <a href="/docs/flow/streaming">"streaming loaders"</a> " and "
                <a href="/docs/components/error_boundary">"load boundaries"</a> "."
            </p>
        </>
    }
}
