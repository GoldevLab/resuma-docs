use resuma::prelude::*;

/// Old `/docs/cookbook/docker` URL — permanent redirect to the deploy guide.
pub fn page(_req: FlowRequest) -> View {
    let origin = crate::site::site_url();
    let origin = origin.trim().trim_end_matches('/');
    stage_response_status(301);
    stage_response_redirect(format!("{origin}/docs/cookbook/deploy"));
    View::empty()
}
