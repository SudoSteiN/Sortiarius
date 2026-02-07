# CRITERIA.md — Auto-generated from SPEC.md
# Last generated: 2026-02-07
# Re-generate if SPEC.md changes

## Build Criteria (Gate: Phase 1-2)
- [ ] Rust workspace compiles without errors (`cargo build --release`)
- [ ] WASM target builds successfully (`wasm-pack build`)
- [ ] React frontend compiles without TypeScript errors (`tsc --noEmit`)
- [ ] Vite dev server starts and loads WASM module
- [ ] All dependencies resolve (Cargo.toml + package.json)
- [ ] No lint errors (clippy for Rust, eslint for TypeScript)

## Functional Criteria — P0 (Gate: Phase 2-7, BLOCKING)

### Lumber & Modeling (Phase 2)
- [ ] US-001: Can select and place standard dimensional lumber (2x4, 2x6, 4x4, etc.)
- [ ] US-001: Nominal vs actual dimensions correct (2x4 renders as 1.5" x 3.5")
- [ ] US-001: Metric equivalents displayed in metric mode
- [ ] US-002: Can create custom-dimensioned boards (specify exact W x H x D)
- [ ] US-002: Custom dimensions work in both imperial and metric
- [ ] US-004: Can move boards by dragging
- [ ] US-004: Can rotate boards with axis constraints
- [ ] US-004: Snap-to-face alignment works
- [ ] US-004: Snap-to-edge alignment works
- [ ] US-004: Grid snapping toggleable

### Visualization (Phase 3)
- [ ] US-005: Can assign wood species to any board (minimum 12 species from spec)
- [ ] US-005: Wood grain pattern renders with approximate natural appearance
- [ ] US-005: Species color is visually distinct between different woods
- [ ] US-006: Can apply stain colors to boards
- [ ] US-006: Can apply paint colors to boards
- [ ] US-006: Finish preview shows effect over wood grain

### Joinery (Phase 4)
- [ ] US-003: Butt joint modifies both boards' geometry
- [ ] US-003: Miter joint modifies both boards' geometry
- [ ] US-003: Pocket hole modifies both boards' geometry
- [ ] US-003: Mortise & tenon modifies both boards' geometry
- [ ] US-003: Dado/rabbet modifies both boards' geometry
- [ ] US-003: Half-lap modifies both boards' geometry
- [ ] US-003: Dowel joint modifies both boards' geometry
- [ ] US-010: Each joint type shows strength rating for current species/dimensions
- [ ] US-010: Can compare joint types side-by-side

### Structural Analysis (Phase 5)
- [ ] US-011: Calculates beam deflection under load for shelves/spans
- [ ] US-011: Estimates compressive capacity for legs/posts
- [ ] US-011: Displays green/yellow/red status per component
- [ ] US-011: Uses published MoE and strength values per species

### Output Generation (Phase 6)
- [ ] US-007: Cut list includes all pieces with name, material, dimensions, quantity
- [ ] US-007: Cut list accounts for joinery geometry modifications
- [ ] US-007: Cut list exportable as PDF or printable
- [ ] US-008: Material list aggregates by species
- [ ] US-008: Material list calculates total board feet (and metric equivalents)
- [ ] US-008: Material list suggests standard lumber to purchase
- [ ] US-008: Waste factor applied
- [ ] US-009: Build instructions are ordered assembly steps
- [ ] US-009: Each step references specific pieces and joints
- [ ] US-009: Includes cut order and assembly order with reasoning

### Save/Load (Phase 7)
- [ ] US-012: Can save project to `.wfp` file
- [ ] US-012: Can load project from `.wfp` file
- [ ] US-012: Auto-save to browser localStorage works
- [ ] US-012: Round-trip: save → load → all data preserved (geometry, materials, joints, finishes)

