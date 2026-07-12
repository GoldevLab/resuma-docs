//! Server-backed todo demo — session, validation, and row-level checks.

use resuma::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Mutex, OnceLock};

use super::todo_security;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocsTodoItem {
    pub id: u64,
    pub owner_id: String,
    pub title: String,
    pub done: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocsTodoList {
    pub user: String,
    pub is_admin: bool,
    pub items: Vec<DocsTodoItem>,
}

static TODOS: OnceLock<Mutex<Vec<DocsTodoItem>>> = OnceLock::new();

fn todos() -> &'static Mutex<Vec<DocsTodoItem>> {
    TODOS.get_or_init(|| {
        Mutex::new(vec![
            DocsTodoItem {
                id: 1,
                owner_id: "alice".into(),
                title: "Review security middleware".into(),
                done: true,
            },
            DocsTodoItem {
                id: 2,
                owner_id: "guest".into(),
                title: "Add a task — owned by current user".into(),
                done: false,
            },
            DocsTodoItem {
                id: 3,
                owner_id: "bob".into(),
                title: "Bob's task — guest cannot toggle".into(),
                done: false,
            },
        ])
    })
}

fn list_for(req: &FlowRequest) -> DocsTodoList {
    let user = todo_security::session_user(req);
    let is_admin = todo_security::admin_users().iter().any(|a| a == &user);
    let guard = todos().lock().unwrap();
    let items: Vec<_> = todo_security::list_visible(&guard, req, |t| &t.owner_id)
        .into_iter()
        .cloned()
        .collect();
    DocsTodoList {
        user,
        is_admin,
        items,
    }
}

#[server]
async fn docs_todo_list(req: &FlowRequest) -> Result<DocsTodoList> {
    Ok(list_for(req))
}

#[server]
async fn docs_todo_add(title: String, req: &FlowRequest) -> Result<DocsTodoList> {
    let title = todo_security::normalize_title(&title)?;
    let owner = todo_security::session_user(req);
    let mut guard = todos().lock().unwrap();
    let id = guard.iter().map(|t| t.id).max().unwrap_or(0) + 1;
    guard.push(DocsTodoItem {
        id,
        owner_id: owner,
        title,
        done: false,
    });
    drop(guard);
    Ok(list_for(req))
}

#[server]
async fn docs_todo_toggle(id: u64, req: &FlowRequest) -> Result<DocsTodoList> {
    let mut guard = todos().lock().unwrap();
    let owner = guard
        .iter()
        .find(|t| t.id == id)
        .map(|t| t.owner_id.clone())
        .ok_or_else(|| ResumaError::validation("todo not found"))?;
    todo_security::assert_owner(&owner, req)?;
    if let Some(item) = guard.iter_mut().find(|t| t.id == id) {
        item.done = !item.done;
    }
    drop(guard);
    Ok(list_for(req))
}

#[server]
async fn docs_whoami(req: &FlowRequest) -> Result<serde_json::Value> {
    Ok(serde_json::json!({
        "user_id": req.user_id(),
        "roles": req.extension("roles"),
        "authenticated": req.extension("authenticated"),
    }))
}

#[server]
async fn docs_runtime_security() -> Result<serde_json::Value> {
    let env = std::env::var("RESUMA_ENV").unwrap_or_else(|_| "development".into());
    Ok(serde_json::json!({
        "resuma_env": env,
        "trust_proxy": std::env::var("RESUMA_TRUST_PROXY").unwrap_or_default(),
        "rate_backend": std::env::var("RESUMA_RATE_BACKEND").unwrap_or_else(|_| {
            if matches!(env.as_str(), "production" | "prod") {
                "disk (auto)".into()
            } else {
                "memory (default)".into()
            }
        }),
        "csrf": "on (default)",
        "origin_check": "on (default)",
    }))
}

