//! Liquid glass — light, white-forward design system for docs and landing.

pub const SITE_CSS: &str = r#"<style>
@import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700;800&display=swap');

:root {
  --bg: #eceff4;
  --bg-subtle: rgba(255, 255, 255, 0.42);
  --bg-card: rgba(255, 255, 255, 0.52);
  --border: rgba(15, 23, 42, 0.06);
  --border-glass: rgba(255, 255, 255, 0.72);
  --text: #0f172a;
  --muted: #64748b;
  --primary: #1e293b;
  --primary-hover: #0f172a;
  --primary-soft: rgba(255, 255, 255, 0.65);
  --accent: #2563eb;
  --accent-soft: rgba(37, 99, 235, 0.08);
  --success: #059669;
  --danger: #dc2626;
  --mono: ui-monospace, "Cascadia Code", "Fira Code", monospace;
  --sans: "Inter", "Segoe UI", ui-sans-serif, system-ui, sans-serif;
  --sidebar-w: 17.5rem;
  --glass-blur: 28px;
  --glass-saturate: 185%;
  --glass-bright: 1.08;
  --grid-line: rgba(15, 23, 42, 0.07);
  --glass-shadow:
    0 10px 40px rgba(15, 23, 42, 0.06),
    0 2px 10px rgba(15, 23, 42, 0.03),
    inset 0 1px 1px rgba(255, 255, 255, 0.95),
    inset 0 -12px 28px rgba(255, 255, 255, 0.18);
  --glass-shadow-lg:
    0 28px 80px rgba(15, 23, 42, 0.08),
    0 12px 32px rgba(15, 23, 42, 0.04),
    inset 0 1px 1px rgba(255, 255, 255, 1),
    inset 0 -18px 40px rgba(255, 255, 255, 0.22);
  --glass-highlight: linear-gradient(
    145deg,
    rgba(255, 255, 255, 0.92) 0%,
    rgba(255, 255, 255, 0.2) 42%,
    rgba(255, 255, 255, 0.55) 100%
  );
  --iridescent: linear-gradient(
    135deg,
    rgba(255, 255, 255, 0.95) 0%,
    rgba(186, 230, 253, 0.35) 22%,
    rgba(251, 207, 232, 0.28) 48%,
    rgba(167, 243, 208, 0.22) 72%,
    rgba(255, 255, 255, 0.88) 100%
  );
  --radius-sm: 12px;
  --radius-md: 20px;
  --radius-lg: 28px;
  --radius-xl: 36px;
  --liquid-a: 58% 42% 32% 68% / 58% 38% 62% 42%;
  --liquid-b: 42% 58% 68% 32% / 48% 62% 38% 52%;
}

* { box-sizing: border-box; }
html { scroll-behavior: smooth; }

body {
  font-family: var(--sans);
  color: var(--text);
  margin: 0;
  line-height: 1.65;
  font-size: 16px;
  min-height: 100vh;
  background: var(--bg);
  position: relative;
  isolation: isolate;
}

