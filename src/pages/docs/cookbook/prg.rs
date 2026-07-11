use crate::site::code_block;
use resuma::prelude::*;

pub fn page(req: FlowRequest) -> View {
    view! {
        <>
            <h1>"PRG pattern"</h1>
            <p class="lead">"Post/Redirect/Get — avoid duplicate form submissions after " <code>"#[submit]"</code>"."</p>

            {crate::site::demos::cookbook_prg(&req)}

            <h2>"Why"</h2>
            <p>"After a successful POST, redirect to a GET URL so refresh does not re-submit the form."</p>

            <h2>"Return a redirect"</h2>
            {code_block(r#"#[submit]
async fn create_item(form: ItemForm, _req: &FlowRequest) -> Result<Redirect, SubmitError> {
    db::insert(&form).await.map_err(|_| SubmitError::new("Failed"))?;
    Ok(redirect("/items"))
}

// Flash message on the target page (query param `flash=…`):
Ok(redirect_with_flash("/items", "Item created"))

// Or any serializable struct with a `redirect` field:
#[data]
struct CreateResult { redirect: String }

#[submit]
async fn create_alt(form: ItemForm, _req: &FlowRequest) -> Result<CreateResult, SubmitError> {
    Ok(CreateResult { redirect: "/items".into() })
}"#)}

            <h2>"Behavior"</h2>
            <ul>
                <li><strong>"With JavaScript"</strong> " — runtime reads " <code>"redirect"</code> " from the JSON response and navigates via SPA fetch (same-origin paths)."</li>
                <li><strong>"Without JavaScript"</strong> " — server responds with " <strong>"303 See Other"</strong> " and " <code>"Location"</code> " header."</li>
                <li><strong>"Security"</strong> " — only root-relative paths (" <code>"/items"</code> ") are allowed; open redirects are rejected."</li>
            </ul>

            <h2>"Flash messages"</h2>
            <p>
                "Use "
                <code>"redirect_with_flash(path, message)"</code>
                " — the target page reads it with "
                <code>"flash_message(&req)"</code>
                ". See also "
                <a href="/docs/flow/query_params">"Query params"</a>
                " for manual flags like "
                <code>"?created=1"</code>
                "."
            </p>
        </>
    }
}
