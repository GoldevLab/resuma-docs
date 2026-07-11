/**
 * Copy buttons for docs pages — main content only (not sidebar/header).
 */

/** Mount Flow widgets (ops dashboard, etc.) inside docs content. */
export async function initDocsFlow() {
  const scope = document.querySelector<HTMLElement>('.docs-main');
  if (!scope?.querySelector('[data-r-flow-dashboard], [data-r-flow-graph]')) return;
  try {
    if (window.__resumaCoreReady) await window.__resumaCoreReady;
    const mod = await import('/_resuma/flow.js');
    mod.initFlowWidgets(scope);
  } catch (e) {
    console.warn('[docs-flow]', e);
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
    ta.style.position = "fixed";
    ta.style.left = "-9999px";
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
