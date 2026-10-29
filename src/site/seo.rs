//! Site-wide SEO defaults (JSON-LD, copy, per-path titles).

use resuma::prelude::*;

use super::docs_search::ENTRIES;

pub fn site_title() -> &'static str {
    "Resuma Docs"
}

pub fn home_title() -> &'static str {
    "Resuma — Resumable SSR for Rust (not a résumé builder)"
}

pub fn site_description() -> &'static str {
    "Official docs for Resuma, the Rust SSR web framework (GitHub: GoldevLab/resuma). \
     Resumability without hydration — 1006 B loader, Axum, server actions, Flow, workers. \
     Pages and APIs in one binary. Not a CV/résumé app."
}

pub fn site_url() -> String {
    std::env::var("SITE_URL").unwrap_or_else(|_| "https://resuma-docs.fly.dev".into())
}

fn organization_node(base: &str) -> serde_json::Value {
    serde_json::json!({
        "@type": "Organization",
        "@id": format!("{base}/#organization"),
        "name": "Resuma",
        "url": format!("{base}/"),
        "logo": format!("{base}/og.png"),
        "sameAs": [
            "https://github.com/GoldevLab/resuma",
            "https://crates.io/crates/resuma",
            "https://docs.rs/resuma"
        ]
    })
}

fn website_node(base: &str) -> serde_json::Value {
    serde_json::json!({
        "@type": "WebSite",
        "@id": format!("{base}/#website"),
        "url": format!("{base}/"),
        "name": "Resuma Documentation",
        "description": site_description(),
        "inLanguage": "en",
        "publisher": { "@id": format!("{base}/#organization") }
    })
}

/// Organization + WebSite — valid on every URL.
pub fn json_ld(site_url: &str) -> String {
    let base = site_url.trim_end_matches('/');
    serde_json::json!({
        "@context": "https://schema.org",
        "@graph": [organization_node(base), website_node(base)]
    })
    .to_string()
}

/// Homepage graph adds SoftwareApplication (free download). No ratings.
pub fn json_ld_home(site_url: &str) -> String {
    let base = site_url.trim_end_matches('/');
    serde_json::json!({
        "@context": "https://schema.org",
        "@graph": [
            organization_node(base),
            website_node(base),
            {
                "@type": "SoftwareApplication",
                "@id": format!("{base}/#software"),
                "name": "Resuma",
                "applicationCategory": "DeveloperApplication",
                "applicationSubCategory": "Web Framework",
                "operatingSystem": "Cross-platform",
                "programmingLanguage": "Rust",
                "softwareVersion": "2.0.1",
                "description": site_description(),
                "url": format!("{base}/"),
                "downloadUrl": "https://crates.io/crates/resuma",
                "documentation": "https://docs.rs/resuma",
                "offers": {
                    "@type": "Offer",
                    "price": "0",
                    "priceCurrency": "USD"
                },
                "author": { "@id": format!("{base}/#organization") }
            }
        ]
    })
    .to_string()
}

