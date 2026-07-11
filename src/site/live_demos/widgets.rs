//! Reusable interactive widgets for documentation live demos.

use resuma::prelude::*;
use serde::{Deserialize, Serialize};

#[component]
pub fn CounterWidget() -> View {
    let count = signal(0_i32);
    view! {
        <>
            <p class="demo-output">"Count: " {count}</p>
            <div class="demo-row">
                <button type="button" class="btn btn-sm" onClick={count.update(|c| *c -= 1)}>"−"</button>
                <button type="button" class="btn btn-sm" onClick={count.update(|c| *c += 1)}>"+"</button>
                <button type="button" class="btn btn-sm btn-ghost" onClick={count.set(0)}>"reset"</button>
            </div>
        </>
    }
}

#[component]
pub fn ShowWidget() -> View {
    let logged_in = signal(false);
    view! {
        <>
            <Show when={logged_in.get()}>
                <p class="demo-output">"Welcome back!"</p>
            </Show>
            <Show when={!logged_in.get()} fallback={view! { <p class="demo-muted">"Sign in to continue"</p> }}>
                <span></span>
            </Show>
            <button type="button" class="btn btn-sm" onClick={logged_in.update(|v| *v = !*v)}>
                <Show when={logged_in.get()}><span>"Logout"</span></Show>
                <Show when={!logged_in.get()}><span>"Login"</span></Show>
            </button>
        </>
    }
}

#[component]
pub fn EffectsWidget() -> View {
    let first = signal("Ada".to_string());
    let last = signal("Lovelace".to_string());
    let display = signal("Ada Lovelace".to_string());
    effect!([first, last, display], move || {
        display.set(format!("{} {}", first.get(), last.get()));
    });
    view! {
        <>
            <div class="demo-row">
                <input placeholder="First" onInput={js! { state.first.set(event.target.value); }} />
                <input placeholder="Last" onInput={js! { state.last.set(event.target.value); }} />
            </div>
            <p class="demo-output">"Full name: " {display}</p>
        </>
    }
}

#[component]
pub fn ErrorBoundaryWidget() -> View {
    let boom = signal(false);
    view! {
        <>
            <Show when={boom} fallback={view! { <p class="demo-muted">"All good — click to trigger"</p> }}>
                {resuma::error_boundary(Err::<View, &str>("Something broke!"), |e| {
                    view! { <p class="demo-error">{e}</p> }
                })}
            </Show>
            <button type="button" class="btn btn-sm btn-ghost" onClick={boom.set(true)}>"Trigger error"</button>
        </>
    }
}

#[component]
pub fn HandlersWidget() -> View {
    let clicked = signal(0_i32);
    view! {
        <>
            <p class="demo-output">"Clicks: " {clicked}</p>
            <button type="button" class="btn btn-sm" onClick={clicked.update(|c| *c += 1)}>"Click me"</button>
        </>
    }
}

#[island]
pub fn island_demo() -> View {
    let n = signal(0_i32);
    view! {
        <>
            <p class="demo-output">"Island counter: " {n}</p>
            <button type="button" class="btn btn-sm" onClick={n.update(|v| *v += 1)}>"+"</button>
        </>
    }
}

#[component]
pub fn ServerActionWidget() -> View {
    let result = signal(String::new());
    view! {
        <>
            <p class="demo-muted">
                "Real "
                <code>"#[server]"</code>
                " RPC via "
                <code>"__resuma.safeAction"</code>
                " — errors return "
                <code>"{ ok: false, error }"</code>
                " instead of throwing."
            </p>
            <div class="demo-row">
                <button type="button" class="btn btn-sm" onClick={js!(async () => {
                    const res = await __resuma.safeAction("docs_echo", ["Hello from docs"]);
                    state.result.set(res.ok ? res.value : res.error);
                })}>"docs_echo"</button>
                <button type="button" class="btn btn-sm btn-ghost" onClick={js!(async () => {
                    const res = await __resuma.safeAction("docs_add", [2, 40]);
                    state.result.set(res.ok ? "2 + 40 = " + res.value : res.error);
                })}>"docs_add(2,40)"</button>
            </div>
            <p class="demo-output">{result}</p>
        </>
    }
}

