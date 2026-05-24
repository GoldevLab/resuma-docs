//! Shared UI for the Resuma documentation site.

mod css;
mod docs_search;
mod hero_bg;
mod pwa;
mod seo;
mod server_demo;
mod sidebar;

pub use css::SITE_CSS;
pub use docs_search::search;
pub use hero_bg::hero_particles_mount;
pub use pwa::config as pwa_config;
pub use seo::{json_ld, site_description, site_title, site_url};
pub use server_demo::server_function_demo;
pub use sidebar::doc_sidebar;

use resuma::prelude::*;

pub fn code_block(code: &str) -> View {
    view! {
        <pre class="code"><code>{code.to_string()}</code></pre>
    }
}

pub fn playground_card(title: &str, body: &str, command: &str) -> View {
    view! {
        <article class="playground-card">
            <h3>{title.to_string()}</h3>
            <p>{body.to_string()}</p>
            <code>{command.to_string()}</code>
        </article>
    }
}

pub fn feature_card(icon: &str, title: &str, body: &str) -> View {
    view! {
        <article class="card">
            <div class="card-icon">{icon.to_string()}</div>
            <h3>{title.to_string()}</h3>
            <p>{body.to_string()}</p>
        </article>
    }
}

pub fn pillar_card(icon: &str, title: &str, body: &str) -> View {
    view! {
        <article class="pillar">
            <div class="pillar-icon">{icon.to_string()}</div>
            <h3>{title.to_string()}</h3>
            <p>{body.to_string()}</p>
        </article>
    }
}

pub fn pipeline_step(num: &str, title: &str, body: &str) -> View {
    view! {
        <article class="pipeline-step">
            <span class="pipeline-num">{num.to_string()}</span>
            <h3>{title.to_string()}</h3>
            <p>{body.to_string()}</p>
        </article>
    }
}

pub fn metric_item(value: &str, label: &str) -> View {
    view! {
        <div class="metric-item">
            <strong>{value.to_string()}</strong>
            <span>{label.to_string()}</span>
        </div>
    }
}

pub fn compare_column(title: &str, body: &str, highlight: bool) -> View {
    let class = if highlight {
        "compare-column compare-column-highlight"
    } else {
        "compare-column"
    };
    view! {
        <article class={class}>
            <h3>{title.to_string()}</h3>
            <p>{body.to_string()}</p>
        </article>
    }
}

pub fn bench_row_full(
    framework: &str,
    initial: &str,
    first_click: &str,
    statik: &str,
    highlight: bool,
) -> View {
    let class = if highlight {
        "bench-row bench-row-highlight"
    } else {
        "bench-row"
    };
    let static_class = if statik == "0 B" { "yes" } else { "" };
    view! {
        <tr class={class}>
            <td><strong>{framework.to_string()}</strong></td>
            <td>{initial.to_string()}</td>
            <td>{first_click.to_string()}</td>
            <td class={static_class}>{statik.to_string()}</td>
        </tr>
    }
}

pub fn doc_link_card(href: &str, title: &str, body: &str, tag: &str) -> View {
    if tag.is_empty() {
        return view! {
            <a href={href.to_string()} class="doc-link-card">
                <h3>{title.to_string()}</h3>
                <p>{body.to_string()}</p>
                <span class="doc-card-arrow">"→"</span>
            </a>
        };
    }
    view! {
        <a href={href.to_string()} class="doc-link-card">
            <span class="doc-card-tag">{tag.to_string()}</span>
            <h3>{title.to_string()}</h3>
            <p>{body.to_string()}</p>
            <span class="doc-card-arrow">"→"</span>
        </a>
    }
}

pub fn learn_path_card(step: &str, title: &str, body: &str, href: &str, label: &str) -> View {
    view! {
        <article class="learn-path-card">
            <span class="learn-path-step">{step.to_string()}</span>
            <h3>{title.to_string()}</h3>
            <p>{body.to_string()}</p>
            <a href={href.to_string()} class="learn-path-link">{label.to_string()}</a>
        </article>
    }
}
