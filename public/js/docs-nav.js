(() => {
  document.addEventListener("resuma:navigate", () => {
    document.documentElement.classList.remove("is-navigating");
    const heading = document.querySelector("[data-r-vt] h1");
    if (!heading) return;
    heading.setAttribute("tabindex", "-1");
    heading.focus({ preventScroll: true });
  });
  document.addEventListener("click", (e) => {
    const el = e.target instanceof Element ? e.target.closest("a[data-r-nav]") : null;
    if (el && el.getAttribute("href")) document.documentElement.classList.add("is-navigating");
  }, true);
  if (!window.__docsVtGuard) {
    window.__docsVtGuard = true;
    window.addEventListener("unhandledrejection", (e) => {
      const err = e.reason;
      const name = err && err.name;
      const msg = (err && (err.message || String(err))) || "";
      if (name === "AbortError" && /Transition was skipped|ViewTransition/i.test(msg)) {
        e.preventDefault();
      }
    });
  }
})();
