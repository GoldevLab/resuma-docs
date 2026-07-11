use crate::site::code_block;
use resuma::prelude::*;

pub fn page(_req: FlowRequest) -> View {
    view! {
        <>
            <h1>"Actions (Submits)"</h1>
            <p class="lead">"#[submit] handlers process form POSTs with typed validation and field-level errors."</p>

            {crate::site::demos::flow_submits()}

            <h2>"Define a submit handler"</h2>
            <p>"The live demo above uses " <code>"docs_greet"</code> " — same pattern as below."</p>
            {code_block(r#"#[data]
struct GreetForm {
    name: String,
}

#[submit]
async fn docs_greet(form: GreetForm, _req: &FlowRequest)
    -> Result<GreetResult, SubmitError>
{
    if form.name.trim().is_empty() {
        return Err(SubmitError::new("Fix errors")
            .field("name", "Required"));
    }
    Ok(GreetResult { message: format!("Hello, {}!", form.name.trim()) })
}"#)}

            <h2>"Wire to Form"</h2>
            {code_block(r#"view! {
    <Form submit={docs_greet}>
        <input name="name" type="text" required />
        <button type="submit">"Greet"</button>
    </Form>
}"#)}

            <h2>"SubmitError"</h2>
            <p>"Return structured validation errors. The runtime maps field_errors to form fields when using client enhancement."</p>
            {code_block(r#"SubmitError::new("Invalid input")
    .field("email", "Must be a valid email")
    .field("message", "Too short")"#)}

            <h2>"HTTP endpoint"</h2>
            <p>"Forms POST to " <code>"/_resuma/submit/:name"</code> ". Works without JavaScript via standard form submission."</p>
        </>
    }
}
