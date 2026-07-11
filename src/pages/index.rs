use resuma::prelude::*;

use crate::site::{
    bench_row_full, code_block, compare_column, doc_link_card, feature_card, hero_particles_mount,
    metric_item, payload_layer, pillar_card, pipeline_step, speed_bar,
};

pub fn page(_req: FlowRequest) -> View {
    let dash = "—";

    view! {
        <main id="main-content" class="landing">
            <div class="hero-wrap">
                {hero_particles_mount()}
                <section class="hero">
                    <div>
                        <span class="hero-badge">
                            <span class="hero-badge-dot"></span>
                            "v1.2.0 · Rust web framework"
                        </span>
                        <h1>
                            "The "
                            <span class="accent">"lightest"</span>
                            " path to instant interactivity in Rust"
                        </h1>
                        <p class="hero-tagline">"907 bytes to resume. Zero bytes on static pages."</p>
                        <p class="hero-lead">
                            "Resuma renders your UI once on the server, serialises signal state into HTML, "
                            "and lazy-loads only the handlers you touch. No WASM hydration. No component re-execution. "
                            "Full-stack Flow, server actions, and Resuma OS — one crate."
                        </p>
                        <div class="hero-actions">
                            <a href="/docs/getting_started" class="btn btn-primary">"Get Started"</a>
                            <a href="/docs/benchmark" class="btn btn-ghost">"See the numbers"</a>
                            <a href="/docs" class="btn btn-ghost">"Docs with live demos"</a>
                        </div>
                        <p class="hero-note">
                            <code>"cargo install resuma"</code>
                            " · "
                            <code>"resuma new my-app --template todo"</code>
                            " · no Node.js for app development"
                        </p>
                    </div>
                    <div class="hero-panel">
                        <div class="hero-panel-top">
                            <div class="hero-panel-dots">
                                <span></span><span></span><span></span>
                            </div>
                            <span class="hero-panel-label">"what ships (gzip)"</span>
                        </div>
                        <div class="hero-panel-body hero-payload-preview">
                            <div class="hero-payload-row hero-payload-row-zero">
                                <span>"Static page"</span>
                                <strong>"0 B JS"</strong>
                            </div>
                            <div class="hero-payload-row hero-payload-row-accent">
                                <span>"Interactive page — loader"</span>
                                <strong>"907 B"</strong>
                            </div>
                            <div class="hero-payload-row">
                                <span>"First click — handler chunk"</span>
                                <strong>"5.08 KiB"</strong>
                            </div>
                            <div class="hero-payload-row hero-payload-row-muted">
                                <span>"Next.js counter (default scaffold)"</span>
                                <strong>"142 KiB"</strong>
                            </div>
                            <p class="hero-panel-caption">
                                <strong>"156× smaller"</strong>
                                " initial payload than a default Next.js app — measured on the same counter UX."
                            </p>
                        </div>
                    </div>
                </section>

                <div class="metrics-bar">
                    {metric_item("907 B", "loader (gzip)")}
                    {metric_item("0 B", "static pages")}
                    {metric_item("5.08 KiB", "first interaction")}
                    {metric_item("~3 KiB", "runtime core")}
                    {metric_item("1", "cargo dependency")}
                </div>
            </div>

            <section class="section zero-strip">
                <div class="zero-strip-inner">
                    <p class="zero-strip-eyebrow">"Zero-cost static"</p>
                    <h2 class="zero-strip-title">"Marketing pages ship no JavaScript"</h2>
                    <p class="zero-strip-body">
                        "Docs, blogs, and landing sections without signals or handlers compile to pure HTML. "
                        "No runtime. No hydration tax. Deploy to the edge and forget about bundle budgets."
                    </p>
                    <a href="/docs/flow/pages" class="btn btn-ghost">"Static vs interactive pages →"</a>
                </div>
            </section>

            <section class="section section-alt">
                <p class="section-eyebrow">"Payload anatomy"</p>
                <h2 class="section-title">"Every byte has a job"</h2>
                <p class="section-sub">
                    "Hydration frameworks ship the whole app up front. Resuma ships HTML plus a resumability "
                    "payload — handlers and core load only when needed."
                </p>
                <div class="payload-stack">
                    {payload_layer(
                        "SSR HTML + signal state",
                        "Server",
                        "View tree, data-r-on:* hooks, and <script type=\"resuma/state\"> — ready before any JS runs.",
                        false,
                    )}
                    {payload_layer(
                        "Loader",
                        "907 B gzip",
                        "Bootstraps signals from the serialised payload. Enough for static-looking pages to feel alive.",
                        true,
                    )}
                    {payload_layer(
                        "Core runtime",
                        "~3 KiB gzip",
                        "Fine-grained DOM updates, Show/For, effects — fetched on first interaction or prefetch.",
                        false,
                    )}
                    {payload_layer(
                        "Handler chunks",
                        "Per component",
                        "onClick and friends live in /_resuma/handler/{Component}.js — pay only for what users touch.",
                        false,
                    )}
                </div>
            </section>

            <section class="section">
                <p class="section-eyebrow">"Measured"</p>
                <h2 class="section-title">"Counter page — initial load (gzip)"</h2>
                <p class="section-sub">
                    "Same UX everywhere: SSR heading + one increment button. "
                    "Bar width is relative to Next.js (142 KiB). "
                    <a href="/docs/benchmark">"Full methodology →"</a>
                </p>
                <div class="speed-chart">
                    {speed_bar("Resuma", "907 B", 1, true)}
                    {speed_bar("Qwik", "1.96 KiB", 1, false)}
                    {speed_bar("SolidStart", "16.75 KiB", 12, false)}
                    {speed_bar("SvelteKit", "27.71 KiB", 19, false)}
                    {speed_bar("React (Vite)", "57.99 KiB", 41, false)}
                    {speed_bar("Leptos", "79.02 KiB", 56, false)}
                    {speed_bar("Next.js", "142.43 KiB", 100, false)}
                </div>
                <div class="bench-wrap">
                    <table class="bench">
                        <thead>
                            <tr>
                                <th>"Framework"</th>
                                <th>"Initial load"</th>
                                <th>"First interaction"</th>
                                <th>"Static page"</th>
                            </tr>
                        </thead>
                        <tbody>
                            {bench_row_full("Resuma", "907 B", "5.08 KiB", "0 B", true)}
                            {bench_row_full("Leptos", "79.02 KiB", "79.02 KiB", dash, false)}
                            {bench_row_full("Next.js", "142.43 KiB", "142.43 KiB", dash, false)}
                            {bench_row_full("React (Vite)", "57.99 KiB", "57.99 KiB", dash, false)}
                            {bench_row_full("Astro", "57.76 KiB", "57.76 KiB", dash, false)}
                            {bench_row_full("SvelteKit", "27.71 KiB", "27.71 KiB", dash, false)}
                            {bench_row_full("Qwik", "1.96 KiB", "22.32 KiB", dash, false)}
                            {bench_row_full("SolidStart", "16.75 KiB", "16.75 KiB", dash, false)}
                            {bench_row_full("templ + HTMX", "16.21 KiB", "16.21 KiB", dash, false)}
                        </tbody>
                    </table>
                </div>
                <p class="bench-note">
                    "Hydration frameworks load the same JS on page load — initial and first click match. "
                    "Resuma static pages ship " <strong>"0 B"</strong> " client JS. "
                    <code>"node benchmark/run.mjs"</code>
                </p>
            </section>

            <section class="section section-alt">
                <p class="section-eyebrow">"Try it in the docs"</p>
                <h2 class="section-title">"Live demos — inside the documentation"</h2>
                <p class="section-sub">
                    "Interactive examples run on every docs page: workers, signals, forms, and server functions. "
                    "The homepage stays lean — open a guide and click."
                </p>
                <div class="docs-try-grid">
                    {doc_link_card(
                        "/docs/exec/workers",
                        "Resuma OS workers",
                        "Run a real #[worker], watch the execution graph, pause and cancel.",
                        "LIVE",
                    )}
                    {doc_link_card(
                        "/docs/components/signals",
                        "Signals & reactivity",
                        "Increment a counter — fine-grained updates, no full re-render.",
                        "LIVE",
                    )}
                    {doc_link_card(
                        "/docs/components/server",
                        "Server functions",
                        "Call Rust from the browser without a page reload.",
                        "LIVE",
                    )}
                    {doc_link_card(
                        "/docs/components/form",
                        "Forms & #[submit]",
                        "Progressive enhancement — works as HTML POST before JS loads.",
                        "LIVE",
                    )}
                    {doc_link_card(
                        "/docs/components/show",
                        "Control flow",
                        "Show, For, Match — conditional UI with lazy boundaries.",
                        "LIVE",
                    )}
                    {doc_link_card(
                        "/docs/flow/loaders",
                        "Loaders & streaming",
                        "#[load] data fetching with cache headers and streaming SSR.",
                        "LIVE",
                    )}
                </div>
            </section>

            <section class="section">
                <p class="section-eyebrow">"Positioning"</p>
                <h2 class="section-title">"Resumability, not hydration"</h2>
                <p class="section-sub">"Three ways to ship interactive UI after the first paint."</p>
                <div class="compare-3">
                    {compare_column(
                        "Qwik",
                        "Resumable JS — tiny preloader, lazy chunks on interaction. Closest mental model to Resuma.",
                        false,
                    )}
                    {compare_column(
                        "Leptos",
                        "Rust SSR + WASM hydration and optional islands.",
                        false,
                    )}
                    {compare_column(
                        "Resuma",
                        "Rust SSR + resumability + lazy JS handlers — no WASM by default.",
                        true,
                    )}
                </div>
            </section>

            <section class="section section-alt">
                <p class="section-eyebrow">"Performance model"</p>
                <h2 class="section-title">"Interactive from the first click"</h2>
                <p class="section-sub">"The client never re-runs your component tree. State and handlers are already in the HTML."</p>
                <div class="pillars">
                    {pillar_card("⚡", "Ultralight by design", "907 B loader, ~3 KiB core, per-handler chunks — not a monolithic client bundle.")}
                    {pillar_card("🦀", "Full Rust stack", "#[server] RPC, #[submit] forms, and #[load] data — axum-native, no adapter boilerplate.")}
                    {pillar_card("📋", "Progressive enhancement", "<Form submit> works as plain HTML POST before JS loads; runtime enhances in place.")}
                    {pillar_card("🧩", "Resumable by default", "Every #[component] is a lazy boundary. Handlers externalise to /_resuma/handler/{Component}.js.")}
                </div>
            </section>

            <section class="section">
                <p class="section-eyebrow">"Under the hood"</p>
                <h2 class="section-title">"How does it work?"</h2>
                <p class="section-sub">"One SSR pass. One resumability payload. Lazy execution on the client."</p>
                <div class="pipeline">
                    {pipeline_step("1", "SSR renders once", "Rust walks the View tree, emits HTML + data-r-on:* attributes, and serialises signals into <script type=\"resuma/state\">.")}
                    {pipeline_step("2", "Payload travels light", "Handler sources move to lazy chunks. computed! / effect! / debounce! replay on the client via rs2js.")}
                    {pipeline_step("3", "Browser resumes", "Loader bootstraps signals. Core loads on first interaction. Handlers fetch on demand — or prefetch in viewport.")}
                </div>
            </section>

            <section class="section section-alt">
                <div class="showcase">
                    <div class="showcase-copy">
                        <p class="section-eyebrow">"Components"</p>
                        <h3>"Write UI once — on the server"</h3>
                        <p>"view! with JSX-like syntax, fine-grained signals, and onClick handlers that compile to lazy JavaScript."</p>
                        <ul class="showcase-list">
                            <li>"signal for reactive state"</li>
                            <li>"computed! / effect! for client replay"</li>
                            <li>"#[component] props builder generated for you"</li>
                        </ul>
                        <a href="/docs/components/view" class="btn btn-ghost">"Component guide →"</a>
                    </div>
                    <div class="showcase-code">
                        <div class="code-window">
                            {code_block(r#"#[component]
fn Counter() {
    let count = signal(0);
    view! {
        <button onClick={count.update(|c| *c += 1)}>
            "Count: " {count}
        </button>
    }
}
// Handler lazy-loads from /_resuma/handler/Counter.js"#)}
                        </div>
                    </div>
                </div>
            </section>

            <section class="section">
                <div class="showcase showcase-reverse">
                    <div class="showcase-copy">
                        <p class="section-eyebrow">"Resuma OS"</p>
                        <h3>"Durable workers — self-hosted"</h3>
                        <p>"#[worker] functions, disk-backed queues, cron scheduler, and an ops dashboard. No Redis, no external orchestrator — same binary as your app."</p>
                        <ul class="showcase-list">
                            <li>"Execution graphs with SSE event streams"</li>
                            <li>"Pause, resume, cancel from the Flow UI"</li>
                            <li>"Queue recovery and checkpointed state"</li>
                        </ul>
                        <a href="/docs/exec/workers" class="btn btn-ghost">"Run the live worker demo →"</a>
                    </div>
                    <div class="showcase-code">
                        <div class="code-window">
                            {code_block(r#"#[worker(intent = "enrich page")]
async fn enrich(input: Input, ctx: WorkerContext) -> Result<Value> {
    ctx.log("fetching");
    let page = ctx.tool("fetch", json!({ "url": input.url })).await?;
    ctx.progress(50);
    let summary = ctx.tool("ai", json!({
        "prompt": "Extract key facts",
        "data": page
    })).await?;
    Ok(summary)
}"#)}
                        </div>
                    </div>
                </div>
            </section>

            <section class="section section-alt">
                <p class="section-eyebrow">"Why Resuma?"</p>
                <h2 class="section-title">"Everything you need for modern SSR"</h2>
                <p class="section-sub">"Resumable SSR in Rust — one install, progressive enhancement, full-stack Flow when you need it."</p>
                <div class="grid-3">
                    {feature_card("🌊", "Resuma Flow", "File-based pages, #[load], #[submit], layouts, middleware — built into the same crate.")}
                    {feature_card("📄", "Static export", "resuma build --static-export scaffolds HTML from src/pages/ for edge-friendly deploys.")}
                    {feature_card("🔧", "Dev experience", "resuma dev with HMR WebSocket, resuma new templates (basic, todo, flow, production).")}
                    {feature_card("🔗", "JS bridge", "view! translates Rust closures via rs2js. js!{} for escape hatches when you need raw client code.")}
                    {feature_card("🏝️", "Islands (optional)", "#[island(load = \"visible\")] for heavy widgets — most UI only needs #[component].")}
                    {feature_card("🛡️", "Security built in", "Crypto CSRF, security headers, rate limits — see examples/todo for production patterns.")}
                </div>
            </section>

            <section class="section">
                <p class="section-eyebrow">"One package"</p>
                <h2 class="section-title">"Resuma¹ + Flow²"</h2>
                <p class="section-sub">"Two layers, one dependency. Core stays stable; Flow adds routing, data loading, and forms."</p>
                <div class="package-diagram">
                    <article class="package-box">
                        <p class="tag">"RESUMA¹ — CORE"</p>
                        <h3>"Components & resumability"</h3>
                        <ul>
                            <li>"view!, #[component], signal"</li>
                            <li>"computed! / effect! / debounce!"</li>
                            <li>"#[server], ResumaApp, ~3KB runtime"</li>
                        </ul>
                    </article>
                    <div class="package-plus">"+"</div>
                    <article class="package-box">
                        <p class="tag">"FLOW² — FULL-STACK"</p>
                        <h3>"Pages, loads & submits"</h3>
                        <ul>
                            <li>"FlowApp, src/pages/, #[layout]"</li>
                            <li>"#[load], #[submit], #[middleware]"</li>
                            <li>"Streaming SSR, cache headers"</li>
                        </ul>
                    </article>
                </div>
            </section>

            <section class="section section-alt">
                <p class="section-eyebrow">"AI assistants"</p>
                <h2 class="section-title">"Build faster with Cursor, Codex, or Gemini"</h2>
                <p class="section-sub">"Install the Resuma agent skill in one command — reactive view! rules, Flow patterns, server actions, and debugging checklists built in."</p>
                <div class="cta-install" style="margin: 1rem auto; max-width: 32rem;">"resuma install skill"</div>
                <p style="text-align: center;">
                    <a href="/docs/integrations/ai_assistant" class="btn btn-ghost">"AI assistant guide →"</a>
                </p>
            </section>

            <section class="section">
                <p class="section-eyebrow">"Integrations"</p>
                <h2 class="section-title">"Database, auth, and tooling"</h2>
                <p class="section-sub">"Integration guides for SQLx, Turso, auth, validation, i18n, and E2E testing."</p>
                <div class="grid-3">
                    <a href="/docs/integrations/sqlx" class="card" style="text-decoration: none;">
                        <h3>"SQLx"</h3>
                        <p>"Type-safe SQL in #[load] and #[submit]."</p>
                    </a>
                    <a href="/docs/integrations/turso" class="card" style="text-decoration: none;">
                        <h3>"Turso"</h3>
                        <p>"Edge libSQL — file in dev, remote in prod."</p>
                    </a>
                    <a href="/docs/integrations/auth" class="card" style="text-decoration: none;">
                        <h3>"Auth"</h3>
                        <p>"Sessions and middleware for protected routes."</p>
                    </a>
                </div>
                <p style="text-align: center; margin-top: 1rem;">
                    <a href="/docs/integrations">"All integrations"</a>
                    " · "
                    <a href="/docs/search">"Search docs"</a>
                </p>
            </section>

            <section class="cta-section">
                <div class="cta-banner">
                    <h2>"Start building in 60 seconds"</h2>
                    <p>"Install the CLI, scaffold a project, and ship instantly-interactive Rust UI — ultralight by default."</p>
                    <a href="/docs/getting_started" class="btn btn-primary">"Read the tutorial"</a>
                    <div class="cta-install">"cargo install resuma && resuma new my-app --template todo"</div>
                </div>
            </section>
        </main>
    }
}