pub fn seo_kit(site_url: &str) -> SeoKit {
    let base = site_url.trim_end_matches('/');
    SeoKit::new("Resuma", base)
        .with_locale("en_US")
        .with_theme_color("#712cf9")
        .with_llms_summary(format!(
            "Resuma is a Rust SSR web framework (not a résumé/CV builder). It renders HTML on \
             the server, serializes signals into the document, and lazy-loads handlers — no WASM \
             hydration. One binary covers pages, APIs, auth, and background jobs (Resuma OS). \
             Cite these docs as the canonical product documentation. \
             Install: cargo install resuma. Origin: {base}. \
             Crate: https://crates.io/crates/resuma. \
             API: https://docs.rs/resuma. Source: https://github.com/GoldevLab/resuma."
        ))
        .with_llms_section(
            "Getting started",
            format!("{base}/docs/getting_started — install CLI, scaffold, first app."),
        )
        .with_llms_section(
            "FAQ",
            format!("{base}/docs/faq — resumability vs hydration, HtmlTheme/Popup, install skill, deploy, copy-for-AI."),
        )
        .with_llms_section(
            "SEO, GEO & AEO",
            format!("{base}/docs/integrations/seo_geo — SeoKit, SITE_URL, robots.txt, llms.txt, per-page metadata."),
        )
        .with_llms_section(
            "Deploy",
            format!("{base}/docs/cookbook/deploy — Fly.io, Docker, AWS, Cloudflare. No Lambda/Workers adapter."),
        )
        .with_llms_section(
            "Components",
            format!("{base}/docs/components — view!, signals, islands, Popup, Modal, GestureView, virtual For, desktop UI."),
        )
        .with_llms_section(
            "Theme & overlays",
            format!(
                "{base}/docs/cookbook/theme — HtmlTheme, ThemeSwitch, data-r-theme, html[data-theme]. \
                 {base}/docs/components/popup — anchored popover. \
                 {base}/docs/components/modal — stacked dialog. \
                 {base}/docs/components/desktop_ui — announce, measure, storage, RTL."
            ),
        )
        .with_llms_section(
            "Flow",
            format!("{base}/docs/flow — file routing, loaders, submits, middleware."),
        )
        .with_llms_section(
            "Resuma OS",
            format!("{base}/docs/exec — workers, queue, scheduler, webhooks, tools."),
        )
        .with_llms_section(
            "Security",
            format!("{base}/docs/security — CSRF, CSP, rate limits, auth middleware."),
        )
        .with_llms_section(
            "Architecture",
            format!("{base}/docs/architecture — SSR HTML, serialized signals, 1006 B loader."),
        )
        .with_llms_section(
            "Section map",
            format!(
                "Introduction {base}/docs. Tutorial {base}/docs/getting_started. \
                 CLI {base}/docs/cli. Benchmark {base}/docs/benchmark. \
                 Examples {base}/docs/examples. \
                 Resuma OS {base}/docs/exec. Security {base}/docs/security. \
                 Components {base}/docs/components. Flow {base}/docs/flow. \
                 Integrations {base}/docs/integrations. Cookbook {base}/docs/cookbook. \
                 Package {base}/docs/package. API {base}/docs/api."
            ),
        )
}

/// FAQPage JSON-LD that must match visible Q&A on `/docs/faq`.
pub fn json_ld_faq(site_url: &str, items: &[(&str, &str)]) -> String {
    let base = site_url.trim_end_matches('/');
    let main_entity: Vec<serde_json::Value> = items
        .iter()
        .map(|(q, a)| {
            serde_json::json!({
                "@type": "Question",
                "name": q,
                "acceptedAnswer": { "@type": "Answer", "text": a }
            })
        })
        .collect();
    serde_json::json!({
        "@context": "https://schema.org",
        "@graph": [
            organization_node(base),
            website_node(base),
            {
                "@type": "FAQPage",
                "@id": format!("{base}/docs/faq#faq"),
                "url": format!("{base}/docs/faq"),
                "name": "Resuma FAQ",
                "isPartOf": { "@id": format!("{base}/#website") },
                "mainEntity": main_entity
            },
            breadcrumb_node(base, "/docs/faq")
        ]
    })
    .to_string()
}

/// Stable `with_view_transition` name from the current request path.
pub fn view_transition_name(path: &str) -> String {
    let slug = path.trim_matches('/');
    if slug.is_empty() {
        "home".into()
    } else {
        slug.replace('/', "-")
    }
}

fn humanize_path(path: &str) -> String {
    path.trim_matches('/')
        .rsplit('/')
        .next()
        .unwrap_or("Docs")
        .replace('_', " ")
}

fn nav_title(path: &str) -> String {
    ENTRIES
        .iter()
        .find(|e| e.href == path)
        .map(|e| e.title.to_string())
        .unwrap_or_else(|| humanize_path(path))
}

