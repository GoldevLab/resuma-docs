use resuma::prelude::*;

pub fn page(_req: FlowRequest) -> View {
    view! {
        <>
            <h1>"SEO, GEO & Analytics"</h1>
            <p class="lead">"Production-grade metadata inspired by real apps (ACUPATAS): Open Graph, JSON-LD, Meta Pixel, and AI crawler policies."</p>

            <h2>"SeoKit"</h2>
            <pre style="background:#0b1020;padding:1rem;border-radius:8px;font-size:.85rem">{r#"use resuma::prelude::*;

let kit = SeoKit::new("My App", "https://example.com")
    .with_locale("es_VE")
    .with_keywords("rust, web, framework")
    .with_meta_pixel("1234567890")
    .with_default_json_ld()
    .with_llms_summary("My App helps teams ship resumable SSR apps in Rust.");

FlowApp::new()
    .with_seo_kit(kit)
    .auto_pages(...)"#}</pre>

            <h2>"What it includes"</h2>
            <ul>
                <li><strong>"SEO"</strong>" — canonical, OG/Twitter, robots meta, theme-color"</li>
                <li><strong>"GEO"</strong>" — " <code>"robots.txt"</code> " GPTBot/Claude rules + " <code>"llms.txt"</code> " generator"</li>
                <li><strong>"Analytics"</strong>" — Meta Pixel with SPA " <code>"PageView"</code> " on " <code>"resuma:navigate"</code></li>
                <li><strong>"JSON-LD"</strong>" — Organization, WebSite, WebPage builders"</li>
            </ul>

            <h2>"Serve robots.txt & llms.txt"</h2>
            <p>"With " <code>".with_seo_kit(kit)"</code> ", Resuma serves " <code>"/robots.txt"</code> " and " <code>"/llms.txt"</code> " automatically (GPTBot rules, sitemap hint, LLM summary). Override by registering your own page at those paths."</p>

            <h2>"Reactive Show"</h2>
            <p>"Use " <code>"<Show when={logged_in.get()}>"</code> " or " <code>"when={logged_in}"</code> " — both branches stay in the DOM and toggle on the client."</p>
        </>
    }
}
