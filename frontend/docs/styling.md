# Frontend Styling Conventions

## The rule: no inline `style` props

Do **not** use the `style` prop on React components or DOM elements for anything
that affects rendering:

```jsx
// ❌ Don't do this
<div style={{ marginTop: 8, color: 'red' }}>…</div>
```

Instead, express styling through CSS classes (or CSS modules) and apply them via
`className`:

```jsx
// ✅ Do this
<div className="stack-sm text-danger">…</div>
```

## Why: Content Security Policy strips inline styles

Our production Content Security Policy does not allow inline styles. When the
browser enforces that policy, inline `style` attributes are **silently dropped** —
the element still renders, but without the styling. This is not a build error and
not a test failure; it only shows up in production, which is exactly why it went
unnoticed for so long.

This was a real production bug. Two commits fixed it:

- `5bd5e51` — moved AppShell chrome off inline styles (CSP was dropping them)
- `e80a15b` — migrated every remaining inline `style` prop to CSS classes

Because the failure mode is silent and environment-specific, the constraint is
easy to reintroduce by accident. That is why it is documented here: so a future
PR does not ship the same bug again.

## The required pattern

- Put styling in CSS (global stylesheets or CSS modules) and reference it with
  `className`.
- For dynamic values that genuinely must vary at runtime, prefer toggling a
  class (e.g. `className={isActive ? 'is-active' : ''}`) or setting a CSS custom
  property through a class rather than writing an inline `style` object.
- If you believe you have a case that truly requires an inline style, raise it in
  the PR description first — do not add one silently.

## Enforcement

There is currently **no** lint rule or CI check that blocks inline `style` props,
so nothing mechanically prevents a regression. Adding one is recommended:

- ESLint's [`react/forbid-dom-props`](https://github.com/jsx-eslint/eslint-plugin-react/blob/master/docs/rules/forbid-dom-props.md)
  can be configured with `forbid: ['style']` to flag inline `style` props on DOM
  elements.
- For components that forward a `style` prop, `react/forbid-component-props`
  covers the same case.

Until such a rule is enabled, this convention is enforced by review. Please keep
it in mind when reviewing frontend changes.
