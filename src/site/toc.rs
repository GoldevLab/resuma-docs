//! Ordered docs map. Sidebar, breadcrumbs, prev/next, and article JSON-LD share it.

use resuma::prelude::*;

pub struct TocLink {
    pub href: &'static str,
    pub label: &'static str,
}

pub struct TocSection {
    pub title: &'static str,
    /// Section overview. `None` when the section has no hub page.
    pub hub: Option<&'static str>,
    pub links: &'static [TocLink],
}

pub const TOC: &[TocSection] = &[
    TocSection {
        title: "Introduction",
        hub: Some("/docs"),
        links: &[
            TocLink {
                href: "/docs",
                label: "Overview",
            },
            TocLink {
                href: "/docs/getting_started",
                label: "Getting Started",
            },
            TocLink {
                href: "/docs/cli",
                label: "CLI",
            },
            TocLink {
                href: "/docs/benchmark",
                label: "Benchmark",
            },
            TocLink {
                href: "/docs/examples",
                label: "Examples",
            },
            TocLink {
                href: "/docs/project_structure",
                label: "Project structure",
            },
            TocLink {
                href: "/docs/faq",
                label: "FAQ",
            },
        ],
    },
    TocSection {
        title: "Resuma OS",
        hub: Some("/docs/exec"),
        links: &[
            TocLink {
                href: "/docs/exec",
                label: "Overview",
            },
            TocLink {
                href: "/docs/exec/workers",
                label: "Workers",
            },
            TocLink {
                href: "/docs/exec/queue",
                label: "Queue",
            },
            TocLink {
                href: "/docs/exec/scheduler",
                label: "Scheduler",
            },
            TocLink {
                href: "/docs/exec/webhooks",
                label: "Webhooks",
            },
            TocLink {
                href: "/docs/exec/tools",
                label: "Tools",
            },
            TocLink {
                href: "/docs/exec/flow_ui",
                label: "Flow UI",
            },
            TocLink {
                href: "/docs/exec/ops",
                label: "Ops & production",
            },
            TocLink {
                href: "/docs/exec/security",
                label: "Exec security",
            },
        ],
    },
    TocSection {
        title: "Security",
        hub: Some("/docs/security"),
        links: &[
            TocLink {
                href: "/docs/security",
                label: "Overview",
            },
            TocLink {
                href: "/docs/security/environment",
                label: "Environment variables",
            },
            TocLink {
                href: "/docs/security/configure",
                label: "Configure server",
            },
            TocLink {
                href: "/docs/security/server_actions",
                label: "Server actions",
            },
            TocLink {
                href: "/docs/security/middleware",
                label: "Auth middleware",
            },
            TocLink {
                href: "/docs/security/authorization",
                label: "Authorization & RLS",
            },
            TocLink {
                href: "/docs/security/backend_patterns",
                label: "Backend patterns",
            },
            TocLink {
                href: "/docs/security/todo",
                label: "Todo example",
            },
        ],
    },
    TocSection {
        title: "Components",
        hub: Some("/docs/components"),
        links: &[
            TocLink {
                href: "/docs/components",
                label: "Overview",
            },
            TocLink {
                href: "/docs/components/view",
                label: "view!",
            },
            TocLink {
                href: "/docs/components/control_flow",
                label: "Control flow",
            },
            TocLink {
                href: "/docs/components/signals",
                label: "Signals",
            },
            TocLink {
                href: "/docs/components/effects",
                label: "Effects",
            },
            TocLink {
                href: "/docs/components/error_boundary",
                label: "Error boundaries",
            },
            TocLink {
                href: "/docs/components/handlers",
                label: "Handlers",
            },
            TocLink {
                href: "/docs/components/islands",
                label: "Islands",
            },
            TocLink {
                href: "/docs/components/client",
                label: "Client (TypeScript)",
            },
            TocLink {
                href: "/docs/components/server",
                label: "Server actions",
            },
            TocLink {
                href: "/docs/components/js",
                label: "js!",
            },
            TocLink {
                href: "/docs/components/slots",
                label: "Slots",
            },
            TocLink {
                href: "/docs/components/nav_link",
                label: "NavLink",
            },
            TocLink {
                href: "/docs/components/form",
                label: "Form",
            },
            TocLink {
                href: "/docs/components/popup",
                label: "Popup",
            },
            TocLink {
                href: "/docs/components/modal",
                label: "Modal",
            },
            TocLink {
                href: "/docs/components/gesture",
                label: "GestureView",
            },
            TocLink {
                href: "/docs/components/virtual_for",
                label: "Virtual For",
            },
            TocLink {
                href: "/docs/components/desktop_ui",
                label: "Desktop UI",
            },
            TocLink {
                href: "/docs/components/store",
                label: "Store",
            },
            TocLink {
                href: "/docs/components/context",
                label: "Context",
            },
            TocLink {
                href: "/docs/components/tasks",
                label: "Tasks",
            },
            TocLink {
                href: "/docs/components/testing",
                label: "Testing",
            },
        ],
    },
    TocSection {
        title: "Resuma Flow",
        hub: Some("/docs/flow"),
        links: &[
            TocLink {
                href: "/docs/flow",
                label: "Overview",
            },
            TocLink {
                href: "/docs/flow/routing",
                label: "Routing",
            },
            TocLink {
                href: "/docs/flow/query_params",
                label: "Query params",
            },
            TocLink {
                href: "/docs/flow/pages",
                label: "Pages",
            },
            TocLink {
                href: "/docs/flow/layouts",
                label: "Layouts",
            },
            TocLink {
                href: "/docs/flow/loaders",
                label: "Loaders",
            },
            TocLink {
                href: "/docs/flow/submits",
                label: "Actions",
            },
            TocLink {
                href: "/docs/flow/middleware",
                label: "Middleware",
            },
            TocLink {
                href: "/docs/flow/endpoints",
                label: "Endpoints",
            },
            TocLink {
                href: "/docs/flow/errors",
                label: "Error handling",
            },
            TocLink {
                href: "/docs/flow/caching",
                label: "Caching",
            },
            TocLink {
                href: "/docs/flow/streaming",
                label: "Streaming",
            },
            TocLink {
                href: "/docs/flow/prefetch",
                label: "Prefetch",
            },
            TocLink {
                href: "/docs/flow/pwa",
                label: "PWA & public/",
            },
        ],
    },
    TocSection {
        title: "Integrations",
        hub: Some("/docs/integrations"),
        links: &[
            TocLink {
                href: "/docs/integrations",
                label: "Overview",
            },
            TocLink {
                href: "/docs/integrations/ai_assistant",
                label: "AI assistant",
            },
            TocLink {
                href: "/docs/integrations/sqlx",
                label: "SQLx",
            },
            TocLink {
                href: "/docs/integrations/turso",
                label: "Turso",
            },
            TocLink {
                href: "/docs/integrations/supabase",
                label: "Supabase",
            },
            TocLink {
                href: "/docs/integrations/auth",
                label: "Auth",
            },
            TocLink {
                href: "/docs/integrations/validator",
                label: "Validation",
            },
            TocLink {
                href: "/docs/integrations/i18n",
                label: "i18n",
            },
            TocLink {
                href: "/docs/integrations/tailwind",
                label: "Tailwind",
            },
            TocLink {
                href: "/docs/integrations/og_image",
                label: "OG Image",
            },
            TocLink {
                href: "/docs/integrations/seo_geo",
                label: "SEO, GEO & AEO",
            },
            TocLink {
                href: "/docs/integrations/e2e",
                label: "E2E testing",
            },
        ],
    },
    TocSection {
        title: "Cookbook",
        hub: Some("/docs/cookbook"),
        links: &[
            TocLink {
                href: "/docs/cookbook",
                label: "Overview",
            },
            TocLink {
                href: "/docs/cookbook/debouncer",
                label: "Debouncer",
            },
            TocLink {
                href: "/docs/cookbook/portals",
                label: "Portals",
            },
            TocLink {
                href: "/docs/cookbook/view_transitions",
                label: "View transitions",
            },
            TocLink {
                href: "/docs/cookbook/theme",
                label: "Theme",
            },
            TocLink {
                href: "/docs/cookbook/streaming_loaders",
                label: "Streaming loaders",
            },
            TocLink {
                href: "/docs/cookbook/prg",
                label: "PRG pattern",
            },
            TocLink {
                href: "/docs/cookbook/loader_invalidation",
                label: "Loader invalidation",
            },
            TocLink {
                href: "/docs/cookbook/deploy",
                label: "Deploy (Fly, AWS, CF, …)",
            },
        ],
    },
    TocSection {
        title: "Reference",
        hub: None,
        links: &[
            TocLink {
                href: "/docs/architecture",
                label: "Architecture",
            },
            TocLink {
                href: "/docs/reactivity",
                label: "Reactivity internals",
            },
            TocLink {
                href: "/docs/package",
                label: "Package",
            },
            TocLink {
                href: "/docs/api",
                label: "API reference",
            },
        ],
    },
];

