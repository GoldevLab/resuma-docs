//! Live demo panel wrapper for documentation pages.

use resuma::prelude::*;

/// Interactive demo box shown at the top of each docs page.
pub fn live_demo(title: &str, body: View) -> View {
    view! {
        <section class="live-demo" aria-label="Live demo">
            <div class="live-demo-header">
                <span class="live-demo-badge">"LIVE"</span>
                <h2 class="live-demo-title">{title.to_string()}</h2>
            </div>
            <div class="live-demo-body">
                {body}
            </div>
        </section>
    }
}

/// Info-only panel when interactivity is limited (integrations, reference).
pub fn live_info(title: &str, body: View) -> View {
    view! {
        <section class="live-demo live-demo-info" aria-label="Live info">
            <div class="live-demo-header">
                <span class="live-demo-badge live-demo-badge-info">"TRY IT"</span>
                <h2 class="live-demo-title">{title.to_string()}</h2>
            </div>
            <div class="live-demo-body">
                {body}
            </div>
        </section>
    }
}