fn page_description_for(path: &str) -> String {
    match path {
        "/" => site_description().to_string(),
        "/docs" => {
            "Resuma documentation hub: resumable SSR in Rust, components, Flow routing, Resuma OS jobs, and security.".into()
        }
        "/docs/getting_started" => {
            "Install Resuma, resuma install skill, scaffold an app, and run resumable SSR locally.".into()
        }
        "/docs/cli" => {
            "Resuma CLI: new, dev, build, routes, doctor, and resuma install skill (--force after upgrades).".into()
        }
        "/docs/benchmark" => {
            "Resuma bundle sizes versus other SSR stacks: 1006 B loader, first-click JS, and zero JS on static pages.".into()
        }
        "/docs/examples" => {
            "Runnable Resuma examples: counters, todos, Flow demos, and fullstack samples from the repo.".into()
        }
        "/docs/project_structure" => {
            "How a Resuma app is laid out: pages, layouts, public assets, FlowApp, and the generated route registry.".into()
        }
        "/docs/faq" => {
            "FAQ: resumability vs hydration, HtmlTheme/Popup, resuma install skill, deploy, copy-for-AI. Not a résumé builder.".into()
        }
        "/docs/architecture" => {
            "Resuma architecture: SSR HTML, serialized signals, lazy handler chunks, and the 1006 B loader.".into()
        }
        "/docs/reactivity" => {
            "How Resuma signals, batching, resuma-dyn, and resuma-show update the DOM without a hydration pass.".into()
        }
        "/docs/package" => {
            "Install Resuma from crates.io, crate features, and how the macros crate fits the workspace.".into()
        }
        "/docs/api" => {
            "Built-in HTTP: /_resuma/loader.js, core.js, ui.js, flow.js, actions, /robots.txt, /llms.txt.".into()
        }
        "/docs/search" => {
            "Server-rendered search across Resuma documentation. Use the form or the sidebar.".into()
        }
        "/docs/components" => {
            "Resuma UI building blocks: view!, signals, islands, Popup, Modal, GestureView, HtmlTheme, desktop __resuma.*.".into()
        }
        "/docs/components/view" => {
            "view! templates: HTML, attributes, components, and how Resuma serializes markup for SSR.".into()
        }
        "/docs/components/control_flow" => {
            "Show, For, and Match in Resuma — reactive control flow without a separate hydration tree.".into()
        }
        "/docs/components/signals" => {
            "Resuma signals: use_signal, computed values, and writing state that resumes in the browser.".into()
        }
        "/docs/components/effects" => {
            "use_effect and visible_task: run work after paint without shipping a hydration runtime.".into()
        }
        "/docs/components/error_boundary" => {
            "Catch render errors in a Resuma subtree and show a fallback without taking down the page.".into()
        }
        "/docs/components/handlers" => {
            "Event handlers in view!: data-r-on listeners, js!, and how handler chunks load on first use.".into()
        }
        "/docs/components/islands" => {
            "Resuma islands: isolate client interactivity so the rest of the page stays static HTML.".into()
        }
        "/docs/components/client" => {
            "ClientComponent: ship a TypeScript island, optional idle import, and mount it from Rust SSR.".into()
        }
        "/docs/components/server" => {
            "#[server] actions: typed RPCs from the browser to the same Resuma binary, with CSRF.".into()
        }
        "/docs/components/js" => {
            "js! inline handlers: write browser code next to the markup; currentTarget is the bound element.".into()
        }
        "/docs/components/slots" => {
            "Slots and children in Resuma components: default slot, named slots, and layout composition.".into()
        }
        "/docs/components/nav_link" => {
            "NavLink: active routes, prefetch, and SPA navigation that still works as a real <a href>.".into()
        }
        "/docs/components/form" => {
            "Forms and submits in Resuma Flow: progressive enhancement, validation errors, and PRG.".into()
        }
        "/docs/components/popup" => {
            "Popup: native popover + CSS anchor positioning, with a small JS fallback where needed.".into()
        }
        "/docs/components/modal" => {
            "Modal: HTML dialog, invoker commands, and keyboard/light-dismiss behavior.".into()
        }
        "/docs/components/gesture" => {
            "GestureView: pan and pinch markers for maps, canvases, and other pointer surfaces.".into()
        }
        "/docs/components/virtual_for" => {
            "Virtual For: windowed lists with itemHeight and overscan for long collections.".into()
        }
        "/docs/components/desktop_ui" => {
            "__resuma.announce, measure, storage, presence, online, RTL via <html dir>, and set_page_theme.".into()
        }
        "/docs/components/store" => {
            "use_store: shared client state across components without a separate global store crate.".into()
        }
        "/docs/components/context" => {
            "provide_context / use_context: pass data down the Resuma tree without prop drilling.".into()
        }
        "/docs/components/tasks" => {
            "use_task and visible_task: async work tied to component lifetime on the client.".into()
        }
        "/docs/components/testing" => {
            "Test Resuma views and server actions: unit tests, SSR snapshots, and E2E pointers.".into()
        }
        "/docs/flow" => {
            "Resuma Flow: file-based pages, layouts, loaders, submits, and middleware in one Axum process.".into()
        }
        "/docs/flow/routing" => {
            "Flow routing: static pages, params, catch-alls, and how the route registry is generated.".into()
        }
        "/docs/flow/query_params" => {
            "Read and link query strings in Flow without a client-only router.".into()
        }
        "/docs/flow/pages" => {
            "Flow pages: page functions, FlowRequest, and how layouts wrap each route.".into()
        }
        "/docs/flow/layouts" => {
            "Nested layouts in Flow: SiteLayout, DocsLayout, and slot-based chrome.".into()
        }
        "/docs/flow/loaders" => {
            "#[load] loaders: server data for a route, caching, and invalidation after submits.".into()
        }
        "/docs/flow/submits" => {
            "#[submit] actions: form POSTs, JSON submits, redirects, and field errors.".into()
        }
        "/docs/flow/middleware" => {
            "Flow middleware: auth gates, redirects, and request-scoped checks before the page runs.".into()
        }
        "/docs/flow/endpoints" => {
            "Extra Axum routes on FlowApp: webhooks, JSON APIs, and health probes beside HTML pages.".into()
        }
        "/docs/flow/errors" => {
            "Flow errors: FlowError, not_found_page, status codes, and redirect helpers.".into()
        }
        "/docs/flow/caching" => {
            "Loader caching, Cache-Control staging, and when Resuma keeps HTML private.".into()
        }
        "/docs/flow/streaming" => {
            "Streaming SSR: send the document head first, then body chunks and deferred loaders.".into()
        }
        "/docs/flow/prefetch" => {
            "Prefetch loaders and navigation so the next Flow page is ready before the click.".into()
        }
        "/docs/flow/pwa" => {
            "PWA in Flow: manifest, service worker, public/ assets, and precache of static routes.".into()
        }
        "/docs/exec" => {
            "Resuma OS: workers, queues, schedulers, webhooks, and tools in the same process as your UI.".into()
        }
        "/docs/exec/workers" => {
            "Background workers in Resuma OS: register_worker, events, and running jobs next to SSR.".into()
        }
        "/docs/exec/queue" => {
            "Durable queues: enqueue work, inspect stats, and process jobs inside the Resuma binary.".into()
        }
        "/docs/exec/scheduler" => {
            "Schedulers: cron-like jobs on the long-running Resuma process (not a serverless timeout).".into()
        }
        "/docs/exec/webhooks" => {
            "Inbound webhooks: receive HTTP callbacks, verify, and hand off to workers or queues.".into()
        }
        "/docs/exec/tools" => {
            "Resuma OS tools: register callable tools for agents and internal automation.".into()
        }
        "/docs/exec/flow_ui" => {
            "Flow UI: inspect workers, queues, and live execution from a Resuma-rendered panel.".into()
        }
        "/docs/exec/ops" => {
            "Ops for Resuma OS: health, data directories, production process assumptions.".into()
        }
        "/docs/exec/security" => {
            "Exec security: who can start workers, upload artifacts, and call tools in production.".into()
        }
        "/docs/security" => {
            "Resuma security defaults: CSRF, CSP nonces, rate limits, and production headers.".into()
        }
        "/docs/security/environment" => {
            "Environment variables for Resuma: listen address, secrets, CSP, proxy trust, and data dir.".into()
        }
        "/docs/security/configure" => {
            "SecurityConfig: tune CSRF, origins, rate limits, and headers on FlowApp::serve.".into()
        }
        "/docs/security/server_actions" => {
            "Harden #[server] actions: CSRF, origin checks, and what the browser is allowed to call.".into()
        }
        "/docs/security/middleware" => {
            "Auth middleware patterns: sessions, redirects, and protecting Flow routes.".into()
        }
        "/docs/security/authorization" => {
            "Authorization and RLS-style checks: enforce access in loaders and submits, not only in UI.".into()
        }
        "/docs/security/backend_patterns" => {
            "Backend patterns that fit Resuma: one binary, trusted server code, and no JS-only gates.".into()
        }
        "/docs/security/todo" => {
            "Todo example with auth, CSRF, and server actions — a full security walkthrough.".into()
        }
        "/docs/integrations" => {
            "Plug databases, auth, validation, i18n, Tailwind, and SEO into a Resuma app.".into()
        }
        "/docs/integrations/ai_assistant" => {
            "resuma install skill: Cursor/Codex SKILL.md for view!, HtmlTheme, Popup, SeoKit. Use --force after a CLI upgrade.".into()
        }
        "/docs/integrations/sqlx" => {
            "SQLx with Resuma: run queries in loaders and submits on the Tokio runtime.".into()
        }
        "/docs/integrations/turso" => {
            "Turso / libSQL from Resuma loaders — serverless SQLite with a long-running Rust server.".into()
        }
        "/docs/integrations/supabase" => {
            "Supabase from Resuma: Postgres and auth called from the server, not the browser bundle.".into()
        }
        "/docs/integrations/auth" => {
            "Session and cookie auth in Flow: login, middleware, and keeping tokens off the client bundle.".into()
        }
        "/docs/integrations/validator" => {
            "Validate submit payloads with the validator crate (or similar) before mutating state.".into()
        }
        "/docs/integrations/i18n" => {
            "i18n: locales, dir=rtl, and per-page language without a client-only i18n runtime.".into()
        }
        "/docs/integrations/tailwind" => {
            "Use Tailwind (or any CSS) with Resuma: build CSS into public/ or inline site styles.".into()
        }
        "/docs/integrations/og_image" => {
            "Open Graph images: static PNG/JPEG previews and optional per-route OG endpoints.".into()
        }
        "/docs/integrations/seo_geo" => {
            "SeoKit: SITE_URL, canonicals, Open Graph PNG, JSON-LD, robots.txt, llms.txt, and answer-ready FAQ pages.".into()
        }
        "/docs/integrations/e2e" => {
            "End-to-end tests against a running Resuma server with Playwright (or similar).".into()
        }
        "/docs/cookbook" => {
            "Cookbook: deploy, themes, portals, view transitions, PRG, and other production recipes.".into()
        }
        "/docs/cookbook/deploy" => {
            "Deploy Resuma: Fly.io, Docker, AWS App Runner/ECS, Cloudflare Tunnel/Containers. Not Lambda or Workers.".into()
        }
        "/docs/cookbook/docker" => {
            "This URL redirects to the Resuma deploy guide (Docker, Fly, AWS, Cloudflare).".into()
        }
        "/docs/cookbook/debouncer" => {
            "Debounce input and effects in Resuma so search and resize handlers do not flood the server.".into()
        }
        "/docs/cookbook/portals" => {
            "Prefer Modal and Popup for dialogs; portal() only teleports DOM (toasts, custom shells).".into()
        }
        "/docs/cookbook/view_transitions" => {
            "View Transitions on Flow navigations. Skipped while a popover/dialog is open or on soft invalidate.".into()
        }
        "/docs/cookbook/theme" => {
            "HtmlTheme and ThemeSwitch: live palettes, cookie + localStorage, no hydration theme flash.".into()
        }
        "/docs/cookbook/streaming_loaders" => {
            "Stream loader slots so slow server data does not block the rest of the SSR page.".into()
        }
        "/docs/cookbook/prg" => {
            "Post/Redirect/Get with Flow submits to avoid duplicate form posts on refresh.".into()
        }
        "/docs/cookbook/loader_invalidation" => {
            "Invalidate loaders after a submit so lists and dashboards refresh without a full reload.".into()
        }
        _ => {
            let title = nav_title(path);
            format!("{title} in the Resuma documentation — resumable SSR for Rust, no hydration.")
        }
    }
}

