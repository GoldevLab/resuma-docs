use resuma::prelude::*;

use crate::site::{doc_link_card, learn_path_card, metric_item};

pub fn page(_req: FlowRequest) -> View {
    view! {
        <div class="docs-hub">
            <header class="docs-hero">
                <h1>"Documentation"</h1>
                <p class="docs-hero-lead">
                    "Resumable SSR in Rust — components, server actions, full-stack Flow, production security, and deploy guides. One "
                    <code>"cargo install resuma"</code> " for core + Flow + CLI."
                </p>
                <form method="get" action="/docs/search" class="docs-search-hero">
                    <input type="search" name="q" placeholder="Search docs…" aria-label="Search documentation" />
                    <button type="submit">"Search"</button>
                </form>
                <div class="docs-quick-links">
                    <a href="/docs/getting_started">"Getting started"</a>
                    <a href="/docs/benchmark">"Benchmark"</a>
                    <a href="/docs/examples">"Examples"</a>
                    <a href="https://docs.rs/resuma/1.0.2" target="_blank">"API (docs.rs)"</a>
                    <a href="https://github.com/GolfredoPerezFernandez/resuma" target="_blank">"GitHub"</a>
                </div>
            </header>

            <div class="docs-stat-strip">
                {metric_item("907 B", "initial JS (gzip)")}
                {metric_item("5.08 KiB", "first interaction")}
                {metric_item("0 B", "static pages")}
                {metric_item("1 crate", "core + Flow + CLI")}
            </div>

            <h2 class="docs-section-title">"Choose your path"</h2>
            <div class="learn-paths">
                {learn_path_card(
                    "1",
                    "New to Resuma",
                    "Install the CLI, pick a template, and ship your first resumable page in minutes.",
                    "/docs/getting_started",
                    "Start tutorial →",
                )}
                {learn_path_card(
                    "2",
                    "Full-stack with Flow",
                    "File-based routing, loaders, form submits, layouts, and middleware in one crate.",
                    "/docs/flow",
                    "Flow guide →",
                )}
                {learn_path_card(
                    "3",
                    "Production ready",
                    "CSRF, rate limits, auth middleware, validation — walk through the todo showcase.",
                    "/docs/security/todo",
                    "Security walkthrough →",
                )}
            </div>

            <h2 class="docs-section-title">"Start here"</h2>
            <div class="grid-3">
                {doc_link_card(
                    "/docs/migration_1_0",
                    "Upgrading to 1.0",
                    "semver, js! handlers, new APIs, and CLI renames from 0.4.x.",
                    "1.0",
                )}
                {doc_link_card(
                    "/docs/getting_started",
                    "Getting Started",
                    "CLI install, templates (basic / todo / flow), first app.",
                    "Recommended",
                )}
                {doc_link_card(
                    "/docs/benchmark",
                    "Bundle benchmark",
                    "Measured comparison vs Qwik, Leptos, Next.js, React, Astro, and more.",
                    "Measured",
                )}
                {doc_link_card(
                    "/docs/examples",
                    "Examples",
                    "Runnable crates: counter, todo, flow-demo, flow-pages.",
                    "",
                )}
            </div>

            <h2 class="docs-section-title">"Learn by topic"</h2>
            <div class="grid-3">
                {doc_link_card(
                    "/docs/components",
                    "Components",
                    "view!, signals, handlers, islands, server actions, js!.",
                    "",
                )}
                {doc_link_card(
                    "/docs/flow",
                    "Resuma Flow",
                    "Pages, loads, submits, routing, streaming, caching.",
                    "",
                )}
                {doc_link_card(
                    "/docs/security",
                    "Security",
                    "CSRF, headers, rate limits, auth, authorization.",
                    "",
                )}
                {doc_link_card(
                    "/docs/cookbook",
                    "Cookbook",
                    "Theme, portals, streaming loaders, Docker deploy.",
                    "",
                )}
                {doc_link_card(
                    "/docs/integrations",
                    "Integrations",
                    "SQLx, Turso, auth, Tailwind, i18n, E2E testing.",
                    "",
                )}
                {doc_link_card(
                    "/docs/architecture",
                    "Architecture",
                    "Resumability vs hydration — SSR payload and runtime.",
                    "",
                )}
                {doc_link_card(
                    "/docs/project_structure",
                    "Project structure",
                    "ResumaApp vs FlowApp layouts and conventions.",
                    "",
                )}
                {doc_link_card(
                    "/docs/cli",
                    "CLI",
                    "new, dev, build, routes --generate, doctor.",
                    "",
                )}
                {doc_link_card(
                    "/docs/api",
                    "API reference",
                    "Link to docs.rs for the full Rust API surface.",
                    "",
                )}
            </div>

            <h2 class="docs-section-title">"What is Resuma?"</h2>
            <p>
                "Components run once on the server. SSR embeds signals and handler references in HTML; "
                "a "
                <strong>"907 B"</strong>
                " gzip loader resumes interactivity on first click — no hydration, no WASM bundle by default."
            </p>
            <p>
                <strong>"Resuma Flow"</strong>
                " adds file-based pages, "
                <code>"#[load]"</code>
                ", "
                <code>"#[submit]"</code>
                ", and middleware — still one "
                <code>"resuma"</code>
                " crate."
            </p>
            <p>
                "Published on "
                <a href="https://crates.io/crates/resuma" target="_blank">"crates.io"</a>
                " · "
                <a href="https://docs.rs/resuma/1.0.2" target="_blank">"docs.rs"</a>
                " · benchmark source in the "
                <a href="https://github.com/GolfredoPerezFernandez/resuma/tree/main/benchmark" target="_blank">"GitHub repo"</a>"."
            </p>
        </div>
    }
}
