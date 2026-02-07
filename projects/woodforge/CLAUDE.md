# Project: WoodForge

## Project Context
Free, browser-based 3D woodworking planning tool. Rust/WASM core with React UI.
Sortiarius global identity, skills, and rules apply via ~/.claude/CLAUDE.md.

## Tech Stack
- **CAD Kernel:** Truck (Rust b-rep library) → WASM
- **Rendering:** wgpu (WebGPU + WebGL2 fallback) → WASM
- **Domain Logic:** Custom Rust modules (joinery, lumber DB, cut lists, structural analysis) → WASM
- **UI:** React + TypeScript + Tailwind CSS
- **State:** Zustand (JS) + component storage (Rust)
- **Build:** wasm-pack + Vite
- **Testing:** cargo test (Rust), Vitest (JS), Playwright (E2E)
- **Hosting:** Static site (all computation in WASM, no backend)

## Repository
- **GitHub:** https://github.com/SudoSteiN/WoodForge
- **Local clone:** `~/WoodForge`
- Source code lives in the WoodForge repo, not here.

## Planning Artifacts (this directory)
- SPEC.md — Product specification
- PLAN.md — Progress tracker
- CRITERIA.md — Evaluation criteria
- PROJECT.md — Quick reference

## Project Rules
- Local-first architecture: all computation in browser WASM
- Joinery cuts actual 3D geometry (not just visual indicators)
- Imperial + metric from day one
- Wood textures: pre-made for common species, procedural generation for variety
- No user accounts in MVP — save/load via file export