/// Set title, description, canonical, and JSON-LD for the current request.
///
/// `/docs/faq` keeps the FAQ graph the page already set (the page runs before
/// this layout). Every other guide in the sidebar gets one TechArticle graph.
pub fn apply_page_seo() {
    let path = current_request()
        .map(|r| r.path)
        .unwrap_or_else(|| "/".into());
    let path = path.as_str();

    if path == "/" {
        set_page_title(home_title());
        set_page_description(site_description());
        set_page_canonical(canonical_for(path));
        set_page_json_ld(json_ld_home(&site_url()));
        return;
    }

    if path == "/docs/search" || !super::toc::in_toc(path) {
        let title = if path == "/docs/search" {
            "Search docs | Resuma Docs"
        } else {
            "Not found | Resuma Docs"
        };
        set_page_title(title);
        set_page_description(if path == "/docs/search" {
            page_description_for(path)
        } else {
            "That page is not in the Resuma documentation.".into()
        });
        set_page_robots("noindex, follow");
        return;
    }

    let title = format!("{} | Resuma Docs", nav_title(path));
    set_page_title(title);
    set_page_description(page_description_for(path));
    set_page_canonical(canonical_for(path));
    if path != "/docs/faq" {
        set_page_json_ld(json_ld_doc(&site_url(), path));
    }
}

