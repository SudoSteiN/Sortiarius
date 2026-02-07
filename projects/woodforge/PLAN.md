# WoodForge — Plan

## Current Phase
Phase 3: Wood Species & Rendering

## Progress
Last updated: 2026-02-07

### Completed
- [x] Product spec finalized (SPEC.md) (2026-02-07)
- [x] Project plan created (PLAN.md) (2026-02-07)
- [x] Evaluation criteria defined (CRITERIA.md) (2026-02-07)
- [x] **Phase 1: Foundation scaffold** (2026-02-07)
  - Rust workspace: core, renderer, wasm-bridge crates
  - React + TypeScript + Vite + Tailwind frontend
  - wasm-pack build pipeline (WASM builds in ~13s, 492KB gzipped)
  - wgpu renderer with directional lighting shader
  - Camera orbit/pan/zoom controls (US-013)
- [x] **Phase 2: Lumber & Scene Graph** (2026-02-07)
  - Box mesh geometry generator (24 verts, 36 indices, face normals)
  - Standard lumber database: 17 sizes + 3 sheet goods + 4 standard lengths
  - Unit system: imperial (fractional inches) ↔ metric (mm) conversion
  - Scene graph with parent-child, world transform, mesh cache, serialization
  - Multi-object renderer with per-object model matrix uniforms
  - Snap system: grid, face, edge with configurable thresholds
  - WASM bridge: scene API (add/remove/move/rotate boards) + lumber API
  - Full React UI: toolbar, scene tree, properties panel, lumber picker, status bar
  - 24 unit tests passing across all core modules

### In Progress
- [ ] **CURRENT →** Phase 3: Wood Species & Rendering

### Up Next

#### Phase 3: Wood Species & Rendering
- [ ] Wood species database (12 species with mechanical properties)
- [ ] Pre-made wood grain textures for MVP species (US-005)
- [ ] Procedural grain generation fallback (Perlin noise)
- [ ] Assign species to boards, render with grain texture (US-005)
- [ ] Stain/paint finish preview over wood grain (US-006)
- [ ] Selection highlighting and transform gizmos

#### Phase 4: Joinery System
- [ ] Joinery type enum and data model (16 types from spec)
- [ ] Boolean operations on B-rep geometry (Truck subtract/intersect)
- [ ] Implement MVP joint types that modify geometry (US-003):
  - Butt joint
  - Miter joint
  - Pocket hole
  - Mortise & tenon
  - Dado / Rabbet
  - Half-lap
  - Dowel joint
- [ ] Joint strength rating per species/dimensions (US-010)
- [ ] Joinery palette UI with visual previews
- [ ] Joint comparison side-by-side view (US-010)

#### Phase 5: Structural Analysis
- [ ] Beam deflection calculator using MoE values (US-011)
- [ ] Compressive capacity estimation for legs/posts (US-011)
- [ ] Green/yellow/red load status visualization
- [ ] Structural analysis panel UI
- [ ] Joint strength lookup tables integrated with species data

#### Phase 6: Output Generation
- [ ] Cut list generator — all pieces with dimensions, accounts for joinery geometry (US-007)
- [ ] Material/shopping list aggregator — board feet + metric, waste factor (US-008)
- [ ] Build instruction generator — ordered steps with piece/joint references (US-009)
- [ ] PDF/printable export for cut list (US-007)
- [ ] Output panels UI (cut list, material list, instructions tabs)

#### Phase 7: Save/Load & Polish
- [ ] `.wfp` JSON file format — save/load projects (US-012)
- [ ] localStorage auto-save (US-012)
- [ ] Undo/redo operation history (US-015)
- [ ] Dimensioned measurements between parts (US-014)
- [ ] Landing page with "New Project" / "Open Project" (UI Screen 1)

#### Phase 8: Optimization & Export (P1)
- [ ] Cut list optimization — bin-packing against standard lengths (US-016)
- [ ] Material cost estimation with editable prices (US-017)
- [ ] 2D shop drawing export with dimensions — PDF (US-018)

#### Phase 9: Testing, Review & Ship
- [ ] Rust unit tests: geometry, joinery, structural analysis, cut list generation
- [ ] Vitest: UI components, state management
- [ ] Playwright E2E: full workflow (create project → add boards → apply joinery → generate cut list)
- [ ] Code review pass: security, performance, quality
- [ ] PWA setup (offline capable after initial load)
- [ ] Deployment: static build → Cloudflare Pages or GitHub Pages
- [ ] README with setup instructions, screenshots, feature overview

### Blocked / Needs Input
_(none)_

## Architecture Decisions
| Decision | Choice | Date | Rationale |
|----------|--------|------|-----------|
| CAD kernel | Truck (Rust b-rep) | 2026-02-07 | Proven Rust CAD kernel, B-rep with Boolean ops, used by CADmium |
| 3D renderer | wgpu (Rust → WASM) | 2026-02-07 | WebGPU primary + WebGL2 fallback, no JS in render loop |
| UI framework | React + TypeScript | 2026-02-07 | Mature ecosystem for complex panels/toolbars, wasm-bindgen bridge |
| Joinery approach | Geometry-modifying (boolean ops) | 2026-02-07 | Accurate cut lists, true 3D representation, not just visual |
| Texture approach | Hybrid pre-made + procedural | 2026-02-07 | Pre-made for 12 MVP species, Perlin noise fallback for variety |
| Unit system | Imperial + metric from day one | 2026-02-07 | Conversion layer in Rust, user-selectable display |
| File format | `.wfp` JSON | 2026-02-07 | Human-readable, self-contained, easy to add cloud save later |
| Hosting | Static site (no backend for MVP) | 2026-02-07 | Free, private, offline-capable, no infrastructure to maintain |

## Session Log
| Date | What was accomplished | Files changed |
|------|----------------------|---------------|
| 2026-02-07 | Product spec finalized (SPEC.md), project plan created (PLAN.md), evaluation criteria defined (CRITERIA.md) | SPEC.md, PLAN.md, CRITERIA.md, CLAUDE.md, .gitignore |