## Functional Criteria — P1 (Non-blocking, should-have)
- [ ] US-013: Orbit camera (right-click drag or middle-click)
- [ ] US-013: Pan camera (shift+middle)
- [ ] US-013: Zoom camera (scroll wheel)
- [ ] US-014: Click two points/faces to see distance
- [ ] US-014: Persistent dimension annotations
- [ ] US-015: Undo (Ctrl+Z) works for all modeling operations
- [ ] US-015: Redo (Ctrl+Y) works for all modeling operations
- [ ] US-016: Bin-packing cut optimization against standard stock lengths
- [ ] US-017: User-editable price per board foot per species
- [ ] US-017: Total cost displayed
- [ ] US-018: Front/side/top orthographic shop drawing views
- [ ] US-018: PDF export of shop drawings

## Test Criteria (Gate: Phase 9)
- [ ] Rust unit tests: >= 70% coverage on core crate (geometry, joinery, structural)
- [ ] Rust unit tests: all joint types produce correct geometry
- [ ] Rust unit tests: lumber database returns correct actual dimensions
- [ ] Rust unit tests: unit conversion (imperial ↔ metric) is accurate
- [ ] Rust unit tests: structural analysis formulas produce correct deflection values
- [ ] Rust unit tests: cut list generation accounts for joinery geometry
- [ ] Vitest: UI component rendering (toolbar, properties panel, scene tree)
- [ ] Vitest: state management (Zustand store actions)
- [ ] Playwright E2E: create new project → add lumber → see in 3D viewport
- [ ] Playwright E2E: apply joinery → verify geometry changes in scene
- [ ] Playwright E2E: generate cut list → verify all pieces listed
- [ ] Playwright E2E: save project → reload → load project → verify state
- [ ] All tests pass (0 failures)
- [ ] Tests run in < 120 seconds (WASM compilation may be slower)

## UI Criteria (Gate: Phase 2-7)
- [ ] Landing page: "Start New Project" and "Open Existing" work
- [ ] Workspace: 3D viewport renders scene
- [ ] Workspace: Toolbar present with Select, Move, Rotate, Add Board, Add Joint, Measure
- [ ] Workspace: Scene tree displays component hierarchy
- [ ] Workspace: Properties panel shows dimensions, material, finish for selected component
- [ ] Workspace: Joinery palette shows joint types with visual previews
- [ ] Workspace: Bottom bar shows project info and units toggle (imperial/metric)
- [ ] Cut List View: table of all cuts, sorted by material
- [ ] Material List View: aggregated by species with board feet
- [ ] Instructions View: numbered steps with piece references
- [ ] Structural Analysis View: color-coded model with load inputs
- [ ] No console errors in browser during normal operation
- [ ] Units toggle switches all displayed measurements between imperial and metric

## Security Criteria (Gate: Phase 9)
- [ ] No secrets in code or build output
- [ ] WASM module doesn't expose internal Rust memory to JS (wasm-bindgen boundaries clean)
- [ ] File format parsing handles malformed `.wfp` files gracefully (no crashes)
- [ ] No XSS vectors in user-provided project names or labels
- [ ] localStorage data doesn't leak across origins (standard browser security)
- [ ] CSP headers configured for static hosting

## Performance Criteria (Gate: Phase 9)
- [ ] WASM module loads in < 3 seconds on broadband
- [ ] 3D viewport maintains 30+ FPS with 50 boards in scene
- [ ] Joinery boolean operation completes in < 500ms per joint
- [ ] Cut list generation completes in < 1 second for 100-piece project
- [ ] Structural analysis calculation completes in < 2 seconds
- [ ] Total bundle size (JS + WASM) < 10MB
- [ ] First meaningful paint < 5 seconds

## Deployment Criteria (Gate: Phase 9)
- [ ] Static build produces deployable dist/ folder
- [ ] Build works on CI (GitHub Actions or equivalent)
- [ ] PWA manifest and service worker for offline support
- [ ] README has setup instructions, build commands, and feature overview
- [ ] `.env.example` if any environment config needed (likely none for MVP)
- [ ] Deployed to Cloudflare Pages, GitHub Pages, or Vercel and accessible

## Exclusions (from SPEC.md Non-Goals)
Do NOT evaluate against:
- General-purpose CAD features (STEP/IGES import/export)
- CNC/CAM/G-code generation
- Social features or project sharing
- Mobile-optimized touch interfaces
- User accounts or authentication (MVP is local-first)