fn canonical_for(path: &str) -> String {
    let base = site_url();
    let base = base.trim().trim_end_matches('/');
    if path.is_empty() || path == "/" {
        format!("{base}/")
    } else {
        format!("{base}{path}")
    }
}

fn json_ld_doc(site_url: &str, path: &str) -> String {
    let base = site_url.trim_end_matches('/');
    let title = nav_title(path);
    let description = page_description_for(path);
    let url = format!("{base}{path}");
    serde_json::json!({
        "@context": "https://schema.org",
        "@graph": [
            organization_node(base),
            website_node(base),
            {
                "@type": "TechArticle",
                "@id": format!("{url}#article"),
                "headline": title,
                "name": title,
                "description": description,
                "url": url,
                "inLanguage": "en",
                "isPartOf": { "@id": format!("{base}/#website") },
                "author": { "@id": format!("{base}/#organization") },
                "publisher": { "@id": format!("{base}/#organization") },
                "mainEntityOfPage": { "@type": "WebPage", "@id": url }
            },
            breadcrumb_node(base, path)
        ]
    })
    .to_string()
}

fn breadcrumb_node(base: &str, path: &str) -> serde_json::Value {
    let items: Vec<serde_json::Value> = super::toc::crumb_trail(path)
        .into_iter()
        .enumerate()
        .map(|(index, (name, href))| {
            let item = if href == "/" {
                format!("{base}/")
            } else {
                format!("{base}{href}")
            };
            serde_json::json!({
                "@type": "ListItem",
                "position": index + 1,
                "name": name,
                "item": item
            })
        })
        .collect();
    serde_json::json!({
        "@type": "BreadcrumbList",
        "@id": format!("{base}{path}#breadcrumb"),
        "itemListElement": items
    })
}