pub fn in_toc(path: &str) -> bool {
    section_of(path).is_some()
}

fn section_of(path: &str) -> Option<&'static TocSection> {
    TOC.iter()
        .find(|section| section.links.iter().any(|link| link.href == path))
}

pub fn page_label(path: &str) -> String {
    if path == "/docs" {
        return "Docs".into();
    }
    let Some(section) = section_of(path) else {
        return "Docs".into();
    };
    let Some(link) = section.links.iter().find(|link| link.href == path) else {
        return section.title.to_string();
    };
    if link.label == "Overview" {
        section.title.to_string()
    } else {
        link.label.to_string()
    }
}

/// `(name, path)` from Home through the current guide.
pub fn crumb_trail(path: &str) -> Vec<(String, &'static str)> {
    let Some(section) = section_of(path) else {
        return Vec::new();
    };
    let Some(current) = section.links.iter().find(|link| link.href == path) else {
        return Vec::new();
    };
    let mut trail = vec![("Home".to_string(), "/")];
    if path != "/docs" {
        trail.push(("Docs".to_string(), "/docs"));
    }
    if let Some(hub) = section.hub {
        if hub != path && hub != "/docs" {
            trail.push((section.title.to_string(), hub));
        }
    }
    trail.push((page_label(path), current.href));
    trail
}

