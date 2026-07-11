//! Live integration demos — mock backends that exercise real Resuma patterns.

use resuma::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Mutex, OnceLock};

use super::todo_security;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MockUser {
    pub id: i64,
    pub name: String,
    pub email: String,
}

static MOCK_USERS: OnceLock<Mutex<Vec<MockUser>>> = OnceLock::new();

fn users() -> &'static Mutex<Vec<MockUser>> {
    MOCK_USERS.get_or_init(|| {
        Mutex::new(vec![
            MockUser {
                id: 1,
                name: "Ada Lovelace".into(),
                email: "ada@example.com".into(),
            },
            MockUser {
                id: 2,
                name: "Grace Hopper".into(),
                email: "grace@example.com".into(),
            },
            MockUser {
                id: 3,
                name: "Margaret Hamilton".into(),
                email: "mh@example.com".into(),
            },
        ])
    })
}

#[server]
async fn docs_mock_sqlx_list() -> Result<Vec<MockUser>> {
    Ok(users().lock().unwrap().clone())
}

#[server]
async fn docs_mock_turso_list() -> Result<serde_json::Value> {
    let rows = users().lock().unwrap().clone();
    Ok(serde_json::json!({
        "backend": "libsql (mock)",
        "replica": "iad",
        "rows": rows,
    }))
}

#[server]
async fn docs_mock_supabase_list(req: &FlowRequest) -> Result<serde_json::Value> {
    let uid = todo_security::session_user(req);
    let is_admin = todo_security::admin_users().iter().any(|a| a == &uid);
    let rows: Vec<_> = users()
        .lock()
        .unwrap()
        .iter()
        .filter(|u| is_admin || u.id == 2 || uid == "guest" && u.id <= 2)
        .cloned()
        .collect();
    Ok(serde_json::json!({
        "rls": if is_admin { "bypass (service role)" } else { "anon policy" },
        "user": uid,
        "rows": rows,
    }))
}

#[server]
async fn docs_scaffold_preview(name: String) -> Result<Vec<String>> {
    let files = match name.as_str() {
        "sqlx" => vec![
            "src/db.rs — PgPool + init_db()".into(),
            "migrations/001_init.sql".into(),
            "Cargo.toml — sqlx + tokio".into(),
        ],
        "turso" => vec![
            "src/db.rs — libsql::Builder".into(),
            ".env — TURSO_DATABASE_URL".into(),
            "Cargo.toml — libsql-client".into(),
        ],
        "auth" => vec![
            "src/security.rs — session middleware".into(),
            "src/pages/login.rs".into(),
            "Cargo.toml — cookie + hmac".into(),
        ],
        "tailwind" => vec![
            "tailwind.config.js".into(),
            "assets/input.css".into(),
            "build.rs — tailwind compile hook".into(),
        ],
        _ => vec![format!(
            "resuma add {name} — scaffolds integration boilerplate"
        )],
    };
    Ok(files)
}

#[server]
async fn docs_og_preview() -> Result<serde_json::Value> {
    let base = super::seo::site_url().trim_end_matches('/').to_string();
    Ok(serde_json::json!({
        "og:title": super::seo::site_title(),
        "og:description": super::seo::site_description(),
        "og:image": format!("{base}/og.svg"),
        "twitter:card": "summary_large_image",
    }))
}

#[server]
async fn docs_i18n_translate(lang: String) -> Result<serde_json::Value> {
    let (title, lead) = match lang.as_str() {
        "es" => (
            "Documentación de Resuma",
            "Framework SSR en Rust — sin hidratación",
        ),
        "fr" => (
            "Documentation Resuma",
            "Framework SSR Rust — sans hydratation",
        ),
        _ => ("Resuma Documentation", "Rust SSR framework — no hydration"),
    };
    Ok(serde_json::json!({ "lang": lang, "title": title, "lead": lead }))
}

