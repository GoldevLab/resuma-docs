//! Minimal server-backed todo demo for the security todo reference page.

use resuma::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Mutex, OnceLock};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocsTodoItem {
    pub id: u64,
    pub title: String,
    pub done: bool,
}

static TODOS: OnceLock<Mutex<Vec<DocsTodoItem>>> = OnceLock::new();

fn todos() -> &'static Mutex<Vec<DocsTodoItem>> {
    TODOS.get_or_init(|| Mutex::new(Vec::new()))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocsTodoList {
    pub items: Vec<DocsTodoItem>,
}

#[server]
async fn docs_todo_list() -> Result<DocsTodoList> {
    let items = todos().lock().unwrap().clone();
    Ok(DocsTodoList { items })
}

#[server]
async fn docs_todo_add(title: String) -> Result<DocsTodoList> {
    let title = title.trim().to_string();
    if title.is_empty() {
        return Err(ResumaError::validation("title required"));
    }
    let mut guard = todos().lock().unwrap();
    let id = guard.iter().map(|t| t.id).max().unwrap_or(0) + 1;
    guard.push(DocsTodoItem {
        id,
        title,
        done: false,
    });
    Ok(DocsTodoList {
        items: guard.clone(),
    })
}

#[server]
async fn docs_todo_toggle(id: u64) -> Result<DocsTodoList> {
    let mut guard = todos().lock().unwrap();
    let item = guard
        .iter_mut()
        .find(|t| t.id == id)
        .ok_or_else(|| ResumaError::validation("todo not found"))?;
    item.done = !item.done;
    Ok(DocsTodoList {
        items: guard.clone(),
    })
}

/// Server-backed todo list — auth/validation patterns from the security todo guide.
#[component]
pub fn TodoDemoWidget() -> View {
    let items = signal(Vec::<DocsTodoItem>::new());
    let status = signal(String::new());

    visible_task!(r#"
        async (state, __resuma) => {
            const res = await __resuma.safeAction("docs_todo_list", []);
            if (res.ok) state.items.set(res.value.items);
            else state.status.set(res.error);
        }
    "#);

    view! {
        <div class="todo-demo">
            <p class="demo-muted">
                "In-memory todo list via "
                <code>"#[server]"</code>
                " — illustrates guarded mutations (see page for production patterns)."
            </p>
            <div class="demo-row">
                <input id="todo-title" type="text" placeholder="New task" />
                <button
                    type="button"
                    class="btn btn-sm btn-primary"
                    onClick={js!(async () => {
                        const input = document.getElementById("todo-title");
                        const title = input?.value ?? "";
                        const res = await __resuma.safeAction("docs_todo_add", [title]);
                        if (!res.ok) {
                            state.status.set(res.error);
                            return;
                        }
                        state.status.set("");
                        state.items.set(res.value.items);
                        if (input) input.value = "";
                    })}
                >
                    "Add"
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
                                if (res.ok) state.items.set(res.value.items);
                            })}
                        >
                            {if item.done { "Undo" } else { "Done" }}
                        </button>
                        <span>{item.title.clone()}</span>
                    </li>
                </For>
            </ul>
            <p class="demo-error" role="alert">{status}</p>
        </div>
    }
}
