//! Demo session + action middleware for the security todo live demo.

use std::future::Future;
use std::pin::Pin;

use resuma::prelude::*;
use serde_json::json;

pub const DEMO_USER_COOKIE: &str = "resuma_demo_user";
pub const DEFAULT_USER: &str = "guest";

/// Attach session context to every `#[server]` action (see `/docs/security/middleware`).
pub fn install() {
    set_action_middleware(action_pipeline);
}

fn action_pipeline(
    req: FlowRequest,
) -> Pin<Box<dyn Future<Output = std::result::Result<FlowRequest, ResumaError>> + Send>> {
    Box::pin(async move { attach_session(req) })
}

pub fn attach_session(mut req: FlowRequest) -> Result<FlowRequest> {
    let user = session_user(&req);
    if !is_allowed_user(&user) {
        return Err(ResumaError::Unauthorized);
    }
    let mut roles = vec!["user".to_string()];
    if admin_users().iter().any(|a| a == &user) {
        roles.push("admin".into());
    }
    req.set_extension("authenticated", json!(true));
    req.set_extension("user_id", json!(user));
    req.set_extension("roles", json!(roles));
    Ok(req)
}

pub fn session_user(req: &FlowRequest) -> String {
    cookie_value(req.header("cookie"), DEMO_USER_COOKIE)
        .filter(|u| is_allowed_user(u))
        .unwrap_or_else(|| DEFAULT_USER.into())
}

fn cookie_value(cookie: Option<&str>, key: &str) -> Option<String> {
    cookie.and_then(|raw| {
        raw.split(';').find_map(|part| {
            let (k, v) = part.split_once('=')?;
            if k.trim() == key {
                Some(v.trim().to_string())
            } else {
                None
            }
        })
    })
}

pub fn demo_users() -> &'static [&'static str] {
    &["guest", "alice", "bob"]
}

pub fn is_allowed_user(user: &str) -> bool {
    !user.is_empty()
        && user.len() <= 32
        && user
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
        && demo_users().contains(&user)
}

pub fn admin_users() -> Vec<String> {
    std::env::var("RESUMA_TODO_ADMINS")
        .ok()
        .map(|s| {
            s.split(',')
                .map(str::trim)
                .filter(|x| !x.is_empty())
                .map(String::from)
                .collect()
        })
        .unwrap_or_else(|| vec!["alice".into()])
}

pub fn normalize_title(title: &str) -> Result<String> {
    let title = title.trim();
    if title.is_empty() {
        return Err(ResumaError::validation("title required"));
    }
    if title.len() > 120 {
        return Err(ResumaError::validation("title max 120 chars"));
    }
    Ok(title.to_string())
}

pub fn assert_owner(owner_id: &str, req: &FlowRequest) -> Result<()> {
    let uid = req.user_id().ok_or(ResumaError::Unauthorized)?;
    if owner_id != uid && !req.has_role("admin") {
        return Err(ResumaError::Forbidden("not your task".into()));
    }
    Ok(())
}

pub fn list_visible<'a, T>(
    items: &'a [T],
    req: &FlowRequest,
    owner: impl Fn(&T) -> &str,
) -> Vec<&'a T> {
    let uid = session_user(req);
    let is_admin = admin_users().iter().any(|a| a == &uid);
    items
        .iter()
        .filter(|t| is_admin || owner(t) == uid.as_str())
        .collect()
}