#[component]
pub fn SqlxDemoWidget() -> View {
    let output = signal(String::new());
    let status = signal(String::new());
    visible_task!(
        r#"
        async (state, __resuma) => {
            const res = await __resuma.safeAction("docs_mock_sqlx_list", []);
            state.output.set(res.ok ? JSON.stringify(res.value, null, 2) : res.error);
            state.status.set(res.ok ? "ok" : "error");
        }
    "#,
        output,
        status
    );
    view! {
        <>
            <p class="demo-muted">"In-memory mock of sqlx::query_as! — real apps use DATABASE_URL + pool."</p>
            <button type="button" class="btn btn-sm" onClick={js!(async () => {
                const res = await __resuma.safeAction("docs_mock_sqlx_list", []);
                state.output.set(res.ok ? JSON.stringify(res.value, null, 2) : res.error);
                state.status.set(res.ok ? "Refreshed" : res.error);
            })}>"Run query"</button>
            <p class="demo-muted">{status}</p>
            <pre class="code demo-output">{output}</pre>
        </>
    }
}

#[component]
pub fn TursoDemoWidget() -> View {
    let output = signal(String::new());
    visible_task!(
        r#"
        async (state, __resuma) => {
            const res = await __resuma.safeAction("docs_mock_turso_list", []);
            state.output.set(res.ok ? JSON.stringify(res.value, null, 2) : res.error);
        }
    "#,
        output
    );
    view! {
        <>
            <p class="demo-muted">"Mock edge SQLite — response includes replica metadata like libSQL."</p>
            <button type="button" class="btn btn-sm" onClick={js!(async () => {
                const res = await __resuma.safeAction("docs_mock_turso_list", []);
                state.output.set(res.ok ? JSON.stringify(res.value, null, 2) : res.error);
            })}>"Run query"</button>
            <pre class="code demo-output">{output}</pre>
        </>
    }
}

#[component]
pub fn SupabaseDemoWidget() -> View {
    let output = signal(String::new());
    visible_task!(
        r#"
        async (state, __resuma) => {
            const res = await __resuma.safeAction("docs_mock_supabase_list", []);
            state.output.set(res.ok ? JSON.stringify(res.value, null, 2) : res.error);
        }
    "#,
        output
    );
    view! {
        <>
            <p class="demo-muted">
                "Mock PostgREST + RLS — switch user on "
                <a href="/docs/security/todo">"Security todo"</a>
                " cookie, then refresh."
            </p>
            <div class="demo-row todo-demo-user-row">
                <button type="button" class="btn btn-sm btn-ghost" onClick={js!(async () => {
                    document.cookie = "resuma_demo_user=guest; path=/; SameSite=Lax";
                    const res = await __resuma.safeAction("docs_mock_supabase_list", []);
                    state.output.set(res.ok ? JSON.stringify(res.value, null, 2) : res.error);
                })}>"guest"</button>
                <button type="button" class="btn btn-sm btn-ghost" onClick={js!(async () => {
                    document.cookie = "resuma_demo_user=alice; path=/; SameSite=Lax";
                    const res = await __resuma.safeAction("docs_mock_supabase_list", []);
                    state.output.set(res.ok ? JSON.stringify(res.value, null, 2) : res.error);
                })}>"alice"</button>
                <button type="button" class="btn btn-sm btn-ghost" onClick={js!(async () => {
                    document.cookie = "resuma_demo_user=bob; path=/; SameSite=Lax";
                    const res = await __resuma.safeAction("docs_mock_supabase_list", []);
                    state.output.set(res.ok ? JSON.stringify(res.value, null, 2) : res.error);
                })}>"bob"</button>
                <button
                    type="button"
                    class="btn btn-sm"
                    onClick={js!(async () => {
                        const res = await __resuma.safeAction("docs_mock_supabase_list", []);
                        state.output.set(res.ok ? JSON.stringify(res.value, null, 2) : res.error);
                    })}
                >
                    "Refresh"
                </button>
            </div>
            <pre class="code demo-output">{output}</pre>
        </>
    }
}

