use crate::site::code_block;
use resuma::prelude::*;

pub fn page(_req: FlowRequest) -> View {
    view! {
        <>
            <h1>"Theme"</h1>
            <p class="lead">"Provide theme tokens via context and expose CSS variables for consistent styling."</p>

            {crate::site::demos::cookbook_theme()}

            <h2>"Provide theme"</h2>
            {code_block(r##"#[layout("/")]
fn AppLayout() -> View {
    provide_theme(Theme {
        mode: "dark".into(),
        primary: "#6366f1".into(),
        background: "#0b1020".into(),
        foreground: "#e6e8ee".into(),
    });

    view! {
        <div class="app" style={theme_css_vars(&use_theme())}>
            <Slot />
        </div>
    }
}"##)}

            <h2>"Consume in components"</h2>
            {code_block(r#"#[component]
fn ThemedButton() -> View {
    let theme = use_theme();
    view! {
        <button style={format!("background: {}", theme.primary)}>
            "Click"
        </button>
    }
}"#)}

            <h2>"PWA colors from theme"</h2>
            {code_block(r##"FlowApp::new()
    .with_theme_pwa(Theme {
        primary: "#c9a962".into(),
        background: "#0a0908".into(),
        ..Default::default()
    })
    .auto_pages("src/pages", PagesRegistry)"##)}
            <p><a href="/docs/flow/pwa">"PWA & static files"</a>"."</p>

            <h2>"Toggle mode"</h2>
            {code_block(r#"let dark = signal(true);

view! {
    <button onClick={dark.update(|d| *d = !*d)}>
        "Toggle theme"
    </button>
}"#)}
        </>
    }
}
