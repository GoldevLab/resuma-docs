//! Shared “how to use this demo” copy for worker / Flow UI pages.

use resuma::prelude::*;

/// Step-by-step guide for Pause / Resume / Cancel on live worker demos.
pub fn worker_try_it_guide(guide_id: &str) -> View {
    let id = guide_id.to_string();
    view! {
        <div class="exec-demo-guide" data-guide={id.clone()} role="region" aria-label="How to use this demo">
            <p class="exec-demo-guide__title">"How to try Pause / Resume / Cancel"</p>
            <ol class="exec-demo-guide__steps">
                <li class="is-active" data-step="1">
                    <span class="exec-demo-guide__num" aria-hidden="true">"1"</span>
                    <span class="exec-demo-guide__text">
                        "Click "
                        <strong>"Run worker"</strong>
                        " — a real graph starts (~25 seconds)."
                    </span>
                </li>
                <li data-step="2">
                    <span class="exec-demo-guide__num" aria-hidden="true">"2"</span>
                    <span class="exec-demo-guide__text">
                        "Watch the panel below: under "
                        <strong>"Controls"</strong>
                        ", wait until status says "
                        <strong>"Running"</strong>
                        "."
                    </span>
                </li>
                <li data-step="3">
                    <span class="exec-demo-guide__num" aria-hidden="true">"3"</span>
                    <span class="exec-demo-guide__text">
                        "While "
                        <strong>"Running"</strong>
                        ", try "
                        <strong>"Pause"</strong>
                        " (stops at next step), "
                        <strong>"Resume"</strong>
                        ", or "
                        <strong>"Cancel"</strong>
                        "."
                    </span>
                </li>
                <li data-step="4">
                    <span class="exec-demo-guide__num" aria-hidden="true">"4"</span>
                    <span class="exec-demo-guide__text">
                        "When status shows "
                        <strong>"done"</strong>
                        ", controls turn off — click "
                        <strong>"Run worker"</strong>
                        " again to retry."
                    </span>
                </li>
            </ol>
            <p class="exec-demo-guide__status" data-guide-status aria-live="polite">
                "Ready — start with step 1."
            </p>
        </div>
    }
}

/// Shown until the execution panel mounts.
pub fn worker_panel_placeholder(placeholder_id: &str) -> View {
    view! {
        <div id={placeholder_id.to_string()} class="exec-flow-placeholder">
            <p class="exec-flow-placeholder__title">"Execution panel"</p>
            <p class="exec-flow-placeholder__text">
                "Graph, controls, and event stream appear here after you click "
                <strong>"Run worker"</strong>
                "."
            </p>
        </div>
    }
}