#[component]
pub fn WhoAmIWidget() -> View {
    let payload = signal(String::new());
    visible_task!(
        r#"
        async (state, __resuma) => {
            const res = await __resuma.safeAction("docs_whoami", []);
            state.payload.set(res.ok ? JSON.stringify(res.value, null, 2) : res.error);
        }
    "#,
        payload
    );
    view! {
        <>
            <p class="demo-muted">
                "Every "
                <code>"#[server]"</code>
                " action runs through "
                <code>"set_action_middleware"</code>
                " — switch user cookie and refresh."
            </p>
            <div class="demo-row todo-demo-user-row">
                <button type="button" class="btn btn-sm btn-ghost" onClick={js!(async () => {
                    document.cookie = "resuma_demo_user=guest; path=/; SameSite=Lax";
                    const res = await __resuma.safeAction("docs_whoami", []);
                    state.payload.set(res.ok ? JSON.stringify(res.value, null, 2) : res.error);
                })}>"guest"</button>
                <button type="button" class="btn btn-sm btn-ghost" onClick={js!(async () => {
                    document.cookie = "resuma_demo_user=alice; path=/; SameSite=Lax";
                    const res = await __resuma.safeAction("docs_whoami", []);
                    state.payload.set(res.ok ? JSON.stringify(res.value, null, 2) : res.error);
                })}>"alice"</button>
                <button type="button" class="btn btn-sm btn-ghost" onClick={js!(async () => {
                    document.cookie = "resuma_demo_user=bob; path=/; SameSite=Lax";
                    const res = await __resuma.safeAction("docs_whoami", []);
                    state.payload.set(res.ok ? JSON.stringify(res.value, null, 2) : res.error);
                })}>"bob"</button>
                <button
                    type="button"
                    class="btn btn-sm"
                    onClick={js!(async () => {
                        const res = await __resuma.safeAction("docs_whoami", []);
                        state.payload.set(res.ok ? JSON.stringify(res.value, null, 2) : res.error);
                    })}
                >
                    "Refresh"
                </button>
            </div>
            <pre class="code demo-output">{payload}</pre>
        </>
    }
}

#[component]
pub fn SecurityEnvWidget() -> View {
    let payload = signal(String::new());
    visible_task!(
        r#"
        async (state, __resuma) => {
            const res = await __resuma.safeAction("docs_runtime_security", []);
            state.payload.set(res.ok ? JSON.stringify(res.value, null, 2) : res.error);
        }
    "#,
        payload
    );
    view! {
        <>
            <p class="demo-muted">
                "Live snapshot from this server process (no secrets) — compare with "
                <code>"resuma doctor"</code>"."
            </p>
            <button
                type="button"
                class="btn btn-sm"
                onClick={js!(async () => {
                    const res = await __resuma.safeAction("docs_runtime_security", []);
                    state.payload.set(res.ok ? JSON.stringify(res.value, null, 2) : res.error);
                })}
            >
                "Refresh"
            </button>
            <pre class="code demo-output">{payload}</pre>
        </>
    }
}

#[component]
pub fn JsWidget() -> View {
    let msg = signal(String::new());
    view! {
        <>
            <input placeholder="Type here" onInput={js! { state.msg.set(event.target.value); }} />
            <p class="demo-output">"js! input: " {msg}</p>
        </>
    }
}

#[component]
fn SlotsDemoShell() -> View {
    view! {
        <div class="demo-slot-shell">
            <Slot name="header" />
            <Slot />
        </div>
    }
}

pub fn slots_widget() -> View {
    view! {
        <SlotsDemoShell>
            <h4 slot="header">"Header slot"</h4>
            <p>"Default slot body"</p>
        </SlotsDemoShell>
    }
}

pub fn nav_link_widget() -> View {
    view! {
        <div class="demo-row">
            <NavLink href="/docs/components/signals" activeClass="active">"Signals"</NavLink>
            <NavLink href="/docs/components/form" activeClass="active">"Form"</NavLink>
            <NavLink href="/docs/flow/loaders" activeClass="active">"Loaders"</NavLink>
        </div>
    }
}