body::before {
  content: "";
  position: fixed;
  inset: 0;
  z-index: -3;
  background:
    linear-gradient(rgba(255, 255, 255, 0.72), rgba(255, 255, 255, 0.72)),
    linear-gradient(90deg, var(--grid-line) 1px, transparent 1px),
    linear-gradient(0deg, var(--grid-line) 1px, transparent 1px),
    radial-gradient(ellipse 90% 70% at 8% -15%, rgba(255, 255, 255, 0.98), transparent 55%),
    radial-gradient(ellipse 70% 55% at 95% 5%, rgba(226, 232, 240, 0.45), transparent 50%),
    radial-gradient(ellipse 60% 45% at 50% 105%, rgba(241, 245, 249, 0.85), transparent 55%),
    linear-gradient(168deg, #f4f6fa 0%, #eceff4 45%, #e4e9f0 100%);
  background-size: auto, 48px 48px, 48px 48px, auto, auto, auto, auto;
  pointer-events: none;
}

body::after {
  content: "";
  position: fixed;
  width: min(42vw, 480px);
  height: min(42vw, 480px);
  top: -18%;
  right: -12%;
  z-index: -2;
  border-radius: 50%;
  background: radial-gradient(circle, rgba(255, 255, 255, 0.85) 0%, transparent 68%);
  filter: blur(48px);
  opacity: 0.55;
  pointer-events: none;
}

@keyframes liquid-float {
  from { transform: translate(0, 0) scale(1) rotate(0deg); }
  to { transform: translate(-4%, 5%) scale(1.05) rotate(3deg); }
}

@keyframes liquid-morph {
  0%, 100% { border-radius: 58% 42% 32% 68% / 58% 38% 62% 42%; }
  33% { border-radius: 42% 58% 68% 32% / 48% 62% 38% 52%; }
  66% { border-radius: 52% 48% 38% 62% / 42% 58% 48% 52%; }
}

@keyframes liquid-shimmer {
  0%, 100% { opacity: 0.55; transform: translateX(-2%) skewX(-4deg); }
  50% { opacity: 0.85; transform: translateX(2%) skewX(4deg); }
}

@media (prefers-reduced-motion: reduce) {
  body::after,
  .liquid-blob,
  .hero-panel { animation: none !important; }
}

/* Floating liquid glass orbs (decorative) */
.liquid-orbs {
  position: absolute;
  inset: 0;
  overflow: hidden;
  pointer-events: none;
  z-index: 0;
}
.liquid-orbs-docs { position: fixed; inset: 0; z-index: -1; }
.liquid-blob {
  position: absolute;
  border-radius: var(--liquid-a);
  background:
    radial-gradient(circle at 32% 24%, rgba(255, 255, 255, 0.95) 0%, transparent 38%),
    radial-gradient(circle at 78% 82%, rgba(251, 207, 232, 0.25) 0%, transparent 45%),
    radial-gradient(circle at 55% 55%, rgba(186, 230, 253, 0.2) 0%, transparent 55%),
    linear-gradient(155deg, rgba(255, 255, 255, 0.48), rgba(255, 255, 255, 0.08));
  border: 1px solid rgba(255, 255, 255, 0.65);
  box-shadow:
    0 24px 64px rgba(15, 23, 42, 0.06),
    inset 0 1px 1px rgba(255, 255, 255, 0.95),
    inset 0 -20px 36px rgba(255, 255, 255, 0.2);
  backdrop-filter: blur(18px) saturate(160%);
  -webkit-backdrop-filter: blur(18px) saturate(160%);
  animation: liquid-morph 26s ease-in-out infinite alternate;
}
.liquid-blob-a {
  width: min(38vw, 420px);
  height: min(32vw, 360px);
  top: 8%;
  right: 6%;
  opacity: 0.72;
  animation-duration: 24s;
}
.liquid-blob-b {
  width: min(22vw, 240px);
  height: min(28vw, 300px);
  bottom: 12%;
  left: 4%;
  opacity: 0.55;
  border-radius: var(--liquid-b);
  animation-duration: 30s;
  animation-direction: alternate-reverse;
}
.liquid-blob-c {
  width: min(28vw, 320px);
  height: min(24vw, 280px);
  top: 18%;
  right: 8%;
  opacity: 0.45;
}

/* Liquid glass surface — shared by panels */
.liquid-surface,
.card,
.pillar,
.pipeline-step,
.package-box,
.docs-sidebar,
.live-demo,
.site-header {
  position: relative;
}
.liquid-surface::before,
.liquid-surface::after,
.card::before,
.card::after,
.pillar::before,
.pillar::after,
.pipeline-step::before,
.pipeline-step::after,
.package-box::before,
.package-box::after,
.docs-sidebar::before,
.docs-sidebar::after,
.live-demo::before,
.live-demo::after {
  content: "";
  position: absolute;
  inset: 0;
  border-radius: inherit;
  pointer-events: none;
}
.liquid-surface::before,
.card::before,
.pillar::before,
.pipeline-step::before,
.package-box::before,
.docs-sidebar::before,
.live-demo::before {
  background: linear-gradient(
    165deg,
    rgba(255, 255, 255, 0.72) 0%,
    rgba(255, 255, 255, 0.08) 38%,
    transparent 62%
  );
  z-index: 0;
}
.liquid-surface::after,
.card::after,
.pillar::after,
.pipeline-step::after,
.package-box::after,
.docs-sidebar::after,
.live-demo::after {
  inset: -1px;
  padding: 1px;
  background: var(--iridescent);
  -webkit-mask:
    linear-gradient(#fff 0 0) content-box,
    linear-gradient(#fff 0 0);
  -webkit-mask-composite: xor;
  mask-composite: exclude;
  opacity: 0.55;
  z-index: 0;
}
.liquid-surface > *,
.card > *,
.pillar > *,
.pipeline-step > *,
.package-box > *,
.docs-sidebar > *,
.live-demo > * { position: relative; z-index: 1; }

a { color: var(--accent); text-decoration: none; transition: color 0.15s; }
a:hover { color: #1d4ed8; text-decoration: underline; }
code, pre { font-family: var(--mono); font-size: 0.9em; }
code {
  background: rgba(255, 255, 255, 0.55);
  padding: 0.12rem 0.4rem;
  border-radius: 7px;
  border: 1px solid var(--border);
  backdrop-filter: blur(8px);
}
pre.code {
  background: rgba(255, 255, 255, 0.5);
  backdrop-filter: blur(var(--glass-blur)) saturate(var(--glass-saturate));
  -webkit-backdrop-filter: blur(var(--glass-blur)) saturate(var(--glass-saturate));
  border: 1px solid var(--border-glass);
  border-radius: var(--radius-md);
  padding: 1rem 1.15rem;
  overflow-x: auto;
  margin: 1rem 0;
  line-height: 1.5;
  box-shadow: var(--glass-shadow);
}
pre.code code { background: none; border: 0; padding: 0; backdrop-filter: none; }

/* ── Chrome ─────────────────────────────────────────────────────────── */

.site-header {
  position: sticky;
  top: 0;
  z-index: 50;
  background: rgba(255, 255, 255, 0.38);
  backdrop-filter: blur(var(--glass-blur)) saturate(var(--glass-saturate)) brightness(var(--glass-bright));
  -webkit-backdrop-filter: blur(var(--glass-blur)) saturate(var(--glass-saturate)) brightness(var(--glass-bright));
  border-bottom: 1px solid rgba(255, 255, 255, 0.5);
  box-shadow: var(--glass-shadow);
}
.site-header::after { opacity: 0.35; border-radius: 0; }
.header-inner {
  max-width: 80rem;
  margin: 0 auto;
  padding: 0.7rem 1.25rem;
  display: flex;
  align-items: center;
  gap: 1.5rem;
  position: relative;
  z-index: 1;
}
.logo {
  font-weight: 700;
  font-size: 1.12rem;
  color: var(--text);
  display: flex;
  align-items: center;
  gap: 0.5rem;
  text-decoration: none !important;
  letter-spacing: -0.02em;
}
.logo-mark {
  width: 2rem;
  height: 2rem;
  border-radius: 11px;
  background: var(--glass-highlight);
  backdrop-filter: blur(12px);
  border: 1px solid var(--border-glass);
  box-shadow: var(--glass-shadow);
  display: grid;
  place-items: center;
  font-size: 1rem;
}
.site-nav { display: flex; gap: 0.35rem; flex: 1; flex-wrap: wrap; }
.site-nav a {
  color: var(--muted);
  font-weight: 500;
  text-decoration: none;
  padding: 0.35rem 0.65rem;
  border-radius: 999px;
  transition: background 0.15s, color 0.15s;
}
.site-nav a.active,
.site-nav a:hover {
  color: var(--text);
  background: rgba(255, 255, 255, 0.55);
  text-decoration: none;
}
.header-actions { display: flex; gap: 0.45rem; flex-wrap: wrap; }

/* Liquid glass pills — glowing edges, specular highlight, convex volume */
.btn {
  position: relative;
  isolation: isolate;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 0.45rem;
  padding: 0.58rem 1.2rem;
  border-radius: 999px;
  font-weight: 600;
  border: 1px solid rgba(255, 255, 255, 0.62);
  cursor: pointer;
  text-decoration: none !important;
  font-size: 0.88rem;
  line-height: 1.2;
  color: var(--text);
  background: rgba(255, 255, 255, 0.18);
  backdrop-filter: blur(22px) saturate(190%) brightness(1.06);
  -webkit-backdrop-filter: blur(22px) saturate(190%) brightness(1.06);
  box-shadow:
    0 6px 28px rgba(15, 23, 42, 0.07),
    0 1px 2px rgba(15, 23, 42, 0.04),
    inset 0 1px 1px rgba(255, 255, 255, 0.98),
    inset 0 -10px 22px rgba(255, 255, 255, 0.12),
    inset 2px 2px 6px rgba(255, 255, 255, 0.38);
  transition: transform 0.2s ease, box-shadow 0.2s ease, background 0.2s ease, border-color 0.2s ease;
}
.btn::before {
  content: "";
  position: absolute;
  inset: 0;
  border-radius: inherit;
  background: linear-gradient(
    148deg,
    rgba(255, 255, 255, 0.92) 0%,
    rgba(255, 255, 255, 0.28) 22%,
    rgba(255, 255, 255, 0.04) 48%,
    transparent 62%
  );
  pointer-events: none;
  z-index: 0;
}
.btn::after {
  content: "";
  position: absolute;
  inset: -1px;
  border-radius: inherit;
  padding: 1px;
  background: linear-gradient(
    145deg,
    rgba(255, 255, 255, 0.95) 0%,
    rgba(255, 255, 255, 0.22) 35%,
    rgba(255, 255, 255, 0.08) 65%,
    rgba(255, 255, 255, 0.42) 100%
  );
  -webkit-mask:
    linear-gradient(#fff 0 0) content-box,
    linear-gradient(#fff 0 0);
  -webkit-mask-composite: xor;
  mask-composite: exclude;
  pointer-events: none;
  z-index: 0;
  opacity: 0.85;
}
.btn:hover {
  transform: translateY(-2px);
  border-color: rgba(255, 255, 255, 0.88);
  background: rgba(255, 255, 255, 0.28);
  box-shadow:
    0 12px 36px rgba(15, 23, 42, 0.1),
    0 2px 6px rgba(15, 23, 42, 0.05),
    inset 0 1px 1px rgba(255, 255, 255, 1),
    inset 0 -10px 24px rgba(255, 255, 255, 0.16),
    inset 2px 2px 8px rgba(255, 255, 255, 0.45);
  text-decoration: none;
}
.btn:active {
  transform: translateY(0);
  box-shadow:
    0 4px 16px rgba(15, 23, 42, 0.08),
    inset 0 2px 8px rgba(15, 23, 42, 0.06),
    inset 0 1px 1px rgba(255, 255, 255, 0.75);
}

.btn-primary {
  color: #fff;
  background: rgba(15, 23, 42, 0.58);
  border-color: rgba(255, 255, 255, 0.42);
  backdrop-filter: blur(24px) saturate(180%) brightness(0.95);
  -webkit-backdrop-filter: blur(24px) saturate(180%) brightness(0.95);
  box-shadow:
    0 10px 36px rgba(15, 23, 42, 0.2),
    0 2px 8px rgba(15, 23, 42, 0.1),
    inset 0 1px 1px rgba(255, 255, 255, 0.55),
    inset 0 -12px 24px rgba(0, 0, 0, 0.12),
    inset 2px 2px 8px rgba(255, 255, 255, 0.22);
}
.btn-primary::before {
  background: linear-gradient(
    148deg,
    rgba(255, 255, 255, 0.72) 0%,
    rgba(255, 255, 255, 0.14) 24%,
    rgba(255, 255, 255, 0.02) 50%,
    transparent 64%
  );
}
.btn-primary::after {
  background: linear-gradient(
    145deg,
    rgba(255, 255, 255, 0.75) 0%,
    rgba(255, 255, 255, 0.18) 40%,
    rgba(255, 255, 255, 0.06) 70%,
    rgba(255, 255, 255, 0.35) 100%
  );
  opacity: 0.9;
}
.btn-primary:hover {
  color: #fff;
  background: rgba(15, 23, 42, 0.68);
  border-color: rgba(255, 255, 255, 0.58);
  box-shadow:
    0 16px 44px rgba(15, 23, 42, 0.26),
    0 4px 12px rgba(15, 23, 42, 0.12),
    inset 0 1px 1px rgba(255, 255, 255, 0.65),
    inset 0 -12px 26px rgba(0, 0, 0, 0.14),
    inset 2px 2px 10px rgba(255, 255, 255, 0.28);
  text-decoration: none;
}

.btn-ghost {
  color: var(--text);
  background: rgba(255, 255, 255, 0.14);
  border-color: rgba(255, 255, 255, 0.68);
}
.btn-ghost:hover {
  color: var(--text);
  background: rgba(255, 255, 255, 0.32);
  text-decoration: none;
}

/* ── Landing hero ───────────────────────────────────────────────────── */

.landing { overflow-x: clip; }

.hero-wrap {
  position: relative;
  overflow: visible;
  background: transparent;
  border-bottom: 1px solid rgba(255, 255, 255, 0.55);
}
.hero-wrap::before {
  content: "";
  position: absolute;
  inset: 0;
  z-index: 0;
  pointer-events: none;
  background:
    radial-gradient(ellipse 55% 45% at 88% 18%, rgba(186, 230, 253, 0.16), transparent 70%),
    radial-gradient(ellipse 40% 35% at 12% 85%, rgba(241, 245, 249, 0.9), transparent 70%);
}
.hero-particles,
[data-r-client="hero-particles"] {
  position: absolute;
  inset: 0;
  z-index: 0;
  pointer-events: none;
  overflow: hidden;
  opacity: 0.35;
  mix-blend-mode: normal;
}
.hero-particles-canvas {
  width: 100%;
  height: 100%;
  display: block;
}
.hero-wrap .hero,
.hero-wrap .metrics-bar { position: relative; z-index: 1; }
@media (prefers-reduced-motion: reduce) {
  .hero-particles,
  [data-r-client="hero-particles"] { display: none; }
}

.hero {
  max-width: 80rem;
  margin: 0 auto;
  padding: 4.5rem 1.25rem 3.5rem;
  display: grid;
  gap: 2.5rem;
}
@media (min-width: 900px) {
  .hero {
    grid-template-columns: minmax(0, 1.05fr) minmax(0, 0.95fr);
    align-items: center;
    gap: 3rem;
    padding: 5.5rem 1.25rem 4.5rem;
  }
}
.hero > div:first-child { min-width: 0; }
.hero > .hero-panel { min-width: 0; width: 100%; max-width: 28rem; }
@media (min-width: 900px) {
  .hero > .hero-panel { margin-left: auto; max-width: none; }
}

.hero-badge {
  display: inline-flex;
  align-items: center;
  gap: 0.45rem;
  padding: 0.38rem 0.9rem;
  border-radius: 999px;
  background: rgba(255, 255, 255, 0.16);
  backdrop-filter: blur(18px) saturate(180%);
  -webkit-backdrop-filter: blur(18px) saturate(180%);
  border: 1px solid rgba(255, 255, 255, 0.65);
  box-shadow:
    inset 0 1px 1px rgba(255, 255, 255, 0.95),
    inset 0 -8px 18px rgba(255, 255, 255, 0.1),
    0 4px 18px rgba(15, 23, 42, 0.05);
  color: var(--muted);
  font-size: 0.76rem;
  font-weight: 600;
  margin-bottom: 1rem;
  letter-spacing: 0.02em;
}
.hero-badge-dot {
  width: 0.45rem;
  height: 0.45rem;
  border-radius: 50%;
  background: var(--success);
  box-shadow: 0 0 0 3px rgba(5, 150, 105, 0.18);
}
.hero h1 {
  font-size: clamp(2.4rem, 5vw, 3.55rem);
  line-height: 1.06;
  margin: 0 0 1rem;
  letter-spacing: -0.04em;
  color: var(--text);
  font-weight: 800;
}
.hero h1 .accent {
  background: linear-gradient(135deg, #1e293b 0%, #64748b 100%);
  -webkit-background-clip: text;
  background-clip: text;
  color: transparent;
}
.hero-tagline {
  font-size: clamp(1.12rem, 2.5vw, 1.4rem);
  font-weight: 600;
  color: var(--muted);
  margin: -0.35rem 0 1.1rem;
  line-height: 1.4;
  letter-spacing: -0.02em;
  max-width: 34rem;
}
.hero-lead {
  color: var(--muted);
  font-size: 1.12rem;
  max-width: 34rem;
  margin: 0 0 1.5rem;
  line-height: 1.65;
}
.hero-actions { display: flex; flex-wrap: wrap; gap: 0.65rem; margin-bottom: 0; }
.hero-actions .btn-primary { padding: 0.7rem 1.35rem; font-size: 0.95rem; }
.hero-note { margin: 1.25rem 0 0; font-size: 0.82rem; color: var(--muted); }
.hero-note code { font-size: 0.78rem; }

.hero-panel {
  position: relative;
  background: rgba(255, 255, 255, 0.78);
  backdrop-filter: blur(20px) saturate(160%);
  -webkit-backdrop-filter: blur(20px) saturate(160%);
  border: 1px solid rgba(255, 255, 255, 0.92);
  border-radius: 22px;
  padding: 0;
  overflow: hidden;
  box-shadow:
    0 20px 50px rgba(15, 23, 42, 0.07),
    0 4px 14px rgba(15, 23, 42, 0.04),
    inset 0 1px 0 rgba(255, 255, 255, 1);
}
.hero-panel::before {
  content: "";
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  height: 42%;
  border-radius: 22px 22px 0 0;
  background: linear-gradient(180deg, rgba(255, 255, 255, 0.55) 0%, transparent 100%);
  pointer-events: none;
  z-index: 0;
}
.hero-panel > * { position: relative; z-index: 1; }
.hero-panel-top {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0.75rem 1rem;
  background: rgba(248, 250, 252, 0.72);
  border-bottom: 1px solid rgba(15, 23, 42, 0.06);
}
.hero-panel-dots { display: flex; gap: 0.35rem; }
.hero-panel-dots span { width: 0.55rem; height: 0.55rem; border-radius: 50%; opacity: 0.75; }
.hero-panel-dots span:first-child { background: #fca5a5; }
.hero-panel-dots span:nth-child(2) { background: #fcd34d; }
.hero-panel-dots span:nth-child(3) { background: #86efac; }
.hero-panel-label { font-size: 0.72rem; color: var(--muted); font-family: var(--mono); }
.hero-panel-body { padding: 1.1rem 1.15rem 1.2rem; }
.hero-panel-body pre.code { margin: 0; border: 0; background: transparent; padding: 0; font-size: 0.82rem; box-shadow: none; backdrop-filter: none; }
.hero-panel-caption {
  margin-top: 0.85rem;
  padding-top: 0.85rem;
  border-top: 1px dashed rgba(15, 23, 42, 0.08);
  font-size: 0.8rem;
  color: var(--muted);
}
.hero-panel-caption strong { color: var(--text); }

.hero-payload-preview { display: flex; flex-direction: column; gap: 0.55rem; }
.hero-payload-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 0.85rem;
  padding: 0.62rem 0.75rem;
  border-radius: 12px;
  border: 1px solid rgba(15, 23, 42, 0.06);
  background: rgba(255, 255, 255, 0.55);
  font-size: 0.82rem;
}
.hero-payload-row span {
  color: var(--muted);
  line-height: 1.35;
  min-width: 0;
}
.hero-payload-row strong {
  color: var(--text);
  font-family: var(--mono);
  font-size: 0.8rem;
  flex-shrink: 0;
  white-space: nowrap;
}
.hero-payload-row-accent {
  background: rgba(255, 255, 255, 0.92);
  border-color: rgba(15, 23, 42, 0.08);
  box-shadow: inset 0 1px 0 rgba(255, 255, 255, 1);
}
.hero-payload-row-accent strong { color: var(--text); }
.hero-payload-row-zero strong { color: var(--success); }
.hero-payload-row-muted { opacity: 0.72; }

/* ── Sections ───────────────────────────────────────────────────────── */

.zero-strip {
  padding-top: 2.5rem;
  padding-bottom: 2.5rem;
  background: rgba(255, 255, 255, 0.35);
  backdrop-filter: blur(16px);
  border-bottom: 1px solid rgba(255, 255, 255, 0.65);
}
.zero-strip-inner { text-align: center; max-width: 40rem; margin: 0 auto; }
.zero-strip-eyebrow {
  display: inline-block;
  font-size: 0.72rem;
  font-weight: 700;
  letter-spacing: 0.08em;
  text-transform: uppercase;
  color: var(--success);
  margin-bottom: 0.5rem;
}
.zero-strip-title {
  font-size: clamp(1.75rem, 3.5vw, 2.35rem);
  margin: 0 0 0.65rem;
  letter-spacing: -0.03em;
  font-weight: 800;
  line-height: 1.15;
}
.zero-strip-body { color: var(--muted); margin: 0 auto 1.25rem; font-size: 1.05rem; line-height: 1.6; }

.payload-stack {
  display: grid;
  gap: 0.85rem;
  grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));
}
.payload-layer {
  background: rgba(255, 255, 255, 0.55);
  backdrop-filter: blur(var(--glass-blur)) saturate(var(--glass-saturate));
  border: 1px solid var(--border-glass);
  border-radius: var(--radius-md);
  padding: 1.15rem 1.05rem;
  box-shadow: var(--glass-shadow);
  transition: transform 0.2s, box-shadow 0.2s;
}
.payload-layer:hover { transform: translateY(-2px); box-shadow: var(--glass-shadow-lg); }
.payload-layer-accent {
  background: rgba(255, 255, 255, 0.72);
  border-color: rgba(255, 255, 255, 0.98);
}
.payload-layer-top {
  display: flex;
  align-items: baseline;
  justify-content: space-between;
  gap: 0.5rem;
  margin-bottom: 0.45rem;
}
.payload-layer-top strong { font-size: 0.95rem; color: var(--text); }
.payload-layer-size { font-family: var(--mono); font-size: 0.78rem; color: var(--muted); font-weight: 600; }
.payload-layer p { margin: 0; color: var(--muted); font-size: 0.86rem; line-height: 1.5; }

.speed-chart { display: flex; flex-direction: column; gap: 0.65rem; margin-bottom: 1.5rem; }
.speed-bar-head {
  display: flex;
  justify-content: space-between;
  align-items: baseline;
  margin-bottom: 0.3rem;
  font-size: 0.88rem;
}
.speed-bar-name { font-weight: 600; color: var(--text); }
.speed-bar-size { font-family: var(--mono); font-size: 0.78rem; color: var(--muted); }
.speed-bar-track {
  height: 0.55rem;
  border-radius: 999px;
  background: rgba(255, 255, 255, 0.45);
  border: 1px solid rgba(255, 255, 255, 0.75);
  overflow: hidden;
  box-shadow: inset 0 1px 2px rgba(15, 23, 42, 0.04);
}
.speed-bar-fill {
  height: 100%;
  border-radius: 999px;
  background: linear-gradient(90deg, #cbd5e1 0%, #94a3b8 100%);
  min-width: 4px;
}
.speed-bar-highlight .speed-bar-name { color: var(--text); }
.speed-bar-highlight .speed-bar-size { color: var(--text); font-weight: 600; }
.speed-bar-highlight .speed-bar-fill {
  background: linear-gradient(90deg, #1e293b 0%, #64748b 100%);
}

.docs-try-grid {
  display: grid;
  gap: 0.85rem;
  grid-template-columns: repeat(auto-fit, minmax(240px, 1fr));
}

.metrics-bar {
  max-width: 80rem;
  margin: 0 auto;
  padding: 0 1.25rem 2.5rem;
  display: grid;
  gap: 0.75rem;
  grid-template-columns: repeat(auto-fit, minmax(9.5rem, 1fr));
}
.metric-item {
  background: rgba(255, 255, 255, 0.62);
  backdrop-filter: blur(16px) saturate(160%);
  -webkit-backdrop-filter: blur(16px) saturate(160%);
  border: 1px solid rgba(255, 255, 255, 0.88);
  border-radius: 16px;
  padding: 1rem 1.1rem;
  text-align: center;
  box-shadow:
    0 8px 24px rgba(15, 23, 42, 0.04),
    inset 0 1px 0 rgba(255, 255, 255, 1);
}
.metric-item strong {
  display: block;
  font-size: 1.65rem;
  letter-spacing: -0.03em;
  color: var(--text);
  line-height: 1.1;
  margin-bottom: 0.25rem;
}
.metric-item span { font-size: 0.78rem; color: var(--muted); font-weight: 500; }

.section { max-width: 80rem; margin: 0 auto; padding: 3.25rem 1.25rem; }
.section-alt {
  background: rgba(255, 255, 255, 0.28);
  backdrop-filter: blur(12px);
  border-top: 1px solid rgba(255, 255, 255, 0.65);
  border-bottom: 1px solid rgba(255, 255, 255, 0.65);
}
.section-title {
  font-size: clamp(1.65rem, 3vw, 2.15rem);
  margin: 0 0 0.55rem;
  letter-spacing: -0.03em;
  font-weight: 800;
  line-height: 1.15;
}
.section-sub {
  color: var(--muted);
  margin: 0 0 2rem;
  max-width: 42rem;
  font-size: 1.05rem;
  line-height: 1.55;
}
.section-eyebrow {
  display: inline-block;
  font-size: 0.72rem;
  font-weight: 700;
  letter-spacing: 0.08em;
  text-transform: uppercase;
  color: var(--muted);
  margin-bottom: 0.5rem;
}

.pillars {
  display: grid;
  gap: 1rem;
  grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));
}
.pillar {
  background: rgba(255, 255, 255, 0.38);
  backdrop-filter: blur(var(--glass-blur)) saturate(var(--glass-saturate)) brightness(var(--glass-bright));
  -webkit-backdrop-filter: blur(var(--glass-blur)) saturate(var(--glass-saturate)) brightness(var(--glass-bright));
  border: 1px solid rgba(255, 255, 255, 0.55);
  border-radius: 24px 28px 22px 30px / 28px 22px 30px 24px;
  padding: 1.35rem 1.25rem;
  box-shadow: var(--glass-shadow);
  transition: transform 0.25s, box-shadow 0.25s, border-color 0.25s;
  overflow: hidden;
}
.pillar:hover {
  transform: translateY(-3px);
  border-color: rgba(255, 255, 255, 0.98);
  box-shadow: var(--glass-shadow-lg);
}
.pillar-icon {
  width: 2.5rem;
  height: 2.5rem;
  border-radius: 12px;
  background: rgba(255, 255, 255, 0.7);
  border: 1px solid var(--border-glass);
  color: var(--text);
  display: grid;
  place-items: center;
  font-size: 1.15rem;
  margin-bottom: 0.85rem;
  box-shadow: inset 0 1px 0 rgba(255, 255, 255, 1);
}
.pillar h3 { margin: 0 0 0.45rem; font-size: 1.02rem; color: var(--text); }
.pillar p { margin: 0; color: var(--muted); font-size: 0.88rem; line-height: 1.55; }

.pipeline {
  display: grid;
  gap: 1rem;
  grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));
}
.pipeline-step {
  position: relative;
  background: rgba(255, 255, 255, 0.55);
  backdrop-filter: blur(var(--glass-blur)) saturate(var(--glass-saturate));
  border: 1px solid var(--border-glass);
  border-radius: var(--radius-md);
  padding: 1.25rem 1.15rem 1.15rem;
  box-shadow: var(--glass-shadow);
}
.pipeline-num {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 1.75rem;
  height: 1.75rem;
  border-radius: 9px;
  background: linear-gradient(145deg, #1e293b, #475569);
  color: white;
  font-size: 0.82rem;
  font-weight: 700;
  margin-bottom: 0.75rem;
  box-shadow: 0 4px 12px rgba(15, 23, 42, 0.15);
}
.pipeline-step h3 { margin: 0 0 0.4rem; font-size: 1rem; }
.pipeline-step p { margin: 0; color: var(--muted); font-size: 0.88rem; line-height: 1.5; }

.showcase { display: grid; gap: 2rem; align-items: center; }
@media (min-width: 900px) {
  .showcase { grid-template-columns: 1fr 1fr; gap: 3rem; }
  .showcase-reverse .showcase-copy { order: 2; }
  .showcase-reverse .showcase-code { order: 1; }
}
.showcase-copy h3 { font-size: 1.35rem; margin: 0 0 0.65rem; letter-spacing: -0.02em; font-weight: 700; }
.showcase-copy p { color: var(--muted); margin: 0 0 1rem; line-height: 1.6; }
.showcase-list { margin: 0; padding: 0; list-style: none; display: flex; flex-direction: column; gap: 0.55rem; }
.showcase-list li { display: flex; gap: 0.55rem; align-items: flex-start; color: var(--muted); font-size: 0.92rem; }
.showcase-list li::before { content: "✓"; color: var(--success); font-weight: 700; flex-shrink: 0; margin-top: 0.05rem; }
.showcase-code .code-window {
  background: rgba(255, 255, 255, 0.58);
  backdrop-filter: blur(var(--glass-blur)) saturate(var(--glass-saturate));
  border: 1px solid var(--border-glass);
  border-radius: var(--radius-md);
  overflow: hidden;
  box-shadow: var(--glass-shadow-lg);
}
.showcase-code pre.code { margin: 0; border: 0; border-radius: 0; font-size: 0.8rem; }

.grid-3 {
  display: grid;
  gap: 1rem;
  grid-template-columns: repeat(auto-fit, minmax(240px, 1fr));
}
.card {
  background: rgba(255, 255, 255, 0.38);
  backdrop-filter: blur(var(--glass-blur)) saturate(var(--glass-saturate)) brightness(var(--glass-bright));
  -webkit-backdrop-filter: blur(var(--glass-blur)) saturate(var(--glass-saturate)) brightness(var(--glass-bright));
  border: 1px solid rgba(255, 255, 255, 0.55);
  border-radius: 22px 26px 24px 28px / 26px 24px 28px 22px;
  padding: 1.2rem;
  box-shadow: var(--glass-shadow);
  transition: transform 0.25s, box-shadow 0.25s;
  overflow: hidden;
}
.card:hover {
  transform: translateY(-2px);
  border-color: rgba(255, 255, 255, 0.98);
  box-shadow: var(--glass-shadow-lg);
}
.card h3 { margin: 0 0 0.4rem; font-size: 1rem; color: var(--text); }
.card p { margin: 0; color: var(--muted); font-size: 0.9rem; }
.card-icon { font-size: 1.35rem; margin-bottom: 0.5rem; }

.package-diagram {
  display: grid;
  gap: 1rem;
  align-items: stretch;
  grid-template-columns: repeat(auto-fit, minmax(260px, 1fr));
}
.package-box {
  border: 1px solid var(--border-glass);
  border-radius: var(--radius-lg);
  padding: 1.5rem 1.35rem;
  background: rgba(255, 255, 255, 0.58);
  backdrop-filter: blur(var(--glass-blur)) saturate(var(--glass-saturate));
  box-shadow: var(--glass-shadow);
}
.package-box h3 { margin: 0 0 0.3rem; font-size: 1.1rem; }
.package-box .tag { color: var(--muted); font-size: 0.72rem; font-weight: 700; letter-spacing: 0.06em; }
.package-box ul { margin: 0.85rem 0 0; padding-left: 1.15rem; color: var(--muted); font-size: 0.88rem; line-height: 1.65; }
.package-plus { display: none; }
@media (min-width: 768px) {
  .package-diagram { grid-template-columns: 1fr auto 1fr; }
  .package-plus {
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 2rem;
    color: var(--muted);
    font-weight: 300;
  }
}

.compare-wrap,
.bench-wrap {
  overflow-x: auto;
  border-radius: var(--radius-md);
  border: 1px solid var(--border-glass);
  background: rgba(255, 255, 255, 0.45);
  backdrop-filter: blur(12px);
  box-shadow: var(--glass-shadow);
}
.compare-3 { display: grid; gap: 1rem; grid-template-columns: repeat(3, 1fr); }
@media (max-width: 768px) { .compare-3 { grid-template-columns: 1fr; } }
.compare-column {
  background: rgba(255, 255, 255, 0.5);
  border: 1px solid var(--border-glass);
  border-radius: var(--radius-md);
  padding: 1.35rem 1.25rem;
  box-shadow: var(--glass-shadow);
}
.compare-column h3 { margin: 0 0 0.65rem; font-size: 1rem; color: var(--text); line-height: 1.35; }
.compare-column p { margin: 0; color: var(--muted); font-size: 0.92rem; line-height: 1.55; }
.compare-column-highlight {
  background: rgba(255, 255, 255, 0.78);
  border-color: rgba(255, 255, 255, 0.98);
  box-shadow: var(--glass-shadow-lg);
}
.compare-column-highlight h3 { color: var(--text); }

.bench { width: 100%; border-collapse: collapse; font-size: 0.92rem; margin: 0; }
.bench th, .bench td { padding: 0.9rem 1rem; text-align: left; border-bottom: 1px solid var(--border); }
.bench th { background: rgba(255, 255, 255, 0.5); color: var(--muted); font-weight: 600; font-size: 0.82rem; }
.bench tr:last-child td { border-bottom: 0; }
.bench-row-highlight td { background: rgba(255, 255, 255, 0.55); }
.bench-row-highlight td:first-child { color: var(--text); font-weight: 600; }
.bench td.yes { color: var(--success); font-weight: 600; }
.bench-note { color: var(--muted); font-size: 0.88rem; margin: 0.75rem 0 0; }

.doc-link-card {
  display: block;
  position: relative;
  background: rgba(255, 255, 255, 0.55);
  backdrop-filter: blur(var(--glass-blur)) saturate(var(--glass-saturate));
  border: 1px solid var(--border-glass);
  border-radius: var(--radius-md);
  padding: 1.15rem 2.5rem 1.15rem 1.15rem;
  text-decoration: none !important;
  box-shadow: var(--glass-shadow);
  transition: transform 0.2s, box-shadow 0.2s, border-color 0.2s;
}
.doc-link-card:hover {
  border-color: rgba(255, 255, 255, 0.98);
  box-shadow: var(--glass-shadow-lg);
  transform: translateY(-2px);
}
.doc-link-card h3 { margin: 0 0 0.35rem; font-size: 1rem; color: var(--text); }
.doc-link-card p { margin: 0; color: var(--muted); font-size: 0.88rem; line-height: 1.5; }
.doc-card-tag {
  display: inline-block;
  margin-bottom: 0.45rem;
  padding: 0.15rem 0.5rem;
  border-radius: 999px;
  background: rgba(255, 255, 255, 0.65);
  border: 1px solid var(--border-glass);
  color: var(--muted);
  font-size: 0.68rem;
  font-weight: 700;
  letter-spacing: 0.04em;
  text-transform: uppercase;
}
.doc-card-arrow {
  position: absolute;
  right: 1rem;
  top: 50%;
  transform: translateY(-50%);
  color: var(--muted);
  font-weight: 600;
}

/* ── Docs hub ───────────────────────────────────────────────────────── */

.docs-hub { max-width: 52rem; }
.docs-hero {
  margin: -0.25rem -1.25rem 2rem;
  padding: 2rem 1.25rem 2.25rem;
  background: rgba(255, 255, 255, 0.45);
  backdrop-filter: blur(var(--glass-blur)) saturate(var(--glass-saturate));
  border: 1px solid var(--border-glass);
  border-radius: var(--radius-lg);
  box-shadow: var(--glass-shadow);
}
@media (min-width: 960px) { .docs-hero { padding: 2.5rem 1.25rem 2.75rem; } }
.docs-hero h1 { font-size: clamp(1.85rem, 4vw, 2.35rem); margin: 0 0 0.65rem; letter-spacing: -0.03em; }
.docs-hero-lead { color: var(--muted); font-size: 1.05rem; line-height: 1.6; margin: 0 0 1.25rem; max-width: 38rem; }
.docs-search-hero { display: flex; gap: 0.5rem; max-width: 28rem; margin-bottom: 1rem; }
.docs-search-hero input[type="search"] {
  flex: 1;
  padding: 0.6rem 0.85rem;
  border: 1px solid var(--border-glass);
  border-radius: 999px;
  font: inherit;
  background: rgba(255, 255, 255, 0.55);
  backdrop-filter: blur(12px);
  color: var(--text);
  box-shadow: inset 0 1px 0 rgba(255, 255, 255, 0.9);
}
.docs-search-hero input[type="search"]:focus {
  outline: none;
  border-color: rgba(255, 255, 255, 1);
  box-shadow: 0 0 0 3px rgba(37, 99, 235, 0.12);
}
.docs-search-hero button {
  padding: 0.6rem 1.1rem;
  border: 0;
  border-radius: 999px;
  background: linear-gradient(145deg, #1e293b, #334155);
  color: white;
  font-weight: 600;
  cursor: pointer;
  font: inherit;
  box-shadow: 0 4px 16px rgba(15, 23, 42, 0.15);
}
.docs-quick-links { display: flex; flex-wrap: wrap; gap: 0.5rem 1rem; font-size: 0.88rem; }
.docs-quick-links a { font-weight: 500; }

.docs-stat-strip {
  display: grid;
  gap: 0.65rem;
  grid-template-columns: repeat(auto-fit, minmax(140px, 1fr));
  margin: 0 0 2rem;
  padding: 1rem;
  border-radius: var(--radius-md);
  background: rgba(255, 255, 255, 0.5);
  backdrop-filter: blur(16px);
  border: 1px solid var(--border-glass);
  box-shadow: var(--glass-shadow);
}
.docs-stat { text-align: center; }
.docs-stat strong { display: block; font-size: 1.35rem; color: var(--text); }
.docs-stat span { font-size: 0.78rem; color: var(--muted); }

.learn-paths {
  display: grid;
  gap: 1rem;
  grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));
  margin-bottom: 2.5rem;
}
.learn-path-card {
  border: 1px solid var(--border-glass);
  border-radius: var(--radius-md);
  padding: 1.25rem;
  background: rgba(255, 255, 255, 0.55);
  backdrop-filter: blur(var(--glass-blur)) saturate(var(--glass-saturate));
  box-shadow: var(--glass-shadow);
}
.learn-path-step {
  display: inline-flex;
  width: 1.65rem;
  height: 1.65rem;
  border-radius: 9px;
  align-items: center;
  justify-content: center;
  background: linear-gradient(145deg, #1e293b, #475569);
  color: white;
  font-size: 0.78rem;
  font-weight: 700;
  margin-bottom: 0.65rem;
}
.learn-path-card h3 { margin: 0 0 0.4rem; font-size: 1.02rem; }
.learn-path-card p { margin: 0 0 0.85rem; color: var(--muted); font-size: 0.88rem; line-height: 1.5; }
.learn-path-link { font-weight: 600; font-size: 0.88rem; text-decoration: none !important; }

.docs-section-title { font-size: 1.15rem; margin: 2rem 0 1rem; letter-spacing: -0.02em; font-weight: 700; }
.docs-section-title:first-of-type { margin-top: 0; }

.compare { width: 100%; border-collapse: collapse; font-size: 0.9rem; border: 0; margin: 0; }
.compare th, .compare td { padding: 0.85rem 1rem; text-align: left; border-bottom: 1px solid var(--border); }
.compare th { background: rgba(255, 255, 255, 0.5); color: var(--muted); font-weight: 600; font-size: 0.82rem; }
.compare th:last-child { color: var(--text); }
.compare tr:last-child td { border-bottom: 0; }
.compare td:first-child { font-weight: 600; color: var(--text); white-space: nowrap; }
.compare .yes { color: var(--success); font-weight: 600; }

.cta-section { max-width: 80rem; margin: 0 auto; padding: 0 1.25rem 4rem; }
.cta-banner {
  background: rgba(255, 255, 255, 0.62);
  backdrop-filter: blur(var(--glass-blur)) saturate(var(--glass-saturate));
  border: 1px solid var(--border-glass);
  border-radius: var(--radius-xl);
  padding: 2.75rem 2rem;
  text-align: center;
  color: var(--text);
  box-shadow: var(--glass-shadow-lg);
  position: relative;
  overflow: hidden;
}
.cta-banner::before {
  content: "";
  position: absolute;
  inset: 0;
  background: linear-gradient(135deg, rgba(255,255,255,0.9) 0%, rgba(241,245,249,0.4) 50%, rgba(255,255,255,0.7) 100%);
  pointer-events: none;
}
.cta-banner > * { position: relative; z-index: 1; }
.cta-banner h2 {
  margin: 0 0 0.65rem;
  font-size: clamp(1.5rem, 3vw, 2rem);
  letter-spacing: -0.03em;
  font-weight: 800;
  color: var(--text);
}
.cta-banner p { margin: 0 auto 1.5rem; max-width: 32rem; color: var(--muted); line-height: 1.55; }
.cta-banner .btn-primary {
  font-size: 1rem;
  padding: 0.75rem 1.6rem;
}
.cta-banner .btn-primary:hover { color: white; }
.cta-install {
  margin-top: 1.25rem;
  font-family: var(--mono);
  font-size: 0.82rem;
  background: rgba(255, 255, 255, 0.55);
  border: 1px solid var(--border-glass);
  display: inline-block;
  padding: 0.5rem 1rem;
  border-radius: var(--radius-sm);
  color: var(--muted);
}

/* ── Footer ─────────────────────────────────────────────────────────── */

.site-footer {
  border-top: 1px solid rgba(255, 255, 255, 0.65);
  padding: 2.25rem 1.25rem;
  text-align: center;
  color: var(--muted);
  font-size: 0.85rem;
  background: rgba(255, 255, 255, 0.35);
  backdrop-filter: blur(16px);
}
.site-footer-links { display: flex; flex-wrap: wrap; justify-content: center; gap: 0.35rem 1rem; margin-top: 0.65rem; }
.site-footer a { color: var(--muted); }
.site-footer a:hover { color: var(--text); }

/* ── Docs shell ─────────────────────────────────────────────────────── */

.docs-shell {
  max-width: 80rem;
  margin: 0 auto;
  padding: 1.25rem 1.25rem 3rem;
  display: grid;
  gap: 2rem;
}
@media (min-width: 960px) {
  .docs-shell { grid-template-columns: var(--sidebar-w) 1fr; align-items: start; }
}

.docs-sidebar {
  position: sticky;
  top: 4.5rem;
  max-height: calc(100vh - 5.5rem);
  overflow-y: auto;
  overflow-x: hidden;
  padding: 1rem;
  border-radius: 30px 26px 28px 32px / 26px 30px 32px 28px;
  background: rgba(255, 255, 255, 0.32);
  backdrop-filter: blur(var(--glass-blur)) saturate(var(--glass-saturate)) brightness(var(--glass-bright));
  -webkit-backdrop-filter: blur(var(--glass-blur)) saturate(var(--glass-saturate)) brightness(var(--glass-bright));
  border: 1px solid rgba(255, 255, 255, 0.55);
  box-shadow: var(--glass-shadow);
}
.docs-sidebar h4 {
  margin: 1.25rem 0 0.5rem;
  font-size: 0.68rem;
  text-transform: uppercase;
  letter-spacing: 0.07em;
  color: var(--muted);
  font-weight: 700;
}
.docs-sidebar h4:first-child { margin-top: 0; }
.docs-sidebar nav { display: flex; flex-direction: column; gap: 0.05rem; margin-bottom: 0.25rem; }

.docs-search-form { display: flex; gap: 0.35rem; margin-bottom: 1rem; }
.docs-search-form input[type="search"] {
  flex: 1;
  padding: 0.45rem 0.65rem;
  border: 1px solid var(--border-glass);
  border-radius: 999px;
  font: inherit;
  background: rgba(255, 255, 255, 0.55);
  backdrop-filter: blur(8px);
  color: var(--text);
}
.docs-search-form button {
  padding: 0.45rem 0.75rem;
  border: 1px solid var(--border-glass);
  border-radius: 999px;
  background: rgba(255, 255, 255, 0.65);
  cursor: pointer;
  font: inherit;
}

.docs-search-results { list-style: none; padding: 0; margin: 1rem 0; }
.docs-search-results li { margin-bottom: 0.65rem; }
.docs-search-results a { display: flex; flex-direction: column; gap: 0.15rem; text-decoration: none; }
.docs-search-results span { color: var(--muted); font-size: 0.85rem; }

.docs-sidebar a {
  padding: 0.35rem 0.6rem;
  border-radius: 8px;
  color: var(--muted);
  font-size: 0.875rem;
  text-decoration: none;
  line-height: 1.35;
  transition: background 0.12s, color 0.12s;
}
.docs-sidebar a.active {
  background: rgba(255, 255, 255, 0.78);
  color: var(--text);
  font-weight: 600;
  box-shadow: inset 0 1px 0 rgba(255, 255, 255, 1);
}
.docs-sidebar a:hover {
  background: rgba(255, 255, 255, 0.55);
  color: var(--text);
  text-decoration: none;
}

.docs-main { min-width: 0; max-width: 48rem; position: relative; }
.docs-copy-toolbar {
  display: flex;
  justify-content: flex-end;
  margin: 0 0 1rem;
  position: sticky;
  top: 0.75rem;
  z-index: 5;
}
.docs-copy-toolbar .btn {
  backdrop-filter: blur(16px);
  -webkit-backdrop-filter: blur(16px);
  background: rgba(255, 255, 255, 0.72);
  border: 1px solid var(--border-glass);
  box-shadow: var(--glass-shadow);
}
.docs-section-head {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: 0.75rem;
  margin: 2rem 0 0.65rem;
  padding-top: 0.5rem;
  border-top: 1px solid rgba(255, 255, 255, 0.65);
}
.docs-section-head:first-of-type {
  border-top: 0;
  margin-top: 1.25rem;
}
.docs-section-head h2 {
  margin: 0 !important;
  padding-top: 0 !important;
  border-top: 0 !important;
  flex: 1;
  min-width: 0;
}
.docs-copy-section {
  flex-shrink: 0;
  margin-top: 0.1rem;
  font-size: 0.78rem !important;
  padding: 0.28rem 0.55rem !important;
  opacity: 0.85;
}
.docs-copy-section:hover { opacity: 1; }
.live-demo-header .docs-copy-live {
  margin-left: auto;
  flex-shrink: 0;
}
.docs-main:has(.docs-hub) { max-width: 52rem; }
.docs-main h1 { font-size: 2rem; margin: 0 0 0.5rem; letter-spacing: -0.03em; color: var(--text); }
.stream-skeleton {
  padding: 1rem 1.1rem;
  border-radius: var(--radius-sm);
  background: rgba(255, 255, 255, 0.55);
  border: 1px dashed var(--border-glass);
  color: var(--text-muted);
  animation: pulse 1.2s ease-in-out infinite;
}
.todo-demo-users { display: flex; align-items: center; gap: 0.5rem; flex-wrap: wrap; margin: 0.35rem 0; }
.todo-demo-users__label { color: var(--text-muted); font-size: 0.88rem; }
.todo-demo-users__current { font-family: var(--font-mono, ui-monospace, monospace); }
.todo-demo-users__badge {
  font-size: 0.72rem;
  text-transform: uppercase;
  letter-spacing: 0.04em;
  padding: 0.12rem 0.45rem;
  border-radius: 999px;
  background: rgba(113, 44, 249, 0.18);
  color: #c4b5fd;
}
.todo-demo-user-row { flex-wrap: wrap; gap: 0.35rem; }
.todo-demo-owner {
  font-size: 0.75rem;
  font-family: var(--font-mono, ui-monospace, monospace);
  color: var(--text-muted);
  min-width: 3rem;
}
.todo-demo-list { list-style: none; padding: 0; margin: 0.75rem 0 0; }
.todo-demo-item { display: flex; align-items: center; gap: 0.5rem; margin: 0.35rem 0; }
.todo-demo-done span:last-child { text-decoration: line-through; opacity: 0.7; }
.tw-demo-card {
  margin-top: 0.75rem;
  padding: 1rem 1.1rem;
  border-radius: 0.65rem;
  color: #fff;
  max-width: 28rem;
}
.tw-demo-purple { background: linear-gradient(135deg, #7c3aed, #a855f7); }
.tw-demo-emerald { background: linear-gradient(135deg, #059669, #10b981); }
.tw-demo-rose { background: linear-gradient(135deg, #e11d48, #f43f5e); }
.tw-demo-title { margin: 0 0 0.35rem; font-weight: 700; font-size: 0.95rem; font-family: var(--font-mono, ui-monospace, monospace); }
.tw-demo-body { margin: 0; font-size: 0.88rem; opacity: 0.92; }
.scheduler-demo-out, .queue-demo-out, .tools-demo-out { white-space: pre-wrap; min-height: 2.5rem; }

.flow-ui-demo { display: flex; flex-direction: column; gap: 1.5rem; }
.flow-ui-demo-section { display: flex; flex-direction: column; gap: 0.65rem; }
.flow-ui-demo-heading {
  margin: 0;
  font-size: 0.95rem;
  font-weight: 600;
  color: var(--text);
  letter-spacing: 0.01em;
}
.flow-ui-demo-hint { margin: 0; font-size: 0.84rem; }
.flow-ui-demo-dash .r-flow-dash { margin: 0; }
.flow-ui-demo-controls { max-width: 28rem; }
.tools-demo { display: flex; flex-direction: column; gap: 0.75rem; max-width: 32rem; }
.docs-main h2 {
  font-size: 1.25rem;
  margin: 2rem 0 0.65rem;
  padding-top: 0.5rem;
  border-top: 1px solid rgba(255, 255, 255, 0.65);
  color: var(--text);
}
.docs-main h2:first-of-type { border-top: 0; margin-top: 1.25rem; }
.docs-main h3 { font-size: 1.05rem; margin: 1.25rem 0 0.5rem; color: var(--text); }
.docs-main p, .docs-main li { color: var(--muted); }
.docs-main strong { color: var(--text); }
.docs-main ul, .docs-main ol { padding-left: 1.25rem; }
.docs-main .lead { font-size: 1.08rem; color: var(--muted); margin-bottom: 1.25rem; }

.docs-callout {
  border-left: 3px solid rgba(15, 23, 42, 0.15);
  background: rgba(255, 255, 255, 0.55);
  backdrop-filter: blur(12px);
  padding: 0.85rem 1rem;
  border-radius: 0 var(--radius-sm) var(--radius-sm) 0;
  margin: 1rem 0;
  font-size: 0.92rem;
  color: var(--muted);
  border: 1px solid var(--border-glass);
  border-left-width: 3px;
}

.docs-table {
  width: 100%;
  border-collapse: collapse;
  font-size: 0.9rem;
  border: 1px solid var(--border-glass);
  border-radius: var(--radius-sm);
  overflow: hidden;
  background: rgba(255, 255, 255, 0.45);
  backdrop-filter: blur(12px);
  box-shadow: var(--glass-shadow);
}
.docs-table th, .docs-table td { padding: 0.65rem 0.85rem; text-align: left; border-bottom: 1px solid var(--border); }
.docs-table th { background: rgba(255, 255, 255, 0.55); color: var(--muted); font-weight: 600; }
.docs-table tr:last-child td { border-bottom: 0; }

.playground-grid {
  display: grid;
  gap: 0.85rem;
  grid-template-columns: repeat(auto-fit, minmax(240px, 1fr));
  margin: 1rem 0 1.5rem;
}
.playground-card {
  display: block;
  background: rgba(255, 255, 255, 0.55);
  backdrop-filter: blur(var(--glass-blur)) saturate(var(--glass-saturate));
  border: 1px solid var(--border-glass);
  border-radius: var(--radius-md);
  padding: 1rem;
  text-decoration: none !important;
  box-shadow: var(--glass-shadow);
  transition: transform 0.2s, box-shadow 0.2s;
}
.playground-card:hover {
  transform: translateY(-2px);
  box-shadow: var(--glass-shadow-lg);
}
.playground-card h3 { margin: 0 0 0.3rem; color: var(--text); font-size: 1rem; }
.playground-card p { margin: 0 0 0.65rem; color: var(--muted); font-size: 0.88rem; }
.playground-card code {
  display: block;
  font-size: 0.8rem;
  color: var(--text);
  background: rgba(255, 255, 255, 0.5);
  padding: 0.45rem 0.55rem;
  border-radius: 8px;
  border: 1px solid var(--border-glass);
}

.template-grid {
  display: grid;
  gap: 0.65rem;
  margin: 0.85rem 0 1.25rem;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
}
.template-pill {
  background: rgba(255, 255, 255, 0.5);
  backdrop-filter: blur(12px);
  border: 1px solid var(--border-glass);
  border-radius: var(--radius-sm);
  padding: 0.65rem 0.85rem;
  font-size: 0.88rem;
  box-shadow: var(--glass-shadow);
  transition: transform 0.15s;
}
.template-pill:hover { transform: translateY(-1px); }
.template-pill strong { display: block; color: var(--text); margin-bottom: 0.2rem; }
.template-pill span { color: var(--muted); font-size: 0.82rem; }

/* ── Demos ──────────────────────────────────────────────────────────── */

.server-demo {
  background: rgba(255, 255, 255, 0.55);
  backdrop-filter: blur(var(--glass-blur)) saturate(var(--glass-saturate));
  border: 1px solid var(--border-glass);
  border-radius: var(--radius-md);
  padding: 1.25rem 1.35rem;
  max-width: 42rem;
  box-shadow: var(--glass-shadow);
}
.server-demo h3 { margin: 0 0 0.35rem; font-size: 1.15rem; color: var(--text); }
.server-demo-lead { margin: 0 0 1rem; color: var(--muted); font-size: 0.95rem; }
.server-demo-row { display: flex; flex-wrap: wrap; gap: 0.75rem; align-items: flex-end; }
.server-demo-label {
  display: flex;
  flex-direction: column;
  gap: 0.35rem;
  font-size: 0.85rem;
  color: var(--muted);
  flex: 1;
  min-width: 12rem;
}
.server-demo-label input {
  font: inherit;
  padding: 0.5rem 0.7rem;
  border: 1px solid var(--border-glass);
  border-radius: var(--radius-sm);
  background: rgba(255, 255, 255, 0.55);
  backdrop-filter: blur(8px);
}
.server-demo-ok, .server-demo-err {
  margin: 0.85rem 0 0;
  padding: 0.65rem 0.85rem;
  border-radius: var(--radius-sm);
  font-size: 0.92rem;
  display: none;
}
.server-demo-ok {
  background: rgba(236, 253, 245, 0.75);
  color: var(--success);
  border: 1px solid rgba(167, 243, 208, 0.8);
  backdrop-filter: blur(8px);
}
.server-demo-err {
  background: rgba(254, 242, 242, 0.75);
  color: var(--danger);
  border: 1px solid rgba(254, 202, 202, 0.8);
  backdrop-filter: blur(8px);
}
.server-demo-ok:not([hidden]), .server-demo-err:not([hidden]) { display: block; }

.exec-demo {
  background: rgba(255, 255, 255, 0.55);
  backdrop-filter: blur(var(--glass-blur)) saturate(var(--glass-saturate));
  border: 1px solid var(--border-glass);
  border-radius: var(--radius-lg);
  padding: 1.35rem 1.45rem;
  box-shadow: var(--glass-shadow);
}
.exec-demo-intro h3 { margin: 0 0 0.35rem; font-size: 1.15rem; color: var(--text); }
.exec-demo-lead { margin: 0 0 1rem; color: var(--muted); font-size: 0.95rem; max-width: 52rem; }
.exec-demo-grid { display: grid; gap: 1.25rem; grid-template-columns: 1fr; }
.exec-demo-grid--single { max-width: 28rem; }
@media (min-width: 960px) {
  .exec-demo-grid { grid-template-columns: minmax(16rem, 22rem) 1fr; align-items: start; }
}
.exec-demo-controls { display: flex; flex-direction: column; gap: 0.75rem; }
.exec-demo-label {
  display: flex;
  flex-direction: column;
  gap: 0.35rem;
  font-size: 0.85rem;
  color: var(--muted);
}
.exec-demo-label input,
.exec-demo-label textarea {
  font: inherit;
  padding: 0.5rem 0.7rem;
  border: 1px solid var(--border-glass);
  border-radius: var(--radius-sm);
  background: rgba(255, 255, 255, 0.55);
  backdrop-filter: blur(8px);
  resize: vertical;
}
.exec-demo-err {
  margin: 0;
  padding: 0.65rem 0.85rem;
  border-radius: var(--radius-sm);
  font-size: 0.92rem;
  background: rgba(254, 242, 242, 0.75);
  color: var(--danger);
  border: 1px solid rgba(254, 202, 202, 0.8);
  display: none;
}
.exec-demo-err:not([hidden]) { display: block; }
.exec-demo-hint { margin: 0; font-size: 0.88rem; color: var(--muted); }
.exec-demo-guide {
  margin: 0.25rem 0 0;
  padding: 1rem 1.1rem;
  border-radius: var(--radius-md);
  background: rgba(99, 102, 241, 0.08);
  border: 1px solid rgba(99, 102, 241, 0.22);
}
.exec-demo-guide__title {
  margin: 0 0 0.65rem;
  font-size: 0.92rem;
  font-weight: 600;
  color: var(--text);
}
.exec-demo-guide__steps {
  margin: 0;
  padding: 0;
  list-style: none;
  display: flex;
  flex-direction: column;
  gap: 0.55rem;
}
.exec-demo-guide__steps li {
  display: flex;
  gap: 0.65rem;
  align-items: flex-start;
  font-size: 0.88rem;
  color: var(--muted);
  line-height: 1.45;
}
.exec-demo-guide__num {
  flex: 0 0 1.35rem;
  height: 1.35rem;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  border-radius: 999px;
  font-size: 0.72rem;
  font-weight: 700;
  background: rgba(148, 163, 184, 0.25);
  color: var(--muted);
}
.exec-demo-guide__steps li.is-active {
  color: var(--text);
}
.exec-demo-guide__steps li.is-active .exec-demo-guide__num {
  background: var(--accent);
  color: #fff;
}
.exec-demo-guide__steps li.is-done {
  opacity: 0.72;
}
.exec-demo-guide__steps li.is-done .exec-demo-guide__num {
  background: rgba(34, 197, 94, 0.2);
  color: #15803d;
}
.exec-demo-guide__status {
  margin: 0.75rem 0 0;
  padding-top: 0.65rem;
  border-top: 1px solid rgba(99, 102, 241, 0.15);
  font-size: 0.84rem;
  color: var(--accent);
  font-weight: 500;
}
.exec-flow-placeholder {
  margin-top: 1.25rem;
  padding: 1.25rem 1.35rem;
  border-radius: var(--radius-lg);
  border: 2px dashed rgba(148, 163, 184, 0.45);
  background: rgba(248, 250, 252, 0.55);
  text-align: center;
}
.exec-flow-placeholder__title {
  margin: 0 0 0.35rem;
  font-size: 0.95rem;
  font-weight: 600;
  color: var(--text);
}
.exec-flow-placeholder__text {
  margin: 0;
  font-size: 0.88rem;
  color: var(--muted);
}
.exec-flow-placeholder[hidden] { display: none; }
.exec-demo-dash .r-flow-dash { margin: 0; }
.exec-flow-slot { margin-top: 1.5rem; }
.exec-flow-slot:not([hidden]) { display: block; }

/* Liquid glass execution panel */
.exec-flow-slot .r-flow-exec {
  display: grid;
  gap: 1.15rem;
  margin: 0;
  padding: 1.15rem;
  border-radius: var(--radius-lg);
  background: rgba(255, 255, 255, 0.28);
  backdrop-filter: blur(22px) saturate(165%);
  -webkit-backdrop-filter: blur(22px) saturate(165%);
  border: 1px solid rgba(255, 255, 255, 0.72);
  box-shadow: var(--glass-shadow-lg);
}
@media (min-width: 900px) {
  .exec-flow-slot .r-flow-exec {
    grid-template-columns: minmax(0, 1.12fr) minmax(0, 0.88fr);
    align-items: start;
    padding: 1.25rem;
    gap: 1.25rem;
  }
}
.exec-flow-slot .r-flow-exec__side {
  display: flex;
  flex-direction: column;
  gap: 1rem;
  min-width: 0;
}
.exec-demo-dash .r-flow-dash,
.exec-flow-slot .r-flow-exec__panel {
  background: rgba(255, 255, 255, 0.62) !important;
  backdrop-filter: blur(18px) saturate(160%) !important;
  -webkit-backdrop-filter: blur(18px) saturate(160%) !important;
  border: 1px solid rgba(255, 255, 255, 0.88) !important;
  border-radius: 18px !important;
  padding: 1.15rem 1.2rem !important;
  box-shadow:
    0 10px 28px rgba(15, 23, 42, 0.05),
    inset 0 1px 0 rgba(255, 255, 255, 1) !important;
}
.exec-flow-slot .r-flow-graph {
  padding: 0 !important;
  background: transparent !important;
  border: 0 !important;
  box-shadow: none !important;
}
.exec-flow-slot .r-event-stream {
  padding: 0 !important;
  background: transparent !important;
  border: 0 !important;
  box-shadow: none !important;
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  min-height: 0;
}
.exec-flow-slot .r-flow-exec__panel h3 {
  margin: 0 0 0.85rem !important;
  font-size: 0.72rem !important;
  font-weight: 700 !important;
  letter-spacing: 0.07em !important;
  text-transform: uppercase !important;
  color: var(--muted) !important;
}
.exec-flow-slot .r-flow-graph__status {
  color: var(--muted) !important;
  margin-top: 0.65rem !important;
  line-height: 1.45 !important;
}
.exec-flow-slot .r-flow-graph__track {
  padding: 0.15rem 0 !important;
  min-height: 2.75rem !important;
}
.exec-flow-slot .r-flow-graph__node {
  background: rgba(255, 255, 255, 0.72) !important;
  border-color: rgba(15, 23, 42, 0.08) !important;
  color: var(--text) !important;
  border-radius: 999px !important;
  padding: 0.4rem 0.75rem !important;
}
.exec-flow-slot .r-flow-graph__node--running {
  border-color: rgba(37, 99, 235, 0.35) !important;
  background: rgba(219, 234, 254, 0.65) !important;
  color: #1d4ed8 !important;
}
.exec-flow-slot .r-flow-graph__node--done {
  border-color: rgba(5, 150, 105, 0.3) !important;
  background: rgba(209, 250, 229, 0.65) !important;
  color: #047857 !important;
}
.exec-flow-slot .r-flow-graph__node--paused {
  border-color: rgba(217, 119, 6, 0.35) !important;
  background: rgba(254, 243, 199, 0.7) !important;
  color: #b45309 !important;
}
.exec-flow-slot .r-flow-graph__node--failed {
  border-color: rgba(220, 38, 38, 0.3) !important;
  background: rgba(254, 226, 226, 0.7) !important;
  color: #b91c1c !important;
}
.exec-flow-slot .r-event-stream__viewport {
  min-height: 12rem;
  max-height: min(42vh, 22rem);
  overflow-x: hidden;
  overflow-y: auto;
  overflow-anchor: none;
  overscroll-behavior: contain;
  padding: 0.7rem 0.8rem;
  border-radius: 14px;
  border: 1px solid rgba(15, 23, 42, 0.07);
  background: rgba(248, 250, 252, 0.82);
  backdrop-filter: blur(12px);
  -webkit-backdrop-filter: blur(12px);
  scroll-behavior: smooth;
  -webkit-overflow-scrolling: touch;
  box-shadow: inset 0 1px 0 rgba(255, 255, 255, 0.95);
}
.exec-flow-slot .r-event-stream__viewport::-webkit-scrollbar {
  width: 7px;
}
.exec-flow-slot .r-event-stream__viewport::-webkit-scrollbar-thumb {
  background: rgba(148, 163, 184, 0.45);
  border-radius: 999px;
}
.exec-flow-slot .r-event-stream__viewport:has(.r-event-stream-list:empty)::before {
  content: "Waiting for events…";
  display: block;
  padding: 0.35rem 0.15rem;
  color: var(--muted);
  font-size: 0.78rem;
  font-style: italic;
}
.exec-flow-slot .r-event-stream-list {
  list-style: none !important;
  margin: 0 !important;
  padding: 0 !important;
  max-height: none !important;
  overflow: visible !important;
  font-family: var(--mono) !important;
  font-size: 0.74rem !important;
  line-height: 1.45 !important;
}
.exec-flow-slot .r-event-stream-list li {
  color: var(--muted) !important;
  padding: 0.45rem 0.6rem 0.45rem 0.75rem !important;
  margin: 0 0 0.3rem !important;
  border-radius: 10px !important;
  border: 1px solid rgba(15, 23, 42, 0.05) !important;
  background: rgba(255, 255, 255, 0.62) !important;
  border-bottom: 1px solid rgba(15, 23, 42, 0.05) !important;
}
.exec-flow-slot .r-event-stream-list li:last-child {
  margin-bottom: 0 !important;
  background: rgba(255, 255, 255, 0.88) !important;
  border-color: rgba(37, 99, 235, 0.12) !important;
  color: var(--text) !important;
  overflow-anchor: auto;
}
.exec-flow-slot .r-worker-panel {
  margin: 0;
}
.exec-flow-slot .r-worker-panel__actions {
  display: grid !important;
  grid-template-columns: repeat(2, minmax(0, 1fr)) !important;
  gap: 0.55rem !important;
}
@media (min-width: 520px) {
  .exec-flow-slot .r-worker-panel__actions {
    grid-template-columns: repeat(4, minmax(0, 1fr)) !important;
  }
}
.exec-flow-slot .r-worker-panel__actions .btn,
.exec-flow-slot .r-worker-panel__actions .r-flow-control {
  position: relative;
  isolation: isolate;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 100%;
  font: inherit;
  font-size: 0.8rem;
  font-weight: 600;
  line-height: 1.2;
  padding: 0.52rem 0.75rem;
  border-radius: 999px;
  cursor: pointer;
  text-decoration: none !important;
  color: var(--text);
  background: rgba(255, 255, 255, 0.18);
  backdrop-filter: blur(22px) saturate(190%) brightness(1.06);
  -webkit-backdrop-filter: blur(22px) saturate(190%) brightness(1.06);
  border: 1px solid rgba(255, 255, 255, 0.62);
  box-shadow:
    0 6px 28px rgba(15, 23, 42, 0.07),
    0 1px 2px rgba(15, 23, 42, 0.04),
    inset 0 1px 1px rgba(255, 255, 255, 0.98),
    inset 0 -10px 22px rgba(255, 255, 255, 0.12),
    inset 2px 2px 6px rgba(255, 255, 255, 0.38);
  transition: transform 0.2s ease, box-shadow 0.2s ease, background 0.2s ease, border-color 0.2s ease;
}
.exec-flow-slot .r-worker-panel__actions .r-flow-control:hover:not(:disabled),
.exec-flow-slot .r-worker-panel__actions .btn:hover:not(:disabled) {
  transform: translateY(-2px);
  border-color: rgba(255, 255, 255, 0.88);
  background: rgba(255, 255, 255, 0.28);
}
.exec-flow-slot .r-worker-panel__actions .r-flow-control:active:not(:disabled),
.exec-flow-slot .r-worker-panel__actions .btn:active:not(:disabled) {
  transform: translateY(0);
}
.exec-flow-slot .r-worker-panel__actions .r-flow-control:disabled,
.exec-flow-slot .r-worker-panel__actions .btn:disabled {
  opacity: 0.42;
  cursor: not-allowed;
  transform: none;
}
.exec-flow-slot .r-worker-panel__actions .r-flow-control--danger {
  color: #fff;
  background: rgba(15, 23, 42, 0.58);
  border-color: rgba(255, 255, 255, 0.42);
  backdrop-filter: blur(24px) saturate(180%) brightness(0.95);
  -webkit-backdrop-filter: blur(24px) saturate(180%) brightness(0.95);
  box-shadow:
    0 10px 36px rgba(15, 23, 42, 0.2),
    0 2px 8px rgba(15, 23, 42, 0.1),
    inset 0 1px 1px rgba(255, 255, 255, 0.55),
    inset 0 -12px 24px rgba(0, 0, 0, 0.12);
}
.exec-flow-slot .r-worker-panel__actions .r-flow-control--danger:hover:not(:disabled) {
  color: #fff;
  background: rgba(15, 23, 42, 0.68);
  border-color: rgba(255, 255, 255, 0.58);
}
.exec-flow-slot .r-worker-panel__actions .r-worker-cancel {
  color: #991b1b;
  border-color: rgba(248, 113, 113, 0.42);
  background: rgba(254, 242, 242, 0.42);
}
.exec-flow-slot .r-worker-panel__actions .r-worker-cancel:hover {
  color: #7f1d1d;
  background: rgba(254, 226, 226, 0.62);
}
/* Fallback when older markup injects bare buttons */
.exec-flow-slot .r-worker-panel:has(> button) {
  display: grid !important;
  grid-template-columns: repeat(2, minmax(0, 1fr)) !important;
  gap: 0.55rem !important;
}
@media (min-width: 520px) {
  .exec-flow-slot .r-worker-panel:has(> button) {
    grid-template-columns: repeat(4, minmax(0, 1fr)) !important;
  }
}
.exec-flow-slot .r-worker-panel__status {
  margin: 0.15rem 0 0;
  font-size: 0.78rem;
  color: var(--muted);
  line-height: 1.45;
  min-height: 1.15rem;
}

/* Live documentation demos */
.live-demo {
  background: rgba(255, 255, 255, 0.36);
  backdrop-filter: blur(calc(var(--glass-blur) + 4px)) saturate(var(--glass-saturate)) brightness(var(--glass-bright));
  -webkit-backdrop-filter: blur(calc(var(--glass-blur) + 4px)) saturate(var(--glass-saturate)) brightness(var(--glass-bright));
  border: 1px solid rgba(255, 255, 255, 0.6);
  border-radius: 32px 28px 30px 34px / 28px 32px 34px 30px;
  padding: 1.35rem 1.5rem;
  margin: 0 0 2rem;
  box-shadow:
    var(--glass-shadow-lg),
    0 14px 40px -10px rgba(167, 243, 208, 0.14),
    0 18px 48px -14px rgba(186, 230, 253, 0.16);
  overflow: hidden;
}
.live-demo::after { opacity: 0.68; }
.live-demo-info {
  background: rgba(255, 255, 255, 0.5);
  border-color: rgba(255, 255, 255, 0.85);
}
.live-demo-header { display: flex; align-items: center; gap: 0.75rem; margin-bottom: 1rem; }
.live-demo-badge {
  font-size: 0.68rem;
  font-weight: 800;
  letter-spacing: 0.06em;
  padding: 0.25rem 0.55rem;
  border-radius: 999px;
  background: linear-gradient(145deg, #1e293b, #475569);
  color: #fff;
  box-shadow: 0 2px 8px rgba(15, 23, 42, 0.12);
}
.live-demo-badge-info { background: linear-gradient(145deg, #475569, #64748b); }
.live-demo-title { margin: 0; font-size: 1.05rem; color: var(--text); font-weight: 700; }
.live-demo-body { font-size: 0.95rem; }

.demo-row { display: flex; flex-wrap: wrap; gap: 0.5rem; align-items: center; margin: 0.5rem 0; }
.demo-output { margin: 0.5rem 0; font-weight: 600; color: var(--text); }
.demo-muted { margin: 0.5rem 0; color: var(--muted); font-size: 0.9rem; }
.demo-error { color: var(--danger); font-weight: 600; }
.demo-slot-shell {
  border: 1px dashed rgba(15, 23, 42, 0.1);
  padding: 0.85rem;
  border-radius: var(--radius-sm);
  background: rgba(255, 255, 255, 0.35);
}
.demo-modal-backdrop {
  position: fixed;
  inset: 0;
  background: rgba(248, 250, 252, 0.55);
  backdrop-filter: blur(8px);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}
.demo-modal {
  background: rgba(255, 255, 255, 0.78);
  backdrop-filter: blur(var(--glass-blur)) saturate(var(--glass-saturate));
  padding: 1.35rem;
  border-radius: var(--radius-lg);
  max-width: 20rem;
  border: 1px solid var(--border-glass);
  box-shadow: var(--glass-shadow-lg);
}
.demo-theme-panel { padding: 0.75rem; border-radius: var(--radius-sm); }
.btn-sm { font-size: 0.85rem; padding: 0.4rem 0.8rem; }
.btn-themed {
  color: #fff;
  background: rgba(30, 41, 59, 0.62);
  border-color: rgba(255, 255, 255, 0.4);
}
#modals { position: relative; z-index: 999; }
.webhook-demo-output {
  margin-top: 0.75rem;
  max-height: 14rem;
  overflow: auto;
  white-space: pre-wrap;
  word-break: break-word;
  font-family: var(--mono);
  font-size: 0.72rem;
}
.webhook-demo-status {
  margin: 0.65rem 0 0;
  font-size: 0.82rem;
  color: var(--muted);
}
.r-visible-task-marker {
  position: absolute;
  width: 1px;
  height: 1px;
  opacity: 0;
  pointer-events: none;
  overflow: hidden;
}

/* Resuma SPA view transitions (NavLink + with_view_transition) */
[data-r-vt] {
  display: block;
  min-height: 0;
}

@media (prefers-reduced-motion: no-preference) {
  ::view-transition-old(root) {
    animation: r-vt-out 220ms cubic-bezier(0.4, 0, 1, 1) both;
  }
  ::view-transition-new(root) {
    animation: r-vt-in 320ms cubic-bezier(0, 0, 0.2, 1) both;
  }
}

@keyframes r-vt-out {
  from { opacity: 1; transform: translateY(0); }
  to { opacity: 0; transform: translateY(-6px); }
}

@keyframes r-vt-in {
  from { opacity: 0; transform: translateY(8px); }
  to { opacity: 1; transform: translateY(0); }
}
</style>"#;
