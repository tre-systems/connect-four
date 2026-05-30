# Diagrams

Graphviz / DOT sources plus rendered PNGs. The `.dot` files are the source of truth; the PNGs are committed for in-browser viewing on GitHub. Smaller, simpler diagrams live inline as Mermaid in [ARCHITECTURE.md](../ARCHITECTURE.md); Graphviz is reserved for the larger, denser views below.

## Files

| Diagram                                | Source                | Rendered              |
| -------------------------------------- | --------------------- | --------------------- |
| System overview (architecture)         | `system-overview.dot` | `system-overview.png` |
| AI move — strategy dispatch & fallback | `ai-move-flow.dot`    | `ai-move-flow.png`    |

## Conventions

Color coding by domain (shared with the wider project house style):

- **Green** nodes / clusters — application code (React UI, Zustand stores, WASM facade, Rust AI engine).
- **Purple** — pure functions (`game-logic`, `logic/*`; no side effects).
- **Teal** — persistence (`localStorage`; D1, drawn dashed because it is scaffolded/not wired).
- **Blue** — client surface (browser / SPA).
- **Orange / gold** — build-time / codegen (wasm-pack, ts-rs, model assets).
- **Diamonds** — decisions. **Bold green outline** — terminal success. **Bold red outline** — error / terminal failure.
- **Dashed** edges — provenance (compile / codegen) or exceptional control flow.

Fonts: Avenir. Rendered at 220 DPI.

## Render

```
npm run diagrams          # render all .dot files to PNG next to the source
npm run check:diagrams    # verify each .dot renders cleanly and the PNG exists
```

Both assume Graphviz is on PATH (`brew install graphviz`). On a machine without `dot`, `check:diagrams` skips with a clear message; refresh PNGs with `npm run diagrams` before committing diagram changes.

To render one manually:

```
dot -Tpng:cairo docs/diagrams/<name>.dot -Gdpi=220 -o docs/diagrams/<name>.png
```
