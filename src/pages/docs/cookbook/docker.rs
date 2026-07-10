use crate::site::code_block;
use resuma::prelude::*;

pub fn page(_req: FlowRequest) -> View {
    view! {
        <>
            <h1>"Docker Deploy"</h1>
            <p class="lead">"Build a minimal production image for a Resuma Flow app."</p>

            {crate::site::demos::cookbook_docker()}

            <h2>"Dockerfile"</h2>
            <p>"This repo (" <code>"resuma-docs"</code> ") is standalone: " <code>"Dockerfile"</code> ", "
                <code>"fly.toml"</code> ", and " <code>".dockerignore"</code> " live at the repo root. "
                "Deploy from this directory — not a symlink/junction on Windows (remote builders cannot read through them)."</p>
            <p>"The image builds the TypeScript client, compiles the " <code>"website"</code> " binary, and runs as non-root on Debian slim."</p>
            {code_block(r##"FROM node:22-bookworm AS client
WORKDIR /app
COPY package.json package-lock.json tsconfig.json ./
COPY client ./client
RUN npm ci 2>/dev/null || npm install
RUN npm run build:client

FROM rust:1-bookworm AS builder
WORKDIR /app
COPY Cargo.toml rust-toolchain.toml ./
COPY src ./src
COPY --from=client /app/static/client ./static/client
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && useradd --system --uid 10001 --create-home resuma
WORKDIR /app
COPY --from=builder /app/target/release/website /app/website
COPY --from=builder /app/src/pages /app/pages
RUN chown -R resuma:resuma /app
USER resuma
ENV HOST=0.0.0.0 PORT=3000 RESUMA_PAGES_ROOT=/app/pages
ENV RESUMA_ENV=production RESUMA_TRUST_PROXY=1
EXPOSE 3000
CMD ["/app/website"]"##)}

            <h2>"Build and run locally"</h2>
            {code_block(r#"docker build -t resuma-docs .
    docker run -p 3000:3000 -e HOST=0.0.0.0 -e PORT=3000 resuma-docs"#)}

            <h2>"Fly.io"</h2>
            <p>"A " <code>"fly.toml"</code> " is included at the repo root (same pattern as other apps in your Fly org)."</p>
            {code_block(r##"# First time (creates app resuma-docs in iad)
fly launch --no-deploy

# Deploy
fly deploy

# Open in browser
fly open"##)}

            <h2>"Bind address"</h2>
            <p>"Flow reads " <code>"RESUMA_ADDR"</code> " or " <code>"HOST"</code> " + " <code>"PORT"</code> ". Fly sets " <code>"HOST=0.0.0.0"</code> " and " <code>"PORT=3000"</code> " in fly.toml."</p>

            <h2>"Notes"</h2>
            <ul>
                <li>"Node.js is only in the build stage (Resuma Client bundles); the runtime image is Rust + pages only."</li>
                <li>"Resuma runtime JS is embedded in the " <code>"resuma"</code> " crate — no extra Node step for apps without client components."</li>
                <li>"Set " <code>"RESUMA_ENV=production"</code> " and " <code>"RESUMA_TRUST_PROXY=1"</code> " — see " <a href="/docs/security/environment">"Environment variables"</a> " (workers need " <code>"RESUMA_EXEC_API_KEY"</code> " via " <code>"fly secrets"</code> ")."</li>
                <li>
                    "Mount a persistent volume at " <code>"RESUMA_DATA_DIR"</code> " (default "
                    <code>".resuma"</code> ") for disk-backed rate limits, exec queues, and scheduler — "
                    <strong>"no Redis required"</strong>"."
                </li>
                <li>"Health check hits " <code>"/"</code> " (see " <code>"fly.toml"</code> "); Flow also serves " <code>"/robots.txt"</code> " and " <code>"/sitemap.xml"</code>"."</li>
            </ul>
        </>
    }
}
