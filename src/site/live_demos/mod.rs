//! Live demo entry points — one function per documentation topic.

mod widgets;

use crate::site::demo_actions::{DocsCachedData, DocsSearchData};
use crate::site::demo_shell::{live_demo, live_info};
use crate::site::exec_demo::exec_showcase_demo;
use crate::site::server_demo::server_function_demo;
use resuma::prelude::*;
use widgets::*;

// ── Resuma OS ───────────────────────────────────────────────────────────────

pub fn exec_overview() -> View {
    live_demo("Resuma OS worker", exec_showcase_demo())
}

pub fn exec_workers() -> View {
    live_demo("#[worker] + execution graph", exec_showcase_demo())
}

pub fn exec_queue() -> View {
    live_info(
        "Queue API",
        view! {
            <>
                {exec_showcase_demo()}
                <p>"Enqueue durable jobs with " <code>"POST /_resuma/queue/{name}"</code></p>
                <p class="demo-muted">"The live worker above uses the same disk-backed queue store as production apps."</p>
            </>
        },
    )
}

pub fn exec_scheduler() -> View {
    live_info(
        "Cron scheduler",
        view! {
            <>
                <p>"Register jobs via " <code>"GET|POST /_resuma/scheduler"</code></p>
                {ServerActionWidget::render(ServerActionWidgetProps::default())}
            </>
        },
    )
}

pub fn exec_webhooks() -> View {
    live_demo(
        "Webhooks",
        view! {
            <>
                <p>
                    "Configure outbound hooks at "
                    <code>"GET|POST /_resuma/webhooks"</code>
                    " — fires on graph lifecycle events."
                </p>
                {crate::site::webhook_demo::WebhookDemoWidget::render(
                    crate::site::webhook_demo::WebhookDemoWidgetProps::default(),
                )}
            </>
        },
    )
}

pub fn exec_tools() -> View {
    live_info(
        "Tools registry",
        view! {
            <p>"Register callable tools for AI agents via " <code>"resuma::exec::tools"</code> " — same binary, no sidecar."</p>
        },
    )
}

pub fn exec_flow_ui() -> View {
    live_demo("Flow execution UI", exec_showcase_demo())
}

pub fn exec_ops() -> View {
    live_info(
        "Ops endpoints",
        view! {
            <>
                <p><code>"GET /_resuma/status"</code> " · " <code>"GET /_resuma/metrics"</code></p>
                <p class="demo-muted">"Prometheus text + JSON snapshot. Use RESUMA_OPS_API_KEY in production."</p>
            </>
        },
    )
}

pub fn exec_security() -> View {
    live_info(
        "Exec security",
        view! {
            <p>"Graph access tokens, SSRF guards on worker HTTP, and rate limits on " <code>"/_resuma/worker/*"</code></p>
        },
    )
}

// ── Security ────────────────────────────────────────────────────────────────

pub fn security_overview() -> View {
    live_demo(
        "Server action round-trip",
        ServerActionWidget::render(ServerActionWidgetProps::default()),
    )
}

pub fn security_configure() -> View {
    live_info(
        "SecurityConfig",
        view! {
            <p>"Set CSRF, origin checks, and rate limits in " <code>"FlowServeOptions::security"</code></p>
        },
    )
}

pub fn security_server_actions() -> View {
    live_demo(
        "#[server] RPC",
        ServerActionWidget::render(ServerActionWidgetProps::default()),
    )
}

pub fn security_middleware() -> View {
    live_info(
        "Auth middleware",
        view! {
            <p>"Wrap routes with " <code>"#[middleware]"</code> " — every request to this docs site runs through Flow middleware."</p>
            <NavLink href="/docs" activeClass="active">"Docs home"</NavLink>
        },
    )
}

pub fn security_authorization() -> View {
    live_info(
        "Authorization patterns",
        view! {
            <p>"Combine " <code>"FlowRequest"</code> " user id with row-level checks in server actions."</p>
        },
    )
}

pub fn security_backend() -> View {
    live_info(
        "Backend patterns",
        view! {
            <p>"Store per-user data under " <code>"RESUMA_DATA_DIR"</code> " with file locks — see Todo example."</p>
        },
    )
}

pub fn security_todo() -> View {
    live_demo(
        "Interactive counter (todo pattern)",
        CounterWidget::render(CounterWidgetProps::default()),
    )
}

// ── Components ──────────────────────────────────────────────────────────────

pub fn components_overview() -> View {
    live_demo(
        "Signals + handlers",
        CounterWidget::render(CounterWidgetProps::default()),
    )
}

pub fn components_view() -> View {
    live_demo(
        "view! counter",
        CounterWidget::render(CounterWidgetProps::default()),
    )
}

pub fn components_control_flow() -> View {
    live_demo(
        "<Show> toggle",
        ShowWidget::render(ShowWidgetProps::default()),
    )
}

pub fn components_signals() -> View {
    live_demo(
        "signal()",
        CounterWidget::render(CounterWidgetProps::default()),
    )
}

pub fn components_effects() -> View {
    live_demo(
        "effect! / computed",
        EffectsWidget::render(EffectsWidgetProps::default()),
    )
}

