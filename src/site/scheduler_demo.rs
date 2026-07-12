//! Live cron scheduler demo — list, create, tick real disk-backed jobs.

use resuma::exec::scheduler::{self, CreateScheduleBody, ScheduleListResponse};
use resuma::prelude::*;
use serde_json::json;

#[server]
async fn docs_scheduler_list() -> Result<ScheduleListResponse> {
    scheduler::list_response()
}

#[server]
async fn docs_scheduler_create() -> Result<String> {
    let suffix = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let job = scheduler::create(CreateScheduleBody {
        name: format!("docs-demo-{suffix}"),
        cron: "* * * * *".into(),
        worker: "docs_showcase".into(),
        input: json!({
            "topic": "Scheduler demo",
            "blurb": "Created from the docs scheduler page — fires when due on tick."
        }),
        queue: "docs".into(),
        enabled: true,
    })?;
    Ok(format!(
        "Created job \"{}\" (id {}) · next run at unix ms {}",
        job.name, job.id, job.next_run_ms
    ))
}

#[server]
async fn docs_scheduler_tick() -> Result<String> {
    let fired = scheduler::tick().await?;
    Ok(format!("Scheduler tick fired {fired} due job(s)"))
}

#[server]
async fn docs_scheduler_remove(id: String) -> Result<String> {
    if scheduler::remove(&id)? {
        Ok(format!("Removed schedule {id}"))
    } else {
        Err(ResumaError::validation("schedule not found"))
    }
}

/// Interactive scheduler panel for `/docs/exec/scheduler`.
#[component]
pub fn SchedulerDemoWidget() -> View {
    let output = signal(String::new());
    view! {
        <div class="scheduler-demo">
            <p class="demo-muted">
                "Uses the real "
                <code>"resuma::exec::scheduler"</code>
                " API on this server — jobs persist under "
                <code>".resuma/scheduler/"</code>
                "."
            </p>
            <div class="demo-row">
                <button type="button" class="btn btn-sm" onClick={js!(async () => {
                    state.output.set("Loading schedules…");
                    const res = await __resuma.safeAction("docs_scheduler_list", []);
                    if (!res.ok) {
                        state.output.set(res.error);
                        return;
                    }
                    const jobs = res.value.jobs ?? [];
                    if (!jobs.length) {
                        state.output.set("No schedules yet — create one below.");
                        return;
                    }
                    state.output.set(jobs.map(j =>
                        j.name + " (" + j.id + ") - cron " + j.cron + " - worker " + j.worker + " - queue " + j.queue
                    ).join("\n"));
                })}>"List jobs"</button>
                <button type="button" class="btn btn-sm btn-primary" onClick={js!(async () => {
                    state.output.set("Creating schedule…");
                    const res = await __resuma.safeAction("docs_scheduler_create", []);
                    state.output.set(res.ok ? res.value : res.error);
                })}>"Create demo job"</button>
                <button type="button" class="btn btn-sm btn-ghost" onClick={js!(async () => {
                    state.output.set("Running tick…");
                    const res = await __resuma.safeAction("docs_scheduler_tick", []);
                    state.output.set(res.ok ? res.value : res.error);
                })}>"Run tick"</button>
            </div>
            <pre class="demo-output scheduler-demo-out" aria-live="polite">{output}</pre>
        </div>
    }
}
