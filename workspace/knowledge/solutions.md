# Sortiarius Knowledge Library — Solutions

> Specific technical solutions from completed projects.
> Each entry includes: what, why, how, and where to find the code.

## Template for New Entries

```markdown
## [Solution Title]
<!-- learned: YYYY-MM-DD -->
<!-- project: project-name -->
<!-- tags: auth, api -->
<!-- reuse: HIGH | MEDIUM | LOW -->

**Problem:** What problem does this solve?
**Solution:** How we solved it.
**Key files:** Where the implementation lives.
**What worked:** The successful approach.
**What didn't:** Approaches we tried that failed (and why).
**Reuse notes:** How to adapt this for a different project.
```

---

## Rust/WASM + wgpu + React Scaffold
<!-- learned: 2026-02-07 -->
<!-- project: woodforge -->
<!-- tags: rust, wasm, wgpu, react, vite -->
<!-- reuse: HIGH -->

**Problem:** Setting up a Rust/WASM project with wgpu 3D rendering and a React frontend.
**Solution:** 3-crate workspace (core domain logic, renderer, wasm-bridge) + Vite React frontend. wasm-bridge is the only cdylib; core and renderer are rlib for native testing. Surface creation lives in wasm-bridge (WASM-only API), renderer takes Instance+Surface as args.
**Key files:** `~/Sortiarius/projects/woodforge/` — full working scaffold.
**What worked:**
- Keeping renderer platform-agnostic by accepting `wgpu::Instance` + `wgpu::Surface` instead of `HtmlCanvasElement`
- Thread-local `RefCell<AppState>` in wasm-bridge for single-threaded WASM state
- Clone mesh data before passing to renderer to avoid borrow conflicts
- Vite path alias + `.d.ts` for WASM module imports
**What didn't:**
- `SurfaceTarget::Canvas` in renderer crate — blocked native `cargo check`. Moved to wasm-bridge.
- Uuid without `serde` feature — forgot it, caught by compiler.
**Reuse notes:** Copy the Cargo.toml structure and wasm-bridge pattern. Swap domain logic in core crate. Renderer pattern (camera + scene_renderer + per-object uniforms) is generic.