#[component]
pub fn ScaffoldDemoWidget() -> View {
    let output = signal(String::new());
    view! {
        <>
            <p class="demo-muted">"Preview files " <code>"resuma add &lt;integration&gt;"</code> " would scaffold."</p>
            <div class="demo-row">
                <button type="button" class="btn btn-sm btn-ghost" onClick={js!(async () => {
                    const res = await __resuma.safeAction("docs_scaffold_preview", ["sqlx"]);
                    state.output.set(res.ok ? res.value.join("\n") : res.error);
                })}>"sqlx"</button>
                <button type="button" class="btn btn-sm btn-ghost" onClick={js!(async () => {
                    const res = await __resuma.safeAction("docs_scaffold_preview", ["turso"]);
                    state.output.set(res.ok ? res.value.join("\n") : res.error);
                })}>"turso"</button>
                <button type="button" class="btn btn-sm btn-ghost" onClick={js!(async () => {
                    const res = await __resuma.safeAction("docs_scaffold_preview", ["auth"]);
                    state.output.set(res.ok ? res.value.join("\n") : res.error);
                })}>"auth"</button>
                <button type="button" class="btn btn-sm btn-ghost" onClick={js!(async () => {
                    const res = await __resuma.safeAction("docs_scaffold_preview", ["tailwind"]);
                    state.output.set(res.ok ? res.value.join("\n") : res.error);
                })}>"tailwind"</button>
            </div>
            <pre class="code demo-output">{output}</pre>
        </>
    }
}

#[component]
pub fn I18nDemoWidget() -> View {
    let payload = signal(String::new());
    view! {
        <>
            <p class="demo-muted">"Server-side locale strings via " <code>"#[server]"</code> " — swap lang, refresh copy."</p>
            <div class="demo-row">
                <button type="button" class="btn btn-sm btn-ghost" onClick={js!(async () => {
                    const res = await __resuma.safeAction("docs_i18n_translate", ["en"]);
                    state.payload.set(res.ok ? JSON.stringify(res.value, null, 2) : res.error);
                })}>"en"</button>
                <button type="button" class="btn btn-sm btn-ghost" onClick={js!(async () => {
                    const res = await __resuma.safeAction("docs_i18n_translate", ["es"]);
                    state.payload.set(res.ok ? JSON.stringify(res.value, null, 2) : res.error);
                })}>"es"</button>
                <button type="button" class="btn btn-sm btn-ghost" onClick={js!(async () => {
                    const res = await __resuma.safeAction("docs_i18n_translate", ["fr"]);
                    state.payload.set(res.ok ? JSON.stringify(res.value, null, 2) : res.error);
                })}>"fr"</button>
            </div>
            <pre class="code demo-output">{payload}</pre>
        </>
    }
}

#[component]
pub fn TailwindDemoWidget() -> View {
    let card_class = signal("tw-demo-card tw-demo-purple".to_string());
    let title = signal("rounded-lg · bg-purple-500".to_string());
    let body = signal("Tailwind utilities compile to static CSS at build time.".to_string());
    view! {
        <>
            <p class="demo-muted">"Utility-class preview — " <code>"resuma add tailwind"</code> " wires compile-time CSS."</p>
            <div class="demo-row">
                <button type="button" class="btn btn-sm" onClick={js! {
                    state.card_class.set("tw-demo-card tw-demo-purple");
                    state.title.set("rounded-lg · bg-purple-500");
                    state.body.set("Tailwind utilities compile to static CSS at build time.");
                }}>"purple"</button>
                <button type="button" class="btn btn-sm btn-ghost" onClick={js! {
                    state.card_class.set("tw-demo-card tw-demo-emerald");
                    state.title.set("rounded-lg · bg-emerald-500");
                    state.body.set("Purge unused classes — tiny production bundles.");
                }}>"emerald"</button>
                <button type="button" class="btn btn-sm btn-ghost" onClick={js! {
                    state.card_class.set("tw-demo-card tw-demo-rose");
                    state.title.set("rounded-lg · bg-rose-500");
                    state.body.set("Works with Resuma islands and Flow layouts.");
                }}>"rose"</button>
            </div>
            <div class={card_class}>
                <p class="tw-demo-title">{title}</p>
                <p class="tw-demo-body">{body}</p>
            </div>
        </>
    }
}

