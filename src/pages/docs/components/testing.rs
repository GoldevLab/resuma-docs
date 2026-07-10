use crate::site::code_block;
use resuma::prelude::*;

pub fn page(_req: FlowRequest) -> View {
    view! {
        <>
            <h1>"Testing"</h1>
            <p class="lead">"Strategies for verifying Resuma apps — HTTP integration, loader logic, and E2E."</p>

            {crate::site::demos::components_testing()}

            <h2>"Integration tests (recommended)"</h2>
            <p>"Test the axum router produced by " <code>"FlowApp::into_router()"</code> ":"</p>
            {code_block(r#"use axum::body::Body;
use axum::http::{Request, StatusCode};
use resuma::prelude::*;
use tower::ServiceExt;

#[tokio::test]
async fn home_returns_200() {
    let app = FlowApp::new()
        .page("/", || view! { <h1>"Hi"</h1> })
        .into_router(FlowServeOptions::default());

    let res = app
        .oneshot(Request::get("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);
}"#)}

            <h2>"Submit + redirect"</h2>
            {code_block(r#"#[submit]
async fn create(_form: Form, _req: &FlowRequest) -> Result<Redirect, SubmitError> {
    Ok(redirect("/items"))
}

// JSON client: { ok: true, redirect: "/items" }
// No-JS browser: 303 See Other"#)}

            <h2>"Unit tests"</h2>
            <p>
                "Test pure Rust helpers, route matching, and redirect validation without HTTP. "
                "The " <code>"resuma"</code> " crate includes unit tests for "
                <code>"match_route"</code> ", cache merge, CSP nonce injection, and "
                <code>"validate_redirect_path"</code> "."
            </p>

            <h2>"Render snapshots"</h2>
            {code_block(r#"use resuma::{render_view, PageOptions};

#[test]
fn counter_markup() {
    let html = render_view(&PageOptions::default(), || {
        view! { <button>"+1"</button> }
    });
    assert!(html.contains("button"));
}"#)}

            <h2>"End-to-end"</h2>
            <p>
                "See " <a href="/docs/integrations/e2e">"E2E testing"</a> " for Playwright against a running "
                <code>"resuma dev"</code> " server. Assert on SSR HTML, then click to verify resumable handlers load."
            </p>

            <h2>"What we don't ship yet"</h2>
            <p>
                "There is no Leptos-style in-process component mount harness. For UI regressions, prefer "
                "HTTP/E2E tests or snapshot " <code>"render_view"</code> " output."
            </p>
        </>
    }
}
