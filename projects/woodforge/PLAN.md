# WoodForge — Plan

## Current Phase
Phase 9: COMPLETE

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
- [x] **Phase 3: Wood Species & Rendering** (2026-02-07)
  - 12 wood species database with mechanical properties (density, MOE, MOR, Janka hardness)
  - Procedural grain generation via Perlin noise (deterministic, per-species seeded)
  - Finish system: stain (blending), paint (opaque), oil (darkening), natural
  - Per-board species and finish assignment with WASM API
  - Renderer texture pipeline (per-object wgpu textures with bind groups)
  - Species picker panel UI with color swatches and property display
- [x] **Phase 4: Joinery System** (2026-02-07)
  - 16 joint types with difficulty ratings and strength values
  - JointStore with CRUD operations and board-based queries
  - Geometry generators: butt, miter, dado, rabbet, half-lap, mortise & tenon
  - Joint face merging with proper normals and UVs
  - Joinery panel UI with board/face selectors and joint list
  - WASM bridge: add/remove joints, get joint types
- [x] **Phase 5: Structural Analysis** (2026-02-07)
  - Beam deflection calculator (δ = 5wL⁴/384EI) with span ratio checks
  - Compressive capacity estimation with Euler buckling detection
  - Joint strength calculation with species hardness/density factors
  - Green/Yellow/Red status thresholds
  - Analysis panel UI with shelf/compression/joint tabs
- [x] **Phase 6: Output Generation** (2026-02-07)
  - Cut list generator with board grouping and joint annotations
  - Material list aggregator with board feet calculations and waste factor
  - Build instruction generator with ordered steps and tool requirements
  - Output panel UI with cut list, materials, cost, and instructions tabs
- [x] **Phase 7: Save/Load & Polish** (2026-02-07)
  - `.wfp` JSON project file format (save/load via file download/upload)
  - Command-pattern undo/redo with inverse operations (50-step history)
  - Project panel UI with save/load/new/undo/redo
  - Toolbar: undo/redo buttons + right panel tab switching
  - Viewport: click-to-select with 3px drag threshold
- [x] **Phase 8: Optimization & Export** (2026-02-07)
  - Cut list optimization via First Fit Decreasing bin-packing
  - Material cost estimation with editable per-species pricing
  - Cost breakdown: materials, hardware, waste factor, total
  - Price database with default $/BF values and user overrides

- [x] **Phase 9: Testing, Review & Ship** (2026-02-07)
  - 110 Rust tests (103 unit + 7 integration) across all core modules
  - 53 Vitest frontend tests (store + type tests)
  - PWA: service worker with workbox precaching (9 entries, 1966 KiB)
  - SVG favicon, manifest.json, robots.txt, PWA meta tags
  - Production build: tsc + vite + wasm-pack (492KB WASM gzipped)

### All Phases Complete

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
| 2026-02-07 | Phase 1+2: Foundation + Lumber & Scene Graph complete | crates/core/*, crates/renderer/*, crates/wasm-bridge/*, frontend/src/* |
| 2026-02-07 | Phases 3-8: Species, Joinery, Structural, Output, Save/Load, Optimization all complete. 103 Rust tests passing. WASM 492KB gzipped. Full frontend with 6 panel tabs. | crates/core/src/{species,joinery,joinery_geometry,structural,output,optimizer,cost,project,history,grain,finish}.rs, crates/wasm-bridge/src/{species,joinery,analysis,project}_api.rs, frontend/src/components/{SpeciesPanel,JoineryPanel,AnalysisPanel,OutputPanel,ProjectPanel}.tsx |
| 2026-02-07 | Phase 9: Testing complete. 110 Rust tests (7 integration), 53 Vitest tests, PWA with service worker, favicon, manifest. All builds green. | crates/core/src/lib.rs (integration tests), frontend/src/__tests__/{store,types}.test.ts, frontend/vitest.config.ts, frontend/vite.config.ts, frontend/index.html, frontend/public/{manifest.json,robots.txt,favicon.svg} |