pub fn components_error_boundary() -> View {
    live_demo(
        "error_boundary()",
        ErrorBoundaryWidget::render(ErrorBoundaryWidgetProps::default()),
    )
}

pub fn components_handlers() -> View {
    live_demo(
        "onClick handler",
        HandlersWidget::render(HandlersWidgetProps::default()),
    )
}

pub fn components_islands() -> View {
    live_demo("#[island]", island_demo())
}

pub fn components_client() -> View {
    live_info(
        "TypeScript client",
        view! {
            <p>"Client components mount via " <code>"ClientComponent"</code> " — see homepage hero particles."</p>
            {JsWidget::render(JsWidgetProps::default())}
        },
    )
}

pub fn components_server() -> View {
    live_demo(
        "#[server]",
        ServerActionWidget::render(ServerActionWidgetProps::default()),
    )
}

pub fn components_js() -> View {
    live_demo("js! handlers", JsWidget::render(JsWidgetProps::default()))
}

pub fn components_slots() -> View {
    live_demo("Slots", slots_widget())
}

pub fn components_nav_link() -> View {
    live_demo("NavLink SPA", nav_link_widget())
}

pub fn components_form() -> View {
    live_demo(
        "#[submit] Form",
        GreetFormWidget::render(GreetFormWidgetProps::default()),
    )
}

pub fn components_store() -> View {
    live_demo(
        "use_store",
        StoreWidget::render(StoreWidgetProps::default()),
    )
}

pub fn components_context() -> View {
    live_demo("provide_context", context_widget())
}

pub fn components_tasks() -> View {
    live_info(
        "Visible tasks",
        view! {
            <p>"Defer work until viewport via " <code>"use_visible_task"</code> " — scroll this panel into view to mount."</p>
            {CounterWidget::render(CounterWidgetProps::default())}
        },
    )
}

pub fn components_testing() -> View {
    live_demo(
        "Testable signal",
        CounterWidget::render(CounterWidgetProps::default()),
    )
}

// ── Flow ────────────────────────────────────────────────────────────────────

pub fn flow_overview() -> View {
    live_demo("Flow + server fn", server_function_demo())
}

pub fn flow_routing() -> View {
    live_info(
        "File-based routes",
        view! {
            <>
                <p>"This page is " <code>"src/pages/docs/flow/routing.rs"</code> " → " <code>"/docs/flow/routing"</code></p>
                <div class="demo-row">
                    <NavLink href="/docs/flow/pages" activeClass="active">"Pages"</NavLink>
                    <NavLink href="/docs/flow/loaders" activeClass="active">"Loaders"</NavLink>
                </div>
            </>
        },
    )
}

pub fn flow_query_params(req: &FlowRequest) -> View {
    let q = req.query_param("q").unwrap_or("");
    let data = match try_use_load::<DocsSearchData>("docs_search") {
        Ok(d) => d,
        Err(_) => DocsSearchData {
            query: q.to_string(),
            results: vec![],
        },
    };
    live_demo(
        "#[load] + query string",
        view! {
            <>
                <form method="get" action="/docs/flow/query_params" class="demo-row">
                    <input type="search" name="q" value={q.to_string()} placeholder="Search (min 2 chars)" />
                    <button type="submit" class="btn btn-sm">"Search"</button>
                </form>
                <p class="demo-output">"Query: " {data.query.clone()}</p>
                <ul>
                    {data.results.iter().map(|r| view! { <li>{r.clone()}</li> }).collect::<Vec<_>>()}
                </ul>
            </>
        },
    )
}

pub fn flow_pages() -> View {
    live_info(
        "Pages registry",
        view! {
            <p><code>"resuma routes --generate --path src/pages"</code> " wires every " <code>"pub fn page"</code></p>
        },
    )
}

pub fn flow_layouts() -> View {
    live_info(
        "Layouts",
        view! {
            <p>"This page uses the " <code>"/docs"</code> " layout with sidebar — defined in " <code>"main.rs"</code></p>
        },
    )
}

pub fn flow_loaders() -> View {
    let cached = match try_use_load::<DocsCachedData>("docs_cached") {
        Ok(d) => d,
        Err(_) => DocsCachedData {
            value: "Loader unavailable outside Flow scope".into(),
            timestamp: String::new(),
        },
    };
    live_demo(
        "#[load] with cache",
        view! {
            <>
                <p class="demo-output">{cached.value.clone()}</p>
                <p class="demo-muted">{"Loaded at: "}{cached.timestamp.clone()}</p>
            </>
        },
    )
}

pub fn flow_submits() -> View {
    live_demo(
        "#[submit]",
        GreetFormWidget::render(GreetFormWidgetProps::default()),
    )
}

pub fn flow_middleware() -> View {
    live_info(
        "Middleware",
        view! {
            <p>"Request logged by Flow middleware — check server stdout."</p>
        },
    )
}

pub fn flow_endpoints() -> View {
    live_demo(
        "#[server] endpoint",
        ServerActionWidget::render(ServerActionWidgetProps::default()),
    )
}

