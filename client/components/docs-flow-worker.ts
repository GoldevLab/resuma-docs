/** Run docs_showcase + mount flow execution panel (keeps page handlers under inline size). */

type DemoKey = "flow-ui" | "exec";

type DemoIds = {
  topic: string;
  blurb: string;
  err: string;
  slot: string;
  btn: string;
};

const DEMOS: Record<DemoKey, DemoIds> = {
  "flow-ui": {
    topic: "flow-ui-topic",
    blurb: "flow-ui-blurb",
    err: "flow-ui-err",
    slot: "flow-ui-slot",
    btn: "flow-ui-start-btn",
  },
  exec: {
    topic: "exec-topic",
    blurb: "exec-blurb",
    err: "exec-err",
    slot: "exec-flow-slot",
    btn: "exec-start-btn",
  },
};

type SafeActionResult =
  | { ok: true; value: Record<string, unknown> }
  | { ok: false; error: string };

type ResumaGlobal = {
  safeAction(name: string, args: unknown[]): Promise<SafeActionResult>;
};

declare global {
  interface Window {
    __resuma?: ResumaGlobal;
    __resumaCoreReady?: Promise<void>;
  }
}

export async function runDocsFlowWorker(key: DemoKey): Promise<void> {
  const ids = DEMOS[key];
  const topic = (document.getElementById(ids.topic) as HTMLInputElement | null)?.value ?? "";
  const blurb =
    (document.getElementById(ids.blurb) as HTMLTextAreaElement | null)?.value ?? "";
  const errEl = document.getElementById(ids.err) as HTMLParagraphElement | null;
  const slot = document.getElementById(ids.slot) as HTMLDivElement | null;
  const btn = document.getElementById(ids.btn) as HTMLButtonElement | null;

  if (!errEl || !slot || !btn) return;

  const __resuma = window.__resuma;
  if (!__resuma) {
    errEl.textContent = "Resuma runtime not ready.";
    errEl.hidden = false;
    return;
  }

  errEl.hidden = true;
  btn.disabled = true;
  try {
    const res = await __resuma.safeAction("run_docs_flow_worker", [topic, blurb]);
    if (!res.ok) {
      errEl.textContent = res.error;
      errEl.hidden = false;
      return;
    }

    const panelHtml = String(res.value.panel_html ?? "");
    if (!panelHtml) {
      errEl.textContent = "Worker started but no panel HTML was returned.";
      errEl.hidden = false;
      return;
    }

    const prev = slot.querySelector("[data-r-flow-execution]");
    if (window.__resumaCoreReady) await window.__resumaCoreReady;

    let flow: {
      disconnectFlowWidgets: (el: Element) => void;
      initFlowWidgets: (root: HTMLElement, opts: { flush: boolean }) => void;
    };
    try {
      flow = await import("/_resuma/flow.js");
    } catch (e) {
      errEl.textContent = "Could not load Flow widgets: " + String(e);
      errEl.hidden = false;
      return;
    }

    if (prev) flow.disconnectFlowWidgets(slot);
    slot.innerHTML = "";
    slot.hidden = true;
    slot.innerHTML = panelHtml;
    slot.querySelectorAll("style[data-r-flow-styles]").forEach((n) => n.remove());
    slot.hidden = false;
    flow.initFlowWidgets(slot, { flush: false });
  } finally {
    btn.disabled = false;
  }
}