pub fn doc_crumbs(path: &str) -> View {
    let trail = crumb_trail(path);
    if trail.len() < 2 {
        return View::empty();
    }
    let last = trail.len() - 1;
    let mut children = Vec::new();
    for (index, (name, href)) in trail.into_iter().enumerate() {
        if index > 0 {
            children.push(Child::View(view! {
                <span class="docs-crumb-sep" aria-hidden="true">"/"</span>
            }));
        }
        if index == last {
            children.push(Child::View(view! {
                <span aria-current="page">{name}</span>
            }));
        } else {
            let href = href.to_string();
            children.push(Child::View(view! {
                <NavLink href={href} class="docs-crumb" activeClass="docs-crumb" exact=true>
                    {name}
                </NavLink>
            }));
        }
    }
    view! {
        <nav class="docs-crumbs" aria-label="Breadcrumb">
            {View::fragment(children)}
        </nav>
    }
}

pub fn doc_pager(path: &str) -> View {
    let pages: Vec<&'static TocLink> = TOC
        .iter()
        .flat_map(|section| section.links.iter())
        .collect();
    let Some(index) = pages.iter().position(|link| link.href == path) else {
        return View::empty();
    };
    let prev = index.checked_sub(1).map(|i| pages[i]);
    let next = pages.get(index + 1).copied();
    if prev.is_none() && next.is_none() {
        return View::empty();
    }
    let prev_view = prev.map_or_else(View::empty, pager_link);
    let next_view = next.map_or_else(View::empty, pager_link_next);
    view! {
        <nav class="docs-pager" aria-label="Previous and next">
            {prev_view}
            {next_view}
        </nav>
    }
}

fn pager_link(link: &'static TocLink) -> View {
    let href = link.href.to_string();
    let label = page_label(link.href);
    view! {
        <NavLink href={href} class="docs-pager-link" activeClass="docs-pager-link" exact=true>
            <small>"Previous"</small>
            <span>{label}</span>
        </NavLink>
    }
}

fn pager_link_next(link: &'static TocLink) -> View {
    let href = link.href.to_string();
    let label = page_label(link.href);
    view! {
        <NavLink href={href} class="docs-pager-link docs-pager-next" activeClass="docs-pager-link" exact=true>
            <small>"Next"</small>
            <span>{label}</span>
        </NavLink>
    }
}

pub fn nav_for(links: &'static [TocLink]) -> View {
    let children: Vec<Child> = links
        .iter()
        .map(|link| {
            let href = link.href.to_string();
            let label = link.label.to_string();
            Child::View(view! {
                <NavLink href={href} class="docs-nav-link" activeClass="docs-nav-link--active" exact=true>
                    {label}
                </NavLink>
            })
        })
        .collect();
    view! {
        <nav class="docs-nav">
            {View::fragment(children)}
        </nav>
    }
}