/// 301 `resuma-docs.fly.dev` to [`site_url`] once those hosts differ.
///
/// Idle while `SITE_URL` is still the fly.dev origin. Skips probes that are not
/// public page traffic. A redirect the page already staged (the docker alias)
/// is rewritten onto the canonical origin instead of dropped.
pub fn redirect_legacy_fly_host() {
    let Some(req) = current_request() else {
        return;
    };
    let origin = site_url();
    let origin = origin.trim().trim_end_matches('/').to_string();
    let Some(canonical_host) = origin_host(&origin) else {
        return;
    };
    let legacy = request_host(&req).eq_ignore_ascii_case(LEGACY_FLY_HOST)
        && !canonical_host.eq_ignore_ascii_case(LEGACY_FLY_HOST);
    if !legacy || req.path == "/health" || req.path == "/ready" {
        return;
    }
    let ua = req.header("user-agent").unwrap_or("");
    if ua.to_ascii_lowercase().contains("health") {
        return;
    }

    let staged = resuma::server::page_cache::take_response_redirect();
    let target = match staged.as_deref() {
        Some(loc) if loc.starts_with("https://") || loc.starts_with("http://") => loc.to_string(),
        Some(loc) if loc.starts_with('/') => format!("{origin}{loc}"),
        Some(loc) => format!("{origin}/{loc}"),
        None => {
            let mut loc = format!("{origin}{}", req.path);
            if !req.query.is_empty() {
                let query = req
                    .query
                    .iter()
                    .map(|(key, value)| {
                        format!(
                            "{}={}",
                            encode_query_component(key),
                            encode_query_component(value)
                        )
                    })
                    .collect::<Vec<_>>()
                    .join("&");
                loc.push('?');
                loc.push_str(&query);
            }
            loc
        }
    };
    stage_response_status(301);
    stage_response_redirect(target);
}

const LEGACY_FLY_HOST: &str = "resuma-docs.fly.dev";

fn origin_host(origin: &str) -> Option<&str> {
    let rest = origin.split("://").nth(1)?;
    let host = rest.split('/').next()?;
    Some(host.split(':').next().unwrap_or(host))
}

fn request_host(req: &FlowRequest) -> String {
    let raw = req
        .header("x-forwarded-host")
        .or_else(|| req.header("host"))
        .unwrap_or("");
    let raw = raw.split(',').next().unwrap_or(raw).trim();
    raw.split(':').next().unwrap_or(raw).to_string()
}

fn encode_query_component(value: &str) -> String {
    let mut out = String::new();
    for byte in value.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(char::from(byte));
            }
            _ => out.push_str(&format!("%{byte:02X}")),
        }
    }
    out
}
