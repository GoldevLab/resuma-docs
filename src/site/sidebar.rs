//! Full docs navigation — mapped to Resuma APIs.

use resuma::prelude::*;

use super::toc::{nav_for, TOC};
use crate::site::DocsCopyNavBtn;

pub fn doc_sidebar(_active_path: &str) -> View {
    view! {
        <>
            {crate::site::browse_docs_chip()}
        <aside id="docs-browse" class="docs-sidebar" data-r-nav-exclusive="true">
            <form method="get" action="/docs/search" class="docs-search-form">
                <input type="search" name="q" placeholder="Search docs..." aria-label="Search docs" />
            </form>
            <DocsCopyNavBtn />
            <div class="docs-sidebar-scroll">
                {sidebar_sections()}
                <section class="docs-sidebar-section docs-sidebar-section--resources">
                    <h4>"Resources"</h4>
                    <nav class="docs-nav">
                        <a href="https://crates.io/crates/resuma" target="_blank" rel="noopener noreferrer" class="docs-nav-link docs-nav-link--external">"crates.io"</a>
                        <a href="https://docs.rs/resuma" target="_blank" rel="noopener noreferrer" class="docs-nav-link docs-nav-link--external">"docs.rs"</a>
                        <a href="https://github.com/GoldevLab/resuma" target="_blank" rel="noopener noreferrer" class="docs-nav-link docs-nav-link--external">"GitHub"</a>
                    </nav>
                </section>
            </div>
        </aside>
        </>
    }
}

fn sidebar_sections() -> View {
    let sections: Vec<Child> = TOC
        .iter()
        .map(|section| {
            let title = section.title.to_string();
            let links = nav_for(section.links);
            Child::View(view! {
                <section class="docs-sidebar-section">
                    <h4>{title}</h4>
                    {links}
                </section>
            })
        })
        .collect();
    View::fragment(sections)
}