pub fn flow_errors() -> View {
    live_demo(
        "Error boundary",
        ErrorBoundaryWidget::render(ErrorBoundaryWidgetProps::default()),
    )
}

pub fn flow_caching() -> View {
    flow_loaders()
}

pub fn flow_streaming() -> View {
    live_info(
        "Streaming loaders",
        view! {
            <p>"Use " <code>"#[load(stream)]"</code> " — HTML streams while loader runs. See cookbook streaming loaders."</p>
        },
    )
}

pub fn flow_prefetch() -> View {
    live_info(
        "NavLink prefetch",
        view! {
            <>
                <p>"Hover a NavLink — route HTML prefetches before click."</p>
                <NavLink href="/docs/flow/routing" activeClass="active">"Hover me →"</NavLink>
            </>
        },
    )
}

pub fn flow_pwa() -> View {
    live_info(
        "PWA manifest",
        view! {
            <p>"Configure via " <code>"FlowApp::with_pwa"</code> " — this docs site ships a web manifest."</p>
        },
    )
}

// ── Integrations ────────────────────────────────────────────────────────────

pub fn integrations_overview() -> View {
    live_info(
        "CLI extensions",
        view! {
            <>
                <p><code>"resuma add sqlx|turso|tailwind|…"</code></p>
                {ServerActionWidget::render(ServerActionWidgetProps::default())}
            </>
        },
    )
}

pub fn integrations_generic(title: &str, cmd: &str) -> View {
    live_info(
        title,
        view! {
            <>
                <p><code>{cmd.to_string()}</code></p>
                {CounterWidget::render(CounterWidgetProps::default())}
            </>
        },
    )
}

// ── Cookbook ────────────────────────────────────────────────────────────────

pub fn cookbook_overview() -> View {
    live_demo(
        "Debounced input",
        DebounceWidget::render(DebounceWidgetProps::default()),
    )
}

pub fn cookbook_debouncer() -> View {
    live_demo(
        "Debouncer",
        DebounceWidget::render(DebounceWidgetProps::default()),
    )
}

pub fn cookbook_portals() -> View {
    live_demo(
        "portal()",
        PortalWidget::render(PortalWidgetProps::default()),
    )
}

pub fn cookbook_view_transitions() -> View {
    live_info(
        "View transitions",
        view! {
            <>
                <p>"Navigate with " <code>"data-r-vt"</code> " for CSS view transitions."</p>
                <NavLink href="/docs/cookbook/theme" activeClass="active" data-r-vt="slide">"Navigate →"</NavLink>
            </>
        },
    )
}

pub fn cookbook_theme() -> View {
    live_demo(
        "provide_theme",
        ThemeWidget::render(ThemeWidgetProps::default()),
    )
}

pub fn cookbook_streaming_loaders() -> View {
    live_info(
        "Streaming loaders",
        view! {
            <p><code>"#[load(stream)]"</code> " defers HTML chunk until loader completes."</p>
        },
    )
}

pub fn cookbook_prg() -> View {
    live_demo(
        "PRG submit",
        view! {
            <Form submit={crate::site::demo_actions::docs_prg}>
                <label>"Item" <input name="item" type="text" required=true /></label>
                <button type="submit" class="btn btn-sm">"Submit → redirect"</button>
            </Form>
        },
    )
}

pub fn cookbook_loader_invalidation() -> View {
    live_info(
        "invalidate()",
        view! {
            <>
                <p>"Call " <code>"__resuma.invalidate(href)"</code> " to bust SPA prefetch cache."</p>
                <button type="button" class="btn btn-sm" onClick={js! {
                    __resuma.invalidate("/docs/flow/loaders");
                }}>"Invalidate loaders page"</button>
            </>
        },
    )
}

pub fn cookbook_docker() -> View {
    live_info(
        "Docker deploy",
        view! {
            <p><code>"docker build -t myapp . && docker run -p 3000:3000 myapp"</code></p>
        },
    )
}

// ── Reference ───────────────────────────────────────────────────────────────

pub fn reference_architecture() -> View {
    live_demo(
        "Resumability pipeline",
        PipelineWidget::render(PipelineWidgetProps::default()),
    )
}

pub fn reference_reactivity() -> View {
    live_demo(
        "Signal → effect chain",
        ReactivityWidget::render(ReactivityWidgetProps::default()),
    )
}

pub fn reference_package() -> View {
    live_info(
        "Install",
        view! {
            <>
                <p><code>"cargo add resuma@1.2.0"</code></p>
                {ServerActionWidget::render(ServerActionWidgetProps::default())}
            </>
        },
    )
}

pub fn reference_cli() -> View {
    live_info(
        "CLI commands",
        view! {
            <ul>
                <li><code>"resuma new"</code></li>
                <li><code>"resuma dev"</code></li>
                <li><code>"resuma routes --generate"</code></li>
            </ul>
        },
    )
}

pub fn reference_api() -> View {
    live_info(
        "docs.rs",
        view! {
            <>
                <p><a href="https://docs.rs/resuma/1.2.0" target="_blank">"docs.rs/resuma"</a></p>
                {CounterWidget::render(CounterWidgetProps::default())}
            </>
        },
    )
}
