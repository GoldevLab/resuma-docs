use crate::site::code_block;
use resuma::prelude::*;

pub fn page(_req: FlowRequest) -> View {
    view! {
        <>
            <h1>"js!"</h1>
            <p class="lead">"The js! macro embeds raw JavaScript for cases where rs2js translation is insufficient."</p>

            <h2>"Basic usage"</h2>
            {code_block(r#"let count = signal(0);

view! {
    <button onClick={ js! {
        state.count.update(c => c + 1);
    }}>
        "+"
    </button>
}"#)}

            <h2>"Server actions"</h2>
            {code_block(r#"view! {
    <button onClick={ js! {
        const result = await __resuma.action('greet', ['World']);
        state.message.set(result);
    }}>
        "Greet"
    </button>
}"#)}

            <h2>"SPA navigation"</h2>
            {code_block(r#"on:change={js! {
    const el = event.target;
    if (!(el instanceof HTMLInputElement)) return;
    await __resuma.navigate(__resuma.buildUrl("/book", { fecha: el.value }));
}}"#)}
            <p>
                "Always read form values from " <code>"event.target"</code> " in "
                <code>"js!"</code> " handlers — " <code>"event.currentTarget"</code> " is unreliable in async code."
            </p>

            <h2>"When to use js!"</h2>
            <ul>
                <li>"Async fetch patterns with " <code>"await __resuma.action(...)"</code></li>
                <li>"Query-driven loader refresh via " <code>"__resuma.navigate"</code></li>
                <li>"Browser APIs not expressible in Rust closures"</li>
                <li>"Complex client-side orchestration"</li>
            </ul>

            <h2>"Prefer rs2js when possible"</h2>
            <p>"Plain Rust closures in onClick are translated automatically and stay type-checked on the server side. Reach for js! only when you need full JS syntax."</p>
        </>
    }
}
