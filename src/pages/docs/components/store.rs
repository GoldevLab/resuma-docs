use crate::site::code_block;
use resuma::prelude::*;

pub fn page(_req: FlowRequest) -> View {
    view! {
        <>
            <h1>"Store"</h1>
            <p class="lead">"Stores wrap structured reactive state — mutations go through update or set, and the whole object serializes as one payload blob."</p>

            <h2>"use_store"</h2>
            {code_block(r#"#[data]
struct User {
    name: String,
    email: String,
}

let user = use_store(User {
    name: "Ada".into(),
    email: "ada@example.com".into(),
});

user.update(|u| u.name = "Augusta".into());
user.set(User { name: "New".into(), email: u.email.clone() });"#)}

            <h2>"#[derive(Store)]"</h2>
            <p>"Generate typed field accessors on " <code>"Store&lt;T&gt;"</code> " (1.0+):"</p>
            {code_block(r#"#[derive(Store)]
#[data]
struct Cart {
    items: u32,
    total_cents: u64,
}

let cart = use_store(Cart { items: 0, total_cents: 0 });
cart.set_items(2);
cart.update(|c| c.total_cents += 500);"#)}

            <h2>"In templates"</h2>
            {code_block(r#"view! {
    <p>{user.signal()}</p>
    <input value={user.get().name.clone()} />
}"#)}

            <h2>"NoSerialize"</h2>
            <p>"Mark fields that must not cross the resumability boundary — handles, callbacks, or non-serializable server state."</p>
            {code_block(r#"#[data]
struct AppState {
    pub count: u32,
    #[serde(skip)]
    pub db: NoSerialize<DbPool>,
}"#)}

            <h2>"Store vs Signal"</h2>
            <p>"Use " <code>"signal"</code> " for scalar values. Use " <code>"use_store"</code> " when you have structured objects with multiple fields that update together."</p>
        </>
    }
}
