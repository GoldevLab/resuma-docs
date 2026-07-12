/**
 * Copy buttons for docs pages — main content only (not sidebar/header).
 */

/** Mount Flow widgets outside dynamically injected exec panels (SSR dashboard only). */
export async function initDocsFlow() {
  const scope = document.querySelector<HTMLElement>(".docs-main");
  if (!scope) return;
  const hasDashboard = scope.querySelector(
    "[data-r-flow-dashboard]:not([data-docs-exec-panel] [data-r-flow-dashboard])",
  );
  const hasStaticGraph = Array.from(
    scope.querySelectorAll<HTMLElement>("[data-r-flow-graph]"),
  ).some((el) => !el.closest("[data-docs-exec-panel]"));
  if (!hasDashboard && !hasStaticGraph) return;
  try {
    if (window.__resumaCoreReady) await window.__resumaCoreReady;
    const mod = await import("/_resuma/flow.js");
    mod.initFlowWidgets(scope, {
      flush: false,
      exclude: "[data-docs-exec-panel]",
    });
  } catch (e) {
    console.warn("[docs-flow]", e);
  }
}

function flashCopied(btn: HTMLButtonElement, label = "Copied!") {
  const prev = btn.textContent;
  btn.textContent = label;
  btn.disabled = true;
  setTimeout(() => {
    btn.textContent = prev;
    btn.disabled = false;
  }, 1600);
}

async function copyText(text: string, btn: HTMLButtonElement) {
  const value = text.trim();
  if (!value) return;
  try {
    await navigator.clipboard.writeText(value);
    flashCopied(btn);
  } catch {
    const ta = document.createElement("textarea");
    ta.value = value;
    ta.setAttribute("readonly", "");
    ta.className = "docs-copy-fallback";
    document.body.appendChild(ta);
    ta.select();
    try {
      document.execCommand("copy");
      flashCopied(btn);
    } catch {
      flashCopied(btn, "Copy failed");
    }
    ta.remove();
  }
}

function sectionText(h2: HTMLHeadingElement) {
  const lines = [h2.innerText.trim()];
  const wrapper = h2.closest(".docs-section-head");
  let el: Element | null = wrapper ? wrapper.nextElementSibling : h2.nextElementSibling;
  while (el) {
    if (el.matches("h2, .docs-section-head, .docs-copy-toolbar")) break;
    if (!el.classList.contains("docs-copy-toolbar")) {
      const t = (el as HTMLElement).innerText.trim();
      if (t) lines.push(t);
    }
    el = el.nextElementSibling;
  }
  return lines.join("\n\n");
}

function pageText(main: HTMLElement) {
  const clone = main.cloneNode(true) as HTMLElement;
  clone
    .querySelectorAll(
      ".docs-copy-toolbar, .docs-section-head button, .docs-copy-section",
    )
    .forEach((n) => n.remove());
  clone.querySelectorAll(".docs-section-head").forEach((wrapper) => {
    const h2 = wrapper.querySelector("h2");
    if (h2) wrapper.replaceWith(h2);
  });
  return clone.innerText.trim();
}

function teardown(main: HTMLElement) {
  main.querySelectorAll(".docs-copy-toolbar").forEach((n) => n.remove());
  main.querySelectorAll(".docs-copy-live").forEach((n) => n.remove());
  main.querySelectorAll(".docs-section-head").forEach((wrapper) => {
    const h2 = wrapper.querySelector("h2");
    if (h2) wrapper.replaceWith(h2);
  });
}

function sidebarHrefMatches(href: string, current: string, exact: boolean): boolean {
  if (exact) {
    if (href === current) return true;
    const base = "http://resuma.local";
    const a = new URL(href, base);
    const b = new URL(current, base);
    if (a.search) return a.pathname + a.search === b.pathname + b.search;
    return a.pathname === b.pathname;
  }
  if (href === current) return true;
  const base = "http://resuma.local";
  const a = new URL(href, base);
  const b = new URL(current, base);
  if (a.search) return a.pathname + a.search === b.pathname + b.search;
  if (a.pathname === b.pathname) return true;
  if (a.pathname !== "/" && b.pathname.startsWith(a.pathname)) {
    const next = b.pathname.charCodeAt(a.pathname.length);
    return next === undefined || next === 47;
  }
  return false;
}

