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

---

## Parallel Agent Pattern for Full-Stack Builds
<!-- learned: 2026-02-07 -->
<!-- project: woodforge -->
<!-- tags: agents, parallel, build-pattern -->
<!-- reuse: HIGH -->

**Problem:** Building a full app (9 phases) efficiently without running out of context.
**Solution:** Use builder agents in parallel for independent work, sequentially for dependent work. Pattern: build Rust core first (tests run natively without WASM), then WASM bridge, then frontend components in parallel.
**What worked:**
- Launching 2-3 parallel builder agents for independent frontend panels (SpeciesPanel, JoineryPanel, OutputPanel in one; ProjectPanel, Toolbar, Viewport in another)
- Building all Rust domain modules first, verifying with `cargo test`, then WASM bridge, then frontend
- `cfg(target_arch = "wasm32")` gating for WASM-only APIs so `cargo check/test` works natively
- Running `cargo test` + `tsc --noEmit` + `npm run build` as the verification trifecta
**What didn't:**
- Agents sometimes created code with missing fields (e.g. `MeshData` without `uvs`). Always grep for struct constructors after adding fields.
- Agents creating ProjectPanel used wrong return types for `loadProject`/`newProject` — fixed by type-checking after agent completion
**Reuse notes:** For any project with 5+ independent components, spin up parallel builder agents. Always verify agent output with compiler checks before moving on.

---

## Command Pattern Undo/Redo in Rust
<!-- learned: 2026-02-07 -->
<!-- project: woodforge -->
<!-- tags: rust, undo-redo, design-pattern -->
<!-- reuse: MEDIUM -->

**Problem:** Implementing undo/redo for a CAD application with diverse operations.
**Solution:** `Operation` enum with variants for each action type. `History` struct holds undo/redo stacks. Each operation has an `inverse()` method that returns the reverse operation. Recording an op pushes it to undo stack and clears redo stack.
**Key files:** `~/Sortiarius/projects/woodforge/crates/core/src/history.rs`
**Reuse notes:** The pattern is generic — change the `Operation` variants to match your domain actions.

---

## Bin-Packing Cut Optimizer
<!-- learned: 2026-02-07 -->
<!-- project: woodforge -->
<!-- tags: rust, algorithm, optimization -->
<!-- reuse: MEDIUM -->

**Problem:** Minimizing waste when cutting boards from standard stock lengths.
**Solution:** First Fit Decreasing (FFD) algorithm — sort pieces longest-first, try to fit each into existing stock boards, open new stock if none fits. Account for kerf width between cuts.
**Key files:** `~/Sortiarius/projects/woodforge/crates/core/src/optimizer.rs`
**Reuse notes:** Generic bin-packing — applicable to any 1D cutting stock problem. Change `StockBoard` and `CutPiece` structs for different domains.

---

## PWA with Vite + WASM
<!-- learned: 2026-02-07 -->
<!-- project: woodforge -->
<!-- tags: pwa, vite, wasm, service-worker -->
<!-- reuse: HIGH -->

**Problem:** Making a WASM-heavy app work offline as a PWA.
**Solution:** `vite-plugin-pwa` with `maximumFileSizeToCacheInBytes: 5 * 1024 * 1024` to accommodate large WASM bundles. Workbox `globPatterns: ['**/*.{js,css,html,wasm}']` ensures WASM is precached.
**Key files:** `~/Sortiarius/projects/woodforge/frontend/vite.config.ts`
**Reuse notes:** Any Vite + WASM project needs the increased file size limit. Without it, workbox silently skips the WASM file.
