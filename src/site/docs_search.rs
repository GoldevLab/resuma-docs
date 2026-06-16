//! Static docs search index (server-side filter, zero client JS).

#[derive(Clone, Copy)]
pub struct DocEntry {
    pub title: &'static str,
    pub href: &'static str,
    pub keywords: &'static str,
}

pub const ENTRIES: &[DocEntry] = &[
    DocEntry {
        title: "Overview",
        href: "/docs",
        keywords: "introduction start documentation hub",
    },
    DocEntry {
        title: "Getting Started",
        href: "/docs/getting_started",
        keywords: "install cli scaffold tutorial template production",
    },
    DocEntry {
        title: "Upgrading to 1.0",
        href: "/docs/migration_1_0",
        keywords: "migration upgrade semver 0.4 breaking changes safeAction",
    },
    DocEntry {
        title: "Benchmark",
        href: "/docs/benchmark",
        keywords: "bundle size gzip loader core qwik leptos next react",
    },
    DocEntry {
        title: "Examples",
        href: "/docs/examples",
        keywords: "runnable counter todo flow-demo smoke fullstack",
    },
    DocEntry {
        title: "FAQ",
        href: "/docs/faq",
        keywords: "hydration resumability wasm production ready bundle",
    },
    DocEntry {
        title: "Project structure",
        href: "/docs/project_structure",
        keywords: "layout src pages ResumaApp FlowApp cargo",
    },
    DocEntry {
        title: "Architecture",
        href: "/docs/architecture",
        keywords: "resumability hydration ssr payload runtime loader",
    },
    DocEntry {
        title: "Reactivity internals",
        href: "/docs/reactivity",
        keywords: "signals batching scheduler resuma-dyn resuma-show",
    },
    DocEntry {
        title: "Package",
        href: "/docs/package",
        keywords: "crates install dependencies resuma-macros version",
    },
    DocEntry {
        title: "CLI",
        href: "/docs/cli",
        keywords: "resuma new dev build routes doctor static-export update",
    },
    DocEntry {
        title: "API reference",
        href: "/docs/api",
        keywords: "docs.rs rust types traits prelude",
    },
    DocEntry {
        title: "Search docs",
        href: "/docs/search",
        keywords: "find query index",
    },
    DocEntry {
        title: "Components overview",
        href: "/docs/components",
        keywords: "view signals handlers islands form",
    },
    DocEntry {
        title: "view!",
        href: "/docs/components/view",
        keywords: "macro template jsx html attributes",
    },
    DocEntry {
        title: "Control flow",
        href: "/docs/components/control_flow",
        keywords: "For Match When Show If keyed list reactive",
    },
    DocEntry {
        title: "Signals",
        href: "/docs/components/signals",
        keywords: "use_signal state reactive update",
    },
    DocEntry {
        title: "Effects",
        href: "/docs/components/effects",
        keywords: "use_effect side effect lifecycle",
    },
    DocEntry {
        title: "Error boundaries",
        href: "/docs/components/error_boundary",
        keywords: "safeAction Result catch fallback ui",
    },
    DocEntry {
        title: "Handlers",
        href: "/docs/components/handlers",
        keywords: "onclick event closure rs2js chunk",
    },
    DocEntry {
        title: "Islands",
        href: "/docs/components/islands",
        keywords: "partial hydration lazy client boundary",
    },
    DocEntry {
        title: "Client components",
        href: "/docs/components/client",
        keywords: "typescript vite module nonce csp",
    },
    DocEntry {
        title: "Server actions",
        href: "/docs/components/server",
        keywords: "server macro rpc action post",
    },
    DocEntry {
        title: "js!",
        href: "/docs/components/js",
        keywords: "escape hatch async safeAction __resuma state",
    },
    DocEntry {
        title: "Slots",
        href: "/docs/components/slots",
        keywords: "children composition layout",
    },
    DocEntry {
        title: "NavLink",
        href: "/docs/components/nav_link",
        keywords: "spa navigation prefetch active class",
    },
    DocEntry {
        title: "Form",
        href: "/docs/components/form",
        keywords: "submit prg progressive enhancement csrf",
    },
    DocEntry {
        title: "Store",
        href: "/docs/components/store",
        keywords: "derive Store typed fields global state",
    },
    DocEntry {
        title: "Context",
        href: "/docs/components/context",
        keywords: "provide use_context dependency injection",
    },
    DocEntry {
        title: "Tasks",
        href: "/docs/components/tasks",
        keywords: "visible task async background work",
    },
    DocEntry {
        title: "Testing",
        href: "/docs/components/testing",
        keywords: "unit test render assert view",
    },
    DocEntry {
        title: "Flow overview",
        href: "/docs/flow",
        keywords: "pages routing fullstack file-based",
    },
    DocEntry {
        title: "Routing",
        href: "/docs/flow/routing",
        keywords: "dynamic params catch-all registry",
    },
    DocEntry {
        title: "Query params",
        href: "/docs/flow/query_params",
        keywords: "search filter loader_refresh navigate buildUrl",
    },
    DocEntry {
        title: "Pages",
        href: "/docs/flow/pages",
        keywords: "page function FlowRequest registry generate",
    },
    DocEntry {
        title: "Layouts",
        href: "/docs/flow/layouts",
        keywords: "nested layout shell sidebar",
    },
    DocEntry {
        title: "Loaders",
        href: "/docs/flow/loaders",
        keywords: "load data fetch server use_load Path Query",
    },
    DocEntry {
        title: "Actions (submits)",
        href: "/docs/flow/submits",
        keywords: "submit form post redirect get prg",
    },
    DocEntry {
        title: "Middleware",
        href: "/docs/flow/middleware",
        keywords: "auth guard session request pipeline",
    },
    DocEntry {
        title: "Endpoints",
        href: "/docs/flow/endpoints",
        keywords: "custom route api json handler",
    },
    DocEntry {
        title: "Error handling",
        href: "/docs/flow/errors",
        keywords: "ResumaError status load_boundary not found",
    },
    DocEntry {
        title: "Caching",
        href: "/docs/flow/caching",
        keywords: "cache-control max-age stale loader",
    },
    DocEntry {
        title: "Streaming",
        href: "/docs/flow/streaming",
        keywords: "chunked html deferred suspense",
    },
    DocEntry {
        title: "Prefetch",
        href: "/docs/flow/prefetch",
        keywords: "viewport lazy handler navlink hover",
    },
    DocEntry {
        title: "PWA & public",
        href: "/docs/flow/pwa",
        keywords: "manifest service worker installable icons precache static",
    },
    DocEntry {
        title: "Security overview",
        href: "/docs/security",
        keywords: "csrf rate limit headers production",
    },
    DocEntry {
        title: "Configure server",
        href: "/docs/security/configure",
        keywords: "csp nonce auto_pages security env",
    },
    DocEntry {
        title: "Secure server actions",
        href: "/docs/security/server_actions",
        keywords: "validation Result middleware audit safeAction",
    },
    DocEntry {
        title: "Auth middleware",
        href: "/docs/security/middleware",
        keywords: "session cookie guard FlowRequest user_id",
    },
    DocEntry {
        title: "Authorization & RLS",
        href: "/docs/security/authorization",
        keywords: "owner row level policy permission",
    },
    DocEntry {
        title: "Backend patterns",
        href: "/docs/security/backend_patterns",
        keywords: "service layer dto repository guard",
    },
    DocEntry {
        title: "Todo security example",
        href: "/docs/security/todo",
        keywords: "reference showcase guards validation",
    },
    DocEntry {
        title: "Cookbook overview",
        href: "/docs/cookbook",
        keywords: "recipes patterns deploy theme",
    },
    DocEntry {
        title: "Debouncer",
        href: "/docs/cookbook/debouncer",
        keywords: "input delay search throttle",
    },
    DocEntry {
        title: "Portals",
        href: "/docs/cookbook/portals",
        keywords: "modal overlay teleport dom",
    },
    DocEntry {
        title: "View transitions",
        href: "/docs/cookbook/view_transitions",
        keywords: "animation page transition css",
    },
    DocEntry {
        title: "Theme",
        href: "/docs/cookbook/theme",
        keywords: "dark mode css variables toggle",
    },
    DocEntry {
        title: "Streaming loaders",
        href: "/docs/cookbook/streaming_loaders",
        keywords: "deferred suspense progressive",
    },
    DocEntry {
        title: "PRG pattern",
        href: "/docs/cookbook/prg",
        keywords: "post redirect get submit form",
    },
    DocEntry {
        title: "Loader invalidation",
        href: "/docs/cookbook/loader_invalidation",
        keywords: "revalidate stale invalidate_href __resuma.invalidate",
    },
    DocEntry {
        title: "Docker deploy",
        href: "/docs/cookbook/docker",
        keywords: "container fly.io production dockerfile",
    },
    DocEntry {
        title: "Integrations overview",
        href: "/docs/integrations",
        keywords: "database auth styling testing",
    },
    DocEntry {
        title: "SQLx",
        href: "/docs/integrations/sqlx",
        keywords: "postgres sqlite orm query migrate pool",
    },
    DocEntry {
        title: "Turso",
        href: "/docs/integrations/turso",
        keywords: "libsql edge sqlite remote",
    },
    DocEntry {
        title: "Supabase",
        href: "/docs/integrations/supabase",
        keywords: "postgres hosted backend auth",
    },
    DocEntry {
        title: "Auth integration",
        href: "/docs/integrations/auth",
        keywords: "session login jwt middleware cookie",
    },
    DocEntry {
        title: "Validation",
        href: "/docs/integrations/validator",
        keywords: "validator zod submit form dto",
    },
    DocEntry {
        title: "i18n",
        href: "/docs/integrations/i18n",
        keywords: "translation locale fluent gettext",
    },
    DocEntry {
        title: "Tailwind CSS",
        href: "/docs/integrations/tailwind",
        keywords: "css styling utility class",
    },
    DocEntry {
        title: "OG Image",
        href: "/docs/integrations/og_image",
        keywords: "open graph social preview meta",
    },
    DocEntry {
        title: "SEO & GEO",
        href: "/docs/integrations/seo_geo",
        keywords: "meta pixel json-ld robots llms gptbot analytics",
    },
    DocEntry {
        title: "AI assistant",
        href: "/docs/integrations/ai_assistant",
        keywords: "cursor skill mcp codex gemini agent install",
    },
    DocEntry {
        title: "E2E testing",
        href: "/docs/integrations/e2e",
        keywords: "playwright test integration browser",
    },
];

pub fn search(query: &str) -> Vec<DocEntry> {
    let q = query.trim().to_lowercase();
    if q.is_empty() {
        return ENTRIES.to_vec();
    }
    ENTRIES
        .iter()
        .copied()
        .filter(|e| {
            e.title.to_lowercase().contains(&q)
                || e.href.to_lowercase().contains(&q)
                || e.keywords.to_lowercase().contains(&q)
        })
        .collect()
}