#[component]
pub fn OgImageDemoWidget() -> View {
    let meta = signal(String::new());
    visible_task!(
        r#"
        async (state, __resuma) => {
            const res = await __resuma.safeAction("docs_og_preview", []);
            state.meta.set(res.ok ? JSON.stringify(res.value, null, 2) : res.error);
        }
    "#,
        meta
    );
    view! {
        <>
            <p class="demo-muted">"Live OG tags this site emits — image at " <code>"/og.svg"</code>"."</p>
            <a href="/og.svg" target="_blank" class="btn btn-sm btn-ghost">"Open og.svg"</a>
            <pre class="code demo-output">{meta}</pre>
        </>
    }
}

#[component]
pub fn E2eDemoWidget() -> View {
    let result = signal(String::new());
    view! {
        <>
            <p class="demo-muted">"Ping server action — same RPC path Playwright exercises in CI."</p>
            <button
                type="button"
                class="btn btn-sm"
                onClick={js!(async () => {
                    const res = await __resuma.safeAction("docs_e2e_ping", []);
                    state.result.set(res.ok ? JSON.stringify(res.value, null, 2) : res.error);
                })}
            >
                "Run ping"
            </button>
            <pre class="code demo-output">{result}</pre>
        </>
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocsSignupForm {
    pub email: String,
    pub password: String,
}

#[submit]
pub async fn docs_signup(
    data: DocsSignupForm,
    _req: &FlowRequest,
) -> std::result::Result<serde_json::Value, SubmitError> {
    let mut err = SubmitError::new("Fix the errors below.");
    let mut has_err = false;
    if !data.email.contains('@') || data.email.len() < 5 {
        err = err.field("email", "Valid email required");
        has_err = true;
    }
    if data.password.len() < 8 {
        err = err.field("password", "Min 8 characters");
        has_err = true;
    }
    if has_err {
        return Err(err);
    }
    Ok(serde_json::json!({ "ok": true, "email": data.email.trim() }))
}

#[component]
pub fn ValidationDemoWidget() -> View {
    let result = signal(String::new());
    visible_task!(
        r#"
        async () => {
            const form = document.getElementById("docs-signup-form");
            const out = document.getElementById("docs-signup-out");
            if (!form || !out) return;
            const onSubmit = async (ev) => {
                if (ev.target !== form) return;
                ev.preventDefault();
                const fd = new FormData(form);
                const body = {};
                fd.forEach((v, k) => { body[k] = String(v); });
                const params = new URLSearchParams(body);
                try {
                    const res = await fetch(form.action, {
                        method: "POST",
                        credentials: "same-origin",
                        headers: {
                            "content-type": "application/x-www-form-urlencoded",
                            accept: "application/json",
                        },
                        body: params.toString(),
                    });
                    const data = await res.json();
                    if (!res.ok || data.ok === false) {
                        const fields = data.field_errors ?? {};
                        out.textContent = Object.entries(fields).map(([k,v]) => k + ": " + v).join(" · ") || data.error || "Validation failed";
                        return;
                    }
                    out.textContent = "Valid — " + JSON.stringify(data.value);
                } catch (e) {
                    out.textContent = String(e);
                }
            };
            form.addEventListener("submit", onSubmit);
            return () => form.removeEventListener("submit", onSubmit);
        }
    "#
    );
    view! {
        <>
            <p class="demo-muted">"Real " <code>"#[submit]"</code> " field errors — try invalid email or short password."</p>
            <Form submit={docs_signup} id="docs-signup-form">
                <label>"Email" <input name="email" type="email" placeholder="you@example.com" /></label>
                <label>"Password" <input name="password" type="password" placeholder="8+ chars" /></label>
                <button type="submit" class="btn btn-sm">"Sign up"</button>
            </Form>
            <p id="docs-signup-out" class="demo-output">{result}</p>
        </>
    }
}
