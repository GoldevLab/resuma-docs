//! Homepage showcase worker — registered via `#[worker]` for Resuma OS demo.

use std::time::Duration;

use resuma::exec::cancel::run_cancellable;
use resuma::exec::WorkerContext;
use resuma::prelude::*;
use resuma::worker;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DocsShowcaseInput {
    pub topic: String,
    #[serde(default)]
    pub blurb: String,
}

#[derive(Debug, serde::Serialize)]
pub struct DocsShowcaseOutput {
    pub topic: String,
    pub words: usize,
    pub chars: usize,
    pub highlights: Vec<String>,
    pub message: String,
}

#[worker(intent = "showcase Resuma OS workers for the documentation homepage")]
pub async fn docs_showcase(
    input: DocsShowcaseInput,
    ctx: WorkerContext,
) -> Result<DocsShowcaseOutput> {
    let topic = input.topic.trim();
    if topic.is_empty() {
        return Err(ResumaError::Validation("topic is required".into()));
    }

    let blurb = if input.blurb.trim().is_empty() {
        format!("Exploring {topic} with Resuma OS — durable graphs, SSE events, pause and cancel.")
    } else {
        input.blurb.trim().to_string()
    };

    ctx.log(format!("docs_showcase: topic=\"{topic}\""));
    ctx.log("Panel mounting — you have ~25s to try Pause or Cancel while steps run.");
    ctx.progress(2);

    run_cancellable(&ctx.cancel_token(), async {
        // Let the browser mount the execution panel before the long steps.
        tokio::time::sleep(Duration::from_secs(5)).await;
        Ok::<(), ResumaError>(())
    })
    .await?;
    ctx.check_cancelled()?;
    ctx.progress(5);

    let steps = [
        "Planning execution graph…",
        "Registering worker nodes…",
        "Streaming progress events…",
        "Checkpointing durable state…",
        "Formatting showcase result…",
    ];

    for (i, msg) in steps.iter().enumerate() {
        ctx.check_cancelled()?;
        run_cancellable(&ctx.cancel_token(), async {
            // Long enough for Pause / Cancel in the live demo UI.
            tokio::time::sleep(Duration::from_millis(4_000)).await;
            Ok::<(), ResumaError>(())
        })
        .await?;
        ctx.check_cancelled()?;
        ctx.log((*msg).to_string());
        ctx.progress(((i + 1) * 18) as u8);
    }

    let words: Vec<_> = blurb.split_whitespace().collect();
    let highlights = vec![
        "durable execution".into(),
        "SSE event stream".into(),
        "pause / resume / cancel".into(),
        "self-hosted queue".into(),
    ];

    Ok(DocsShowcaseOutput {
        topic: topic.to_string(),
        words: words.len(),
        chars: blurb.len(),
        highlights,
        message: format!(
            "Processed \"{topic}\" — {} words, {} chars. Try Pause or Cancel in the panel.",
            words.len(),
            blurb.len()
        ),
    })
}