/** Keep docs sidebar active state in sync after SPA navigation. */
export function updateDocsSidebarNav(path = location.pathname + location.search): void {
  const sidebar = document.querySelector(".docs-sidebar");
  if (!sidebar) return;

  let best: HTMLAnchorElement | null = null;
  let bestLen = -1;

  sidebar.querySelectorAll<HTMLAnchorElement>("a[data-r-nav]").forEach((a) => {
    const href = a.getAttribute("href");
    if (!href) return;
    const exact = a.hasAttribute("data-r-nav-exact");
    if (!sidebarHrefMatches(href, path, exact)) return;
    if (href.length > bestLen) {
      best = a;
      bestLen = href.length;
    }
  });

  sidebar.querySelectorAll<HTMLAnchorElement>("a[data-r-nav]").forEach((a) => {
    const activeClass = a.getAttribute("data-r-active-class") ?? "docs-nav-link--active";
    const isBest = a === best;
    a.className = isBest ? `docs-nav-link ${activeClass}` : "docs-nav-link";
    if (isBest) a.setAttribute("aria-current", "page");
    else a.removeAttribute("aria-current");
  });
}

function scrollActiveSidebarLink() {
  const sidebar = document.querySelector<HTMLElement>(".docs-sidebar-scroll");
  const active =
    sidebar?.querySelector<HTMLElement>('.docs-nav-link--active[aria-current="page"]') ??
    sidebar?.querySelector<HTMLElement>(".docs-nav-link--active");
  if (!sidebar || !active) return;
  const top = active.offsetTop - sidebar.clientHeight / 2 + active.offsetHeight / 2;
  sidebar.scrollTo({ top: Math.max(0, top), behavior: "smooth" });
}

/** Refresh the Caching page live demo (headers + server stamp). */
export async function refreshCacheDemo(): Promise<void> {
  const root = document.querySelector<HTMLElement>("[data-docs-cache-demo]");
  if (!root) return;
  const headerEl = root.querySelector<HTMLElement>("[data-cache-header]");
  const stampEl = root.querySelector<HTMLElement>("[data-cache-stamp]");
  const statusEl = root.querySelector<HTMLElement>("[data-cache-status]");
  const r = window.__resuma;
  if (!r) return;
  try {
    const res = await fetch(location.pathname + location.search, {
      credentials: "same-origin",
      cache: "no-store",
    });
    if (headerEl) {
      headerEl.textContent = res.headers.get("cache-control") ?? "(none)";
    }
    const info = await r.safeAction("docs_cache_info", []);
    if (info.ok && stampEl) {
      stampEl.textContent = String(info.value.stamp ?? "—");
    }
    if (statusEl) {
      statusEl.textContent = `Headers refreshed (${res.status})`;
    }
  } catch {
    if (statusEl) statusEl.textContent = "Refresh failed";
  }
}

export function initCacheDemo(): void {
  void refreshCacheDemo();
}

export function initDocsSidebar() {
  const sync = () => {
    updateDocsSidebarNav();
    scrollActiveSidebarLink();
  };
  sync();
  document.addEventListener("resuma:navigate", sync);
}

export function initDocsCopy() {
  const main = document.querySelector<HTMLElement>(".docs-main");
  if (!main) return;

  teardown(main);

  const toolbar = document.createElement("div");
  toolbar.className = "docs-copy-toolbar";
  const pageBtn = document.createElement("button");
  pageBtn.type = "button";
  pageBtn.className = "btn btn-ghost btn-sm docs-copy-page";
  pageBtn.textContent = "Copy page";
  pageBtn.addEventListener("click", () => {
    void copyText(pageText(main), pageBtn);
  });
  toolbar.appendChild(pageBtn);
  main.insertBefore(toolbar, main.firstChild);

  main.querySelectorAll(".live-demo").forEach((section) => {
    const header = section.querySelector(".live-demo-header");
    if (!header || header.querySelector(".docs-copy-section")) return;
    const btn = document.createElement("button");
    btn.type = "button";
    btn.className = "btn btn-ghost btn-sm docs-copy-section docs-copy-live";
    btn.textContent = "Copy";
    const title = section.querySelector(".live-demo-title")?.textContent?.trim();
    if (title) btn.setAttribute("aria-label", `Copy demo: ${title}`);
    btn.addEventListener("click", () => {
      void copyText((section as HTMLElement).innerText.trim(), btn);
    });
    header.appendChild(btn);
  });

  main.querySelectorAll("h2").forEach((node) => {
    const h2 = node as HTMLHeadingElement;
    if (h2.closest(".docs-section-head, .docs-copy-toolbar, .live-demo-header")) return;
    const wrapper = document.createElement("div");
    wrapper.className = "docs-section-head";
    h2.parentNode?.insertBefore(wrapper, h2);
    wrapper.appendChild(h2);

    const btn = document.createElement("button");
    btn.type = "button";
    btn.className = "btn btn-ghost btn-sm docs-copy-section";
    btn.textContent = "Copy";
    btn.setAttribute("aria-label", `Copy section: ${h2.textContent}`);
    btn.addEventListener("click", () => {
      void copyText(sectionText(h2), btn);
    });
    wrapper.appendChild(btn);
  });
}
