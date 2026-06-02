use crate::site::code_block;
use resuma::prelude::*;

pub fn page(_req: FlowRequest) -> View {
    view! {
        <>
            <h1>"NavLink"</h1>
            <p class="lead">"NavLink renders an anchor with active-state styling and client-side navigation when the Resuma runtime is loaded."</p>

            <h2>"Basic usage"</h2>
            {code_block(r#"view! {
    <nav>
        <NavLink href="/" activeClass="active">"Home"</NavLink>
        <NavLink href="/docs" activeClass="active">"Docs"</NavLink>
        <NavLink href="/about" activeClass="active">"About"</NavLink>
    </nav>
}"#)}

            <h2>"How matching works"</h2>
            <p>
                "Exact match wins. Prefix match applies when the current path starts with href followed by a slash — "
                <code>"/docs"</code> " is active on " <code>"/docs/getting_started"</code> "."
            </p>
            <p>
                "If " <code>"href"</code> " has no query string, the link stays active when the URL adds query params — "
                <code>"/reservar"</code> " stays active on " <code>"/reservar?fecha=…"</code> ". "
                "Links built with " <code>"query_nav_link"</code> " require an exact query match."
            </p>

            <h2>"Query in the URL"</h2>
            {code_block(r#"query_nav_link(
    "/servicios",
    &[("destacado", "1")],
    "active",
    "pill",
    vec![/* link label */],
)"#)}

            <h2>"Client navigation"</h2>
            <p>
                "NavLink sets " <code>"data-r-nav"</code> ". When JavaScript is enabled, clicks fetch the next page's SSR HTML, "
                "swap " <code>"#resuma-root"</code> " + the resumability payload, and update the URL with "
                <code>"history.pushState"</code> " — no full document reload."
            </p>
            <p>"Modifier clicks (Ctrl/Cmd, middle button) and " <code>"target=\"_blank\""</code> " fall back to normal browser navigation."</p>
            <p>
                "Programmatic navigation: " <code>"await __resuma.navigate(href)"</code> " and "
                <code>"__resuma.buildUrl(path, { key: value })"</code> " — see "
                <a href="/docs/flow/query_params">"Query params"</a> "."
            </p>

            <h2>"In layouts"</h2>
            {code_block(r#"#[layout("/docs")]
fn DocsLayout() -> View {
    view! {
        <aside>
            <NavLink href="/docs/getting_started" activeClass="active">
                "Getting Started"
            </NavLink>
        </aside>
        <main><Slot /></main>
    }
}"#)}
        </>
    }
}
