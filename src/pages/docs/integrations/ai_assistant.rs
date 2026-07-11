use crate::site::code_block;
use resuma::prelude::*;

pub fn page(_req: FlowRequest) -> View {
    view! {
        <>
            <h1>"AI assistant (Cursor, Codex, Gemini)"</h1>
            <p class="lead">
                "Teach your editor how to write Resuma — reactive " <code>"view!"</code> ", Flow routes, server actions, and common pitfalls — with one CLI command."
            </p>

            {crate::site::demos::integrations_ai()}

            <h2>"Skill vs MCP — which one?"</h2>
            <table class="bench">
                <thead>
                    <tr>
                        <th></th>
                        <th>"Agent skill (recommended)"</th>
                        <th>"MCP server"</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>"Best for"</td>
                        <td>"Writing & fixing Resuma code in any project"</td>
                        <td>"Tools that call APIs, DBs, or live runtime state"</td>
                    </tr>
                    <tr>
                        <td>"Setup"</td>
                        <td><code>"resuma install skill"</code></td>
                        <td>"Custom server + editor MCP config (not bundled yet)"</td>
                    </tr>
                    <tr>
                        <td>"Editors"</td>
                        <td>"Cursor, Codex/agents, any SKILL.md-compatible agent"</td>
                        <td>"Cursor, Claude Desktop, etc."</td>
                    </tr>
                </tbody>
            </table>
            <p>
                "Start with the " <strong>"skill"</strong> ". It encodes Resuma patterns (signals, Show, Flow, #[server]) so the model ships working apps faster. "
                "An official Resuma MCP may come later for docs search and " <code>"resuma routes --generate"</code> " as tools."
            </p>

            <h2>"Install the skill (one command)"</h2>
            {code_block(r#"# Global — available in all projects (Cursor)
resuma install skill

# Only this repo / monorepo
resuma install skill --project

# Codex-style agents path (~/.agents/skills/)
resuma install skill --target agents

# Cursor + agents
resuma install skill --target all

# See paths without writing
resuma install skill --list

# Overwrite existing SKILL.md
resuma install skill --force"#)}

            <h2>"Where files land"</h2>
            <ul>
                <li><code>"~/.cursor/skills/resuma/SKILL.md"</code> " — Cursor personal skill"</li>
                <li><code>".cursor/skills/resuma/SKILL.md"</code> " — committed with your app (team shares the same guidance)"</li>
                <li><code>"~/.agents/skills/resuma/SKILL.md"</code> " — compatible with open agent skills ecosystems"</li>
            </ul>

            <h2>"Gemini / other editors"</h2>
            <p>
                "Copy " <code>"SKILL.md"</code> " from " <code>"resuma install skill --list"</code> " into your editor's rules or instructions file, "
                "or run " <code>"resuma install skill --project"</code> " and point the editor at " <code>".cursor/skills/resuma/SKILL.md"</code> "."
            </p>

            <h2>"What the skill covers"</h2>
            <ul>
                <li><code>"{signal}"</code> " vs " <code>"{signal.get()}"</code> " in " <code>"view!"</code> " (client reactivity)"</li>
                <li>"Reactive " <code>"<Show when={…}>"</code></li>
                <li><code>"FlowApp"</code> ", file-based pages, " <code>"resuma routes --generate"</code></li>
                <li><code>"#[server]"</code> ", " <code>"#[submit]"</code> ", " <code>"#[load]"</code></li>
                <li><code>"SeoKit"</code> " and auto " <code>"/robots.txt"</code> " / " <code>"/llms.txt"</code></li>
                <li>"Debugging checklist (core preload, handler chunks, CSRF)"</li>
            </ul>

            <h2>"Verify"</h2>
            {code_block(r#"resuma doctor
# In Cursor: ask "create a Resuma counter with view! and signal"
# The agent should use {count} not {count.get()} in the template."#)}

            <p>
                <a href="/docs/cli">"CLI reference"</a>
                " · "
                <a href="/docs/getting_started">"Getting started"</a>
                " · "
                <a href="/docs/integrations/seo_geo">"SEO & GEO"</a>
            </p>
        </>
    }
}