#[component]
pub fn GreetFormWidget() -> View {
    let result = signal(String::new());
    visible_task!(
        r#"
        async () => {
            const form = document.getElementById("docs-greet-form");
            const out = document.getElementById("docs-greet-out");
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
                        out.textContent = Object.values(fields).join(" · ") || data.error || "Submit failed";
                        return;
                    }
                    out.textContent = data.value?.message ?? JSON.stringify(data.value);
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
            <Form submit={crate::site::demo_actions::docs_greet} id="docs-greet-form">
                <label>"Name" <input name="name" type="text" required=true /></label>
                <button type="submit" class="btn btn-sm">"Greet via #[submit]"</button>
            </Form>
            <p id="docs-greet-out" class="demo-output">{result}</p>
        </>
    }
}

#[component]
pub fn VisibleTaskWidget() -> View {
    let armed = signal(false);
    visible_task!(
        r#"
        async (state, __resuma) => {
            state.armed.set(true);
        }
    "#,
        armed
    );
    view! {
        <>
            <p class="demo-muted">"This panel uses " <code>"visible_task!"</code> " — the message below appears when the demo scrolls into view."</p>
            <Show
                when={armed.get()}
                fallback={view! { <p class="demo-output">"Waiting for viewport…"</p> }}
            >
                <p class="demo-output">"Visible task ran — deferred client work is active."</p>
            </Show>
        </>
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct UiStore {
    theme: String,
    count: i32,
}

#[component]
pub fn StoreWidget() -> View {
    let ui = use_store(UiStore {
        theme: "dark".into(),
        count: 0,
    });
    let label = signal("Theme: dark · Count: 0".to_string());
    view! {
        <>
            <p class="demo-output">{label}</p>
            <button type="button" class="btn btn-sm" onClick={js! {
                state.ui.update(s => { s.count += 1; });
                const u = state.ui.value;
                state.label.set("Theme: " + u.theme + " · Count: " + u.count);
            }}>"Increment store"</button>
        </>
    }
}

#[data]
struct LocaleCtx {
    lang: String,
}

static LOCALE: resuma::ContextId<LocaleCtx> = resuma::ContextId::new();

#[component]
fn LocaleProvider() -> View {
    provide_context(&LOCALE, LocaleCtx { lang: "es".into() });
    view! { <LocaleConsumer /> }
}

#[component]
fn LocaleConsumer() -> View {
    let locale = use_context(&LOCALE);
    view! { <p class="demo-output">"Context locale: " {locale.lang.clone()}</p> }
}

pub fn context_widget() -> View {
    view! { <LocaleProvider /> }
}

#[component]
pub fn DebounceWidget() -> View {
    let results = signal(String::new());
    view! {
        <>
            <input type="search" placeholder="Type 2+ chars…" onInput={js! {
                const q = event.target.value;
                clearTimeout(window.__docsDebounce);
                window.__docsDebounce = setTimeout(() => {
                    state.results.set(
                        q.length >= 2 ? "Debounced: '" + q + "'" : ""
                    );
                }, 300);
            }} />
            <p class="demo-output">{results}</p>
        </>
    }
}

#[component]
pub fn PortalWidget() -> View {
    let open = signal(false);
    view! {
        <>
            <button type="button" class="btn btn-sm" onClick={open.set(true)}>"Open modal"</button>
            <Show when={open}>
                {portal("modals", vec![Child::View(view! {
                    <div class="demo-modal-backdrop" onClick={open.set(false)}>
                        <div class="demo-modal" onClick={js! { event.stopPropagation(); }}>
                            <h4>"Portal modal"</h4>
                            <p>"Rendered via portal() into #modals"</p>
                            <button type="button" class="btn btn-sm" onClick={open.set(false)}>"Close"</button>
                        </div>
                    </div>
                })])}
            </Show>
        </>
    }
}

#[component]
pub fn ThemeWidget() -> View {
    let theme = Theme {
        mode: "dark".into(),
        primary: "#712cf9".into(),
        background: "#0b1020".into(),
        foreground: "#e6e8ee".into(),
    };
    provide_theme(theme.clone());
    view! {
        <div class="demo-theme-panel" style={theme_css_vars(&theme)}>
            <p>"Theme via provide_theme"</p>
            <button type="button" class="btn btn-sm btn-themed">"Themed button"</button>
        </div>
    }
}

#[component]
pub fn ReactivityWidget() -> View {
    let n = signal(0_i32);
    let doubled = signal(0_i32);
    effect!([n, doubled], move || {
        doubled.set(n.get() * 2);
    });
    view! {
        <>
            <p class="demo-output">"n = " {n} " → doubled = " {doubled}</p>
            <button type="button" class="btn btn-sm" onClick={n.update(|v| *v += 1)}>"Bump n"</button>
        </>
    }
}

#[component]
pub fn LoaderInvalidationWidget() -> View {
    let stamp = signal(String::new());
    let status = signal(String::new());
    visible_task!(
        r#"
        async (state, __resuma) => {
            const res = await __resuma.safeAction("docs_loader_stamp", []);
            if (res.ok) state.stamp.set(res.value);
        }
    "#,
        stamp
    );
    view! {
        <>
            <p class="demo-muted">
                "Server stamp via "
                <code>"#[server]"</code>
                " — invalidate SPA prefetch cache, then refresh."
            </p>
            <p class="demo-output">"Stamp: " {stamp}</p>
            <div class="demo-row">
                <button type="button" class="btn btn-sm" onClick={js!(async () => {
                    const res = await __resuma.safeAction("docs_loader_stamp", []);
                    state.stamp.set(res.ok ? res.value : res.error);
                    state.status.set(res.ok ? "Refreshed via action" : res.error);
                })}>"Refresh stamp"</button>
                <button type="button" class="btn btn-sm btn-ghost" onClick={js!(async () => {
                    await __resuma.invalidate("/docs/flow/loaders");
                    state.status.set("Invalidated /docs/flow/loaders prefetch");
                })}>"Invalidate loaders"</button>
                <button type="button" class="btn btn-sm btn-ghost" onClick={js!(async () => {
                    await __resuma.invalidate();
                    const res = await __resuma.safeAction("docs_loader_stamp", []);
                    state.stamp.set(res.ok ? res.value : res.error);
                    state.status.set("Invalidated current route + refreshed");
                })}>"Invalidate + refresh"</button>
            </div>
            <p class="demo-muted">{status}</p>
        </>
    }
}

#[component]
pub fn DeployInfoWidget() -> View {
    let payload = signal(String::new());
    visible_task!(
        r#"
        async (state, __resuma) => {
            const res = await __resuma.safeAction("docs_deploy_info", []);
            state.payload.set(res.ok ? JSON.stringify(res.value, null, 2) : res.error);
        }
    "#,
        payload
    );
    view! {
        <>
            <p class="demo-muted">"Live process env — same vars you set in Docker / Fly " <code>"fly.toml"</code>"."</p>
            <button type="button" class="btn btn-sm" onClick={js!(async () => {
                const res = await __resuma.safeAction("docs_deploy_info", []);
                state.payload.set(res.ok ? JSON.stringify(res.value, null, 2) : res.error);
            })}>"Refresh"</button>
            <pre class="code demo-output">{payload}</pre>
        </>
    }
}

#[component]
pub fn ViewTransitionsWidget() -> View {
    view! {
        <>
            <p class="demo-muted">"Click navigate — route transition uses " <code>"data-r-vt"</code> " (slide)."</p>
            <div class="demo-row">
                <NavLink href="/docs/cookbook/theme" activeClass="active" data-r-vt="slide">"→ Theme"</NavLink>
                <NavLink href="/docs/cookbook/portals" activeClass="active" data-r-vt="slide">"→ Portals"</NavLink>
                <NavLink href="/docs/cookbook/debouncer" activeClass="active" data-r-vt="slide">"→ Debouncer"</NavLink>
            </div>
        </>
    }
}

#[component]
pub fn PipelineWidget() -> View {
    let step = signal(0_i32);
    let label = signal("1. SSR render".to_string());
    view! {
        <>
            <p class="demo-output">{label}</p>
            <div class="demo-row">
                <button type="button" class="btn btn-sm" onClick={js! {
                    const s = (state.step.value + 1) % 5;
                    state.step.set(s);
                    const labels = [
                        "1. SSR render",
                        "2. Serialize signals",
                        "3. Embed payload",
                        "4. Loader resumes",
                        "5. Client reactive",
                    ];
                    state.label.set(labels[s]);
                }}>"Next step"</button>
                <button type="button" class="btn btn-sm btn-ghost" onClick={js! {
                    state.step.set(0);
                    state.label.set("1. SSR render");
                }}>"Reset"</button>
            </div>
        </>
    }
}
