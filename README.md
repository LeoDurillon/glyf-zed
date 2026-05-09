# Glyf for Zed

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

Emmet-inspired HTML and JSX abbreviation expansion for [Zed](https://zed.dev).
Write compact abbreviations and get full HTML or JSX structures as completions.

```
ul>li.item*3  →  <ul>
                   <li class="item"></li>
                   <li class="item"></li>
                   <li class="item"></li>
                 </ul>
```

Powered by [`glyf-lsp`](https://github.com/LeoDurillon/glyf-lsp) — the binary
is downloaded automatically on first use.

---

## Installation

> **Not yet published to the Zed marketplace** — install as a dev extension
> while the submission is pending.

### Dev extension (current)

1. Make sure Rust is installed via [rustup](https://rustup.rs) — Zed requires
   it to build extensions locally.
2. Clone this repository:
   ```sh
   git clone https://github.com/LeoDurillon/zed-glyf
   ```
3. In Zed open the extensions panel (`zed: extensions`) and click
   **Install Dev Extension**.
4. Select the cloned directory.

On first use the extension automatically downloads the `glyf-lsp` binary for
your platform — no manual setup needed.

### Marketplace (coming soon)

Once the submission is approved you will be able to install directly:

```
zed: extensions  →  search "glyf"
```

---

## Usage

Completions are triggered automatically while typing on any of these characters:

`.`  `:`  `>`  `+`  `*`  `(`

Type an abbreviation, select the **Glyf** completion, and the full HTML/JSX is
inserted with tab stops so you can jump through empty fields.

### Syntax reference

| Abbreviation | Output |
|---|---|
| `div` | `<div></div>` |
| `div.foo` | `<div class="foo"></div>` |
| `div#app` | `<div id="app"></div>` |
| `a:href=url` | `<a href="url"></a>` |
| `h1<My title` | `<h1>My title</h1>` |
| `ul>li*3` | `<ul>` with three `<li>` children |
| `div+p` | `<div>` followed by `<p>` |
| `(li)*3` | three `<li>` elements |
| `.foo` | `<div class="foo"></div>` (implicit div) |
| `e>p` | `<>` with `<p>` inside (JSX fragment) |
| `br` | `<br />` (built-in self-closing snippet) |

Full syntax reference: [glyf-core docs](https://docs.rs/glyf-core).

---

## Custom snippets

Define your own aliases in Zed's `settings.json`:

```json
{
  "lsp": {
    "glyf": {
      "initialization_options": {
        "snippets": {
          "mc": "MyComponent",
          "card": "div.card>p.card-header+p.card-body",
          "btn": "MyButton"
        }
      }
    }
  }
}
```

Custom snippets **shadow** built-ins when they share the same key, and support
the full glyf syntax including multi-element expansions.

---

## Supported file types

HTML · JavaScript · TypeScript · TSX

---

## License

MIT — see [LICENSE](LICENSE).
