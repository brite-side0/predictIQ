# Styling Guide

## CSP Constraint: Inline `style` Props Are Not Allowed

This project runs under a strict **Content Security Policy (CSP)** in production. The policy does **not** include `'unsafe-inline'` for styles, which means:

- Inline `style` attributes on DOM elements are **stripped or blocked** by the browser.
- React's `style={{ ... }}` prop compiles down to an inline `style` attribute, so it is affected by the same restriction.
- The failure is **silent**: no exception is thrown, no console error is guaranteed, and the element simply renders without the intended styles. This makes it easy to ship a broken UI without noticing during development (where CSP is often relaxed).

Because of this, **do not use the `style` prop** on DOM elements or on components that forward it to the DOM. Use CSS classes or CSS modules instead.

## Required Pattern: CSS Classes / CSS Modules

The migration established the following pattern for all styling:

1. **Static styles** live in a stylesheet (global CSS or a CSS module).
2. **Dynamic styles** are expressed by toggling **class names**, not by computing inline style objects.
3. Components receive class names via the `className` prop.

### Incorrect

```tsx
// ❌ Inline style prop — stripped/blocked by CSP in production.
function ProgressBar({ percent }: { percent: number }) {
  return (
    <div
      className="progress"
      style={{ width: `${percent}%` }} // silently ignored under CSP
    />
  );
}
```

```tsx
// ❌ Passing a style object through to a DOM element.
function Badge({ color, children }: { color: string; children: React.ReactNode }) {
  return <span style={{ color }}>{children}</span>;
}
```

### Correct

Use a CSS module (or global class) and toggle classes:

```css
/* ProgressBar.module.css */
.progress {
  height: 4px;
  background: var(--color-accent);
}

.width0 { width: 0%; }
.width25 { width: 25%; }
.width50 { width: 50%; }
.width75 { width: 75%; }
.width100 { width: 100%; }
```

```tsx
// ✅ Class-based styling — CSP-safe.
import styles from "./ProgressBar.module.css";

const WIDTH_CLASS: Record<number, string> = {
  0: styles.width0,
  25: styles.width25,
  50: styles.width50,
  75: styles.width75,
  100: styles.width100,
};

function ProgressBar({ percent }: { percent: number }) {
  const widthClass = WIDTH_CLASS[percent] ?? styles.width0;
  return <div className={`${styles.progress} ${widthClass}`} />;
}
```

```tsx
// ✅ Variant classes instead of inline color.
import styles from "./Badge.module.css";

function Badge({ variant, children }: { variant: "info" | "warn"; children: React.ReactNode }) {
  return <span className={`${styles.badge} ${styles[variant]}`}>{children}</span>;
}
```

For values that are genuinely dynamic and cannot be enumerated as classes, prefer:

- A CSS custom property set via a class or a small set of classes, or
- A dedicated stylesheet rule keyed off a `data-*` attribute, or
- A CSS-in-JS solution that emits a real stylesheet (not inline attributes).

If you believe you have a case that truly requires an inline style, raise it for review before adding one — it will not work in production.

## Enforcement Gap

There is currently **no lint rule** that catches inline `style` props. This is why the CSP violations were able to land in the first place. To prevent regressions, add the following ESLint rules:

```jsonc
// .eslintrc (excerpt)
{
  "rules": {
    "react/forbid-dom-props": [
      "error",
      { "forbid": [{ "propName": "style", "message": "Inline style props are blocked by CSP. Use a CSS class or CSS module instead." }] }
    ],
    "react/forbid-component-props": [
      "error",
      { "forbid": [{ "propName": "style", "message": "Inline style props are blocked by CSP. Use a CSS class or CSS module instead." }] }
    ]
  }
}
```

`react/forbid-dom-props` covers intrinsic elements (`<div style={...}>`), while `react/forbid-component-props` covers custom components that may forward `style` to the DOM. Both are needed to close the gap.

## Historical Context

This constraint was discovered the hard way. Two commits fixed the fallout from inline styles that were silently dropped under CSP:

- **`5bd5e51`** — initial fix replacing inline `style` props with CSS classes.
- **`e80a15b`** — follow-up cleanup covering remaining components and edge cases.

These commits are the reference for the pattern described above. When in doubt, look at how those changes were structured.

## Summary

- **Never** use the `style` prop on DOM elements or components that forward it to the DOM.
- Use **CSS classes / CSS modules** and toggle `className` for dynamic styling.
- The failure mode is **silent** — it will not show up in dev if CSP is relaxed.
- Add `react/forbid-dom-props` and `react/forbid-component-props` to enforce this automatically.