/// Server-backed todo list with guest / alice / bob session demo.
#[component]
pub fn TodoDemoWidget() -> View {
    let items = signal(Vec::<DocsTodoItem>::new());
    let user = signal("guest".to_string());
    let is_admin = signal(false);
    let status = signal(String::new());
    let error = signal(String::new());
    let has_error = computed!([error], move || !error.get().is_empty());

    visible_task!(
        r#"
        async (state, __resuma) => {
            const res = await __resuma.safeAction("docs_todo_list", []);
            if (res.ok) {
                state.items.set(res.value.items);
                state.user.set(res.value.user);
                state.is_admin.set(res.value.is_admin);
                state.status.set("");
                state.error.set("");
            } else {
                state.error.set(res.error);
            }
        }
    "#,
        items,
        user,
        is_admin,
        status,
        error
    );

    view! {
        <div class="todo-demo">
            <p class="demo-muted">
                "Switch user (cookie), add tasks, toggle another user's task for "
                <code>"403 Forbidden"</code> ", or "
                <code>"Fail validation"</code> " for "
                <code>"422"</code> "."
            </p>
            <div class="todo-demo-users">
                <span class="todo-demo-users__label">"Signed in as:"</span>
                <strong class="todo-demo-users__current">{user}</strong>
                <Show when={is_admin}>
                    <span class="todo-demo-users__badge">"admin"</span>
                </Show>
            </div>
            <div class="demo-row todo-demo-user-row">
                <button type="button" class="btn btn-sm btn-ghost" onClick={js!(async () => {
                    document.cookie = "resuma_demo_user=guest; path=/; SameSite=Lax";
                    const res = await __resuma.safeAction("docs_todo_list", []);
                    if (!res.ok) { state.error.set(res.error); return; }
                    state.items.set(res.value.items);
                    state.user.set(res.value.user);
                    state.is_admin.set(res.value.is_admin);
                    state.error.set("");
                    state.status.set("guest — sees only own tasks");
                })}>"guest"</button>
                <button type="button" class="btn btn-sm btn-ghost" onClick={js!(async () => {
                    document.cookie = "resuma_demo_user=alice; path=/; SameSite=Lax";
                    const res = await __resuma.safeAction("docs_todo_list", []);
                    if (!res.ok) { state.error.set(res.error); return; }
                    state.items.set(res.value.items);
                    state.user.set(res.value.user);
                    state.is_admin.set(res.value.is_admin);
                    state.error.set("");
                    state.status.set("alice — admin, sees all tasks");
                })}>"alice"</button>
                <button type="button" class="btn btn-sm btn-ghost" onClick={js!(async () => {
                    document.cookie = "resuma_demo_user=bob; path=/; SameSite=Lax";
                    const res = await __resuma.safeAction("docs_todo_list", []);
                    if (!res.ok) { state.error.set(res.error); return; }
                    state.items.set(res.value.items);
                    state.user.set(res.value.user);
                    state.is_admin.set(res.value.is_admin);
                    state.error.set("");
                    state.status.set("bob — regular user");
                })}>"bob"</button>
            </div>
            <div class="demo-row todo-demo-add-row">
                <input id="todo-title" type="text" placeholder="New task" aria-describedby="todo-demo-error" />
                <button
                    type="button"
                    class="btn btn-sm btn-primary"
                    onClick={js!(async () => {
                        const input = document.getElementById("todo-title");
                        const title = input?.value ?? "";
                        const res = await __resuma.safeAction("docs_todo_add", [title]);
                        if (!res.ok) {
                            state.error.set(res.error);
                            state.status.set("");
                            input?.classList.add("input-error");
                            return;
                        }
                        state.error.set("");
                        state.status.set("Task added");
                        state.items.set(res.value.items);
                        state.user.set(res.value.user);
                        input?.classList.remove("input-error");
                        if (input) input.value = "";
                    })}
                >
                    "Add"
                </button>
                <button
                    type="button"
                    class="btn btn-sm btn-ghost"
                    title="POST empty title — expect 422 Invalid request"
                    onClick={js!(async () => {
                        const res = await __resuma.safeAction("docs_todo_add", [""]);
                        if (res.ok) {
                            state.error.set("unexpected ok");
                            state.status.set("");
                            return;
                        }
                        state.error.set(res.error);
                        state.status.set("");
                        document.getElementById("todo-title")?.classList.add("input-error");
                    })}
                >
                    "Fail validation"
                </button>
            </div>
            <ul class="todo-demo-list">
                <For each={items} let:item>
                    <li class={(if item.done { "todo-demo-item todo-demo-done" } else { "todo-demo-item" }).to_string()}>
                        <button
                            type="button"
                            class="btn btn-sm btn-ghost"
                            onClick={js!(async () => {
                                const res = await __resuma.safeAction("docs_todo_toggle", [item.id]);
                                if (!res.ok) {
                                    state.error.set(res.error);
                                    state.status.set("");
                                    return;
                                }
                                state.error.set("");
                                state.status.set("");
                                state.items.set(res.value.items);
                            })}
                        >
                            {if item.done { "Undo" } else { "Done" }}
                        </button>
                        <span class="todo-demo-owner">{item.owner_id.clone()}</span>
                        <span>{item.title.clone()}</span>
                    </li>
                </For>
            </ul>
            <Show when={has_error}>
                <p id="todo-demo-error" class="demo-alert demo-alert--error" role="alert">{error}</p>
            </Show>
            <p class="demo-status">{status}</p>
            <p class="demo-hint">
                "Production (" <code>"RESUMA_ENV=production"</code> "): validation returns "
                <code>"Invalid request"</code> " — not internal field names."
            </p>
        </div>
    }
}
