use resuma::prelude::*;

pub fn page(_req: FlowRequest) -> View {
    view! {
        <>
            <h1>"FAQ"</h1>
            <p class="lead">"Common questions about resumability, bundle size, and how Resuma compares to hydration-based frameworks."</p>

            <h2>"What is resumability vs hydration?"</h2>
            <p>"Hydration re-executes your entire component tree on the client to attach event listeners. Resumability serializes signals and handler references into HTML during SSR; the client resumes only what the user interacts with — no full-tree replay."</p>

            <h2>"Does Resuma run Rust in the browser?"</h2>
            <p>"No. Components always execute on the server. Client-side code is a fixed runtime (loader + lazy core on first interaction) plus small handler chunks translated from closures at compile time (rs2js in resuma-macros). Business logic stays in Rust."</p>

            <h2>"How big is the client bundle?"</h2>
            <p>
                "Static pages ship zero JS. Interactive pages parse HTML immediately with "
                <strong>"907 B"</strong>
                " gzip loader.js, then fetch core.js (~4 KiB gzip) on the first click — about "
                <strong>"5 KiB"</strong>
                " total for first interaction. Handler and island chunks load on demand. See the "
                <a href="/docs/benchmark">"benchmark page"</a>
                " and "
                <a href="/docs/architecture">"architecture"</a>
                " for measured numbers."
            </p>

            <h2>"Do I need Node.js?"</h2>
            <p>"Only if you rebuild the JS runtime from source. Prebuilt assets ship inside the " <code>"resuma"</code> " crate (" <code>"assets/"</code> "). For app development, Rust + cargo (or " <code>"cargo install resuma"</code> ") is enough."</p>

            <h2>"Can I use Resuma without Flow?"</h2>
            <p>"Yes. ResumaApp supports single-page apps with manual route registration — ideal for counters, widgets, and embedded UI. Flow adds multi-page routing, loaders, submits, and middleware when you need a full site."</p>

            <h2>"Does Flow include a PWA?"</h2>
            <p>
                "Yes — manifest and service worker are enabled by default on " <code>"FlowApp"</code> ". "
                "Use " <code>".without_pwa()"</code> " or " <code>"RESUMA_PWA=0"</code> " to disable. "
                <a href="/docs/flow/pwa">"Details →"</a>
            </p>

            <h2>"How do forms work without JavaScript?"</h2>
            <p>"The " <code>"Form"</code> " component renders a real HTML form with " <code>"POST /_resuma/submit/:name"</code> ". Progressive enhancement: the runtime intercepts submit when loaded, but forms work as plain POST without JS."</p>

            <h2>"Is Resuma production-ready?"</h2>
            <p>
                "Yes — " <strong>"1.0"</strong> " follows semver for public APIs. Security defaults (CSRF, CSP, rate limits) ship enabled. "
                "Resuma OS adds self-hosted workers and ops. "
                "See " <a href="/docs/security">"Security"</a> ", "
                <a href="/docs/exec/ops">"Ops & production"</a> ", and "
                <a href="/docs/security/todo">"todo reference"</a> ", and "
                <a href="https://github.com/GoldevLab/resuma/blob/main/docs/STABILITY.md" target="_blank" rel="noopener">"STABILITY.md"</a> " in the framework repo."
            </p>

            <h2>"Where is the backend security reference?"</h2>
            <p><code>"examples/todo"</code> " — guards, DTO validation, service layer, authorization. Docs: " <a href="/docs/security/todo">"/docs/security/todo"</a>"."</p>

            <h2>"Do I need Redis, Postgres, or Cloudflare Workers?"</h2>
            <p>
                <strong>"No — for core Resuma features."</strong> " SSR, signals, server actions, CSRF, CSP, and "
                "rate limiting work out of the box with no external services. Rate limits use "
                <strong>"memory"</strong> " in dev and a "
                <strong>"disk backend"</strong> " in production (" <code>"{RESUMA_DATA_DIR}/rate-limit/"</code> ")."
            </p>
            <p>
                <strong>"Resuma OS"</strong> " (" <code>"resuma::exec"</code> ") adds disk-backed queues, cron scheduler, "
                "durable graphs, and webhooks — also without Redis. See "
                <a href="/docs/exec">"/docs/exec"</a> "."
            </p>
            <p>
                "Optional: add Postgres/SQLite via SQLx for your app's data — Resuma does not require a database "
                "for the framework itself. See "
                <a href="/docs/integrations/sqlx">"SQLx integration"</a> "."
            </p>
        </>
    }
}
