# WoodForge — Product Spec

> Free, browser-based 3D woodworking planning tool with joinery intelligence, structural analysis, and automated cut/material lists.

## Problem Statement

Woodworkers — from first-time hobbyists to professionals — need to plan projects in 3D before cutting wood. The current options are all broken: SketchUp Free stripped its woodworking plugins and costs $299/year for Pro. FreeCAD has a woodworking workbench but the learning curve is brutal and the UI is hostile. CraftyAmigo is browser-based but too limited for anything beyond a shelf. No free tool treats woodworking as a first-class domain — joinery, grain direction, standard lumber dimensions, cut list optimization, and structural validation are afterthoughts everywhere. The result: most hobbyists either sketch on paper (inaccurate), overspend on materials (no cut optimization), or discover structural problems after the glue dries.

WoodForge solves this by being the first free, browser-based CAD tool built specifically for woodworking — where joinery methods, wood species, and material lists are native features, not plugins.

## Goals

- **Primary:** 3D parametric modeling with woodworking-native operations (joinery, lumber dimensions, grain direction) that produces precise cut lists, material lists, and build instructions.
- **Secondary:** Structural analysis (will this shelf sag?), wood species visualization (grain/color rendering), stain/paint preview, cost estimation, shop drawing export.

## Non-Goals

- Not a general-purpose CAD tool (no STEP/IGES industrial workflows)
- Not a CNC/CAM tool (no G-code generation — maybe later)
- Not a marketplace or social platform (no project sharing/selling in MVP)
- Not a mobile-first app (responsive but optimized for desktop/tablet with mouse)
- No user accounts in MVP (local-first, save to browser/file export)

## User Personas

| Persona | Description | Primary Need |
|---------|-------------|-------------|
| **Weekend Warrior** | First-time hobbyist, wants to build a bookshelf or workbench. Little/no CAD experience. | Easy 3D modeling with standard lumber, simple joinery, "tell me what to buy and how to cut it" |
| **Experienced Maker** | Builds furniture regularly, knows joinery, wants precision. Has used SketchUp or FreeCAD before. | Parametric modeling, accurate cut lists, joinery visualization, structural confidence |
| **Professional Builder** | Builds for clients, needs detailed plans and material costing. May produce multiple variations. | Full customization, cost estimation, shop drawings, professional-quality output |

## User Stories

### P0 — Must Have (MVP)

- [ ] **US-001:** As a Weekend Warrior, I want to place standard dimensional lumber (2x4, 2x6, 4x4, etc.) in a 3D scene so that I can visualize my project.
  - AC: Can select from standard North American lumber dimensions. Pieces render at actual dimensions (nominal vs actual handled correctly: a "2x4" renders as 1.5" x 3.5"). Metric equivalents displayed when metric mode is active.

- [ ] **US-002:** As a Weekend Warrior, I want to create custom-dimensioned boards so that I can model plywood sheets, ripped lumber, or specialty cuts.
  - AC: Can specify exact width, height, depth in imperial or metric. Can start from standard lumber or from scratch.

- [ ] **US-003:** As an Experienced Maker, I want to apply joinery between two boards so that I can plan how pieces connect.
  - AC: Supports at minimum: butt joint, miter joint, pocket hole, mortise & tenon, dado/rabbet, half-lap, dowel joint. Selecting a joint type **modifies both boards' actual 3D geometry** (boolean operations cut the shapes). Joint geometry is reflected in cut lists.

- [ ] **US-004:** As a Weekend Warrior, I want to move, rotate, and snap boards together in 3D so that I can build assemblies intuitively.
  - AC: Drag to move, axis-constrained rotation. Snap-to-face and snap-to-edge for alignment. Grid snapping optional.

- [ ] **US-005:** As an Experienced Maker, I want to visualize different wood species on my boards so that I can see how the finished product will look.
  - AC: Can assign a wood species (oak, walnut, pine, maple, cherry, cedar, at minimum) to any board. Renders approximate grain pattern and natural color. Uses pre-made textures for common species with procedural generation for variety and uncommon species.

- [ ] **US-006:** As a Weekend Warrior, I want to preview stain or paint colors on my boards so that I can make finish decisions before building.
  - AC: Can apply named stain colors or custom paint colors. Preview shows approximate effect over the wood grain.

- [ ] **US-007:** As a Professional Builder, I want to generate a precise cut list from my project so that I know exactly what cuts to make.
  - AC: Lists every piece with: name/label, material, length, width, thickness, quantity, notes. Accounts for joinery geometry modifications. Exportable as PDF or printable.

- [ ] **US-008:** As a Weekend Warrior, I want to generate a material/shopping list so that I know what to buy at the lumber yard.
  - AC: Aggregates all pieces by material/species, calculates total board feet (and metric equivalents), lists standard lumber needed. Accounts for reasonable waste factor.

- [ ] **US-009:** As a Weekend Warrior, I want detailed build instructions generated from my project so that I can follow step-by-step assembly.
  - AC: Ordered assembly steps. Each step references specific pieces and joints. Includes which pieces to cut first, which to assemble first, and why.

- [ ] **US-010:** As an Experienced Maker, I want to evaluate which joinery method is most structurally sound for a given connection so that I can make informed design decisions.
  - AC: Each joint type displays a strength rating for the current wood species and board dimensions. Can compare side-by-side.

- [ ] **US-011:** As a Professional Builder, I want to estimate the structural load capacity of my design so that I know if it will hold the intended weight.
  - AC: For shelves/spans: calculates deflection under load. For legs/posts: estimates compressive capacity. Displays green/yellow/red status. Uses published modulus of elasticity and strength values per species.

- [ ] **US-012:** As any user, I want to save and load my projects so that I can work on them across sessions.
  - AC: Save to local file (`.wfp` JSON format). Load from file. Auto-save to browser localStorage as backup.

### P1 — Should Have

- [ ] **US-013:** As an Experienced Maker, I want to orbit, pan, and zoom the 3D view with mouse/trackpad so that I can inspect my design from all angles.
  - AC: Orbit (right-click drag or middle-click), pan (shift+middle), zoom (scroll wheel). Smooth, responsive.

- [ ] **US-014:** As a Professional Builder, I want to see dimensioned measurements between parts so that I can verify precise spacing.
  - AC: Click two points/faces to see distance. Persistent dimension annotations. Supports imperial and metric display.

- [ ] **US-015:** As any user, I want undo/redo for all modeling operations.
  - AC: Ctrl+Z / Ctrl+Y. Full operation history.

- [ ] **US-016:** As a Professional Builder, I want to optimize my cut list against standard lumber lengths so that I minimize waste.
  - AC: Bin-packing algorithm that shows how to cut boards from standard stock lengths (8', 10', 12') with minimum waste.

- [ ] **US-017:** As a Professional Builder, I want to estimate material costs based on current wood species prices so that I can quote projects.
  - AC: User-editable price per board foot per species. Total cost displayed.

- [ ] **US-018:** As any user, I want to export my project as a 2D shop drawing with dimensions so that I can print it for the workshop.
  - AC: Front/side/top orthographic views with dimensions. PDF export.

### P2 — Nice to Have

- [ ] **US-019:** As an Experienced Maker, I want to see exploded views of my assembly so that I can understand how pieces fit together.
- [ ] **US-020:** As any user, I want project templates (workbench, bookshelf, table) so that I can start from a proven design and customize.
- [ ] **US-021:** As a Professional Builder, I want to create and reuse custom component libraries (e.g., "my standard drawer box") across projects.
- [ ] **US-022:** As any user, I want a bill of materials that includes hardware (screws, glue, brackets) based on my joinery choices.

## Data Model

| Entity | Key Fields | Relationships |
|--------|-----------|---------------|
| **Project** | id, name, description, created_at, updated_at, units (imperial/metric) | has many Components, has many Joints |
| **Component** | id, label, width, height, depth, position, rotation, material_id, grain_direction | belongs to Project, has one Material, has many Joints |
| **Material** | id, species_name, density, modulus_of_elasticity, compressive_strength, hardness_janka, color_hex, grain_texture_id, cost_per_bf | referenced by Components |
| **Joint** | id, type (enum), component_a_id, component_b_id, position, strength_rating, geometry_modifications[] | connects two Components |
| **Finish** | id, type (stain/paint), color, opacity | applied to Components |
| **CutListEntry** | component_id, label, material, length, width, thickness, quantity, joint_notes | generated from Project |
| **Instruction** | step_number, description, components_involved[], joint_type, tools_needed | generated from Project |

### Joint Types (Enum)

```
ButtJoint, MiterJoint, PocketHole, MortiseAndTenon,
Dado, Rabbet, HalfLap, CrossLap, DovetailThrough,
DovetailHalfBlind, BoxJoint, FingerJoint, DowelJoint,
BiscuitJoint, TongueAndGroove, BridleJoint
```

### Standard Lumber Database

```
North American (Imperial — nominal → actual):
2x2 (1.5" x 1.5"), 2x3 (1.5" x 2.5"), 2x4 (1.5" x 3.5"),
2x6 (1.5" x 5.5"), 2x8 (1.5" x 7.25"), 2x10 (1.5" x 9.25"),
2x12 (1.5" x 11.25"), 4x4 (3.5" x 3.5"), 4x6 (3.5" x 5.5"),
6x6 (5.5" x 5.5"), 1x2, 1x3, 1x4, 1x6, 1x8, 1x10, 1x12
Standard lengths: 8', 10', 12', 16'
Sheet goods: 4'x8' plywood (1/4", 1/2", 3/4")

Metric equivalents stored for all dimensions.
Custom dimensions supported in both unit systems.
```

### Wood Species Database (MVP set)

| Species | Density (lb/ft³) | MoE (psi) | Janka | Color |
|---------|-----------------|-----------|-------|-------|
| Pine (Southern Yellow) | 35 | 1,700,000 | 870 | Light yellow |
| Douglas Fir | 32 | 1,900,000 | 710 | Reddish brown |
| Red Oak | 44 | 1,820,000 | 1,290 | Pink-brown |
| White Oak | 47 | 1,780,000 | 1,360 | Light brown |
| Hard Maple | 44 | 1,830,000 | 1,450 | Cream-white |
| Black Walnut | 38 | 1,680,000 | 1,010 | Dark brown |
| Cherry | 35 | 1,490,000 | 995 | Warm red-brown |
| Western Red Cedar | 23 | 1,110,000 | 350 | Red-brown |
| Poplar | 29 | 1,580,000 | 540 | Green-cream |
| Birch | 43 | 2,010,000 | 1,260 | Light cream |
| Ash | 41 | 1,740,000 | 1,320 | Light tan |
| Mahogany | 31 | 1,400,000 | 800 | Deep red-brown |

## Tech Stack

| Layer | Choice | Rationale |
|-------|--------|-----------|
| **CAD Kernel** | Truck (Rust b-rep library) | Proven Rust CAD kernel with B-rep, NURBS, Boolean ops. Used by CADmium before it was archived. |
| **3D Rendering** | wgpu (Rust → WASM) | WebGPU primary + WebGL2 fallback. Direct GPU access from Rust. No JS overhead in the render loop. |
| **Domain Logic** | Custom Rust modules | Joinery system, lumber database, cut list generator, structural analysis — all in Rust compiled to WASM. |
| **UI Framework** | React + TypeScript | Mature ecosystem for complex UI (toolbars, panels, property editors). Communicates with WASM core via wasm-bindgen. |
| **Styling** | Tailwind CSS | Rapid UI development, dark/light theme support. |
| **State Management** | Zustand (JS side) + ECS-like (Rust side) | Lightweight JS state for UI. Rust-side component storage for scene graph. |
| **File Format** | Custom JSON (`.wfp` - WoodForge Project) | Human-readable, versionable, easy to parse. Binary option later for large projects. |
| **Build** | wasm-pack + Vite | WASM compilation + fast HMR for the React UI layer. |
| **Testing** | Rust: cargo test. JS: Vitest. E2E: Playwright | Native Rust tests for geometry/analysis. Vitest for UI. Playwright for integration. |
| **Hosting** | Static site (GitHub Pages, Cloudflare Pages, or Vercel) | All computation in WASM — no backend needed for MVP. Free hosting. |

### Architecture

```
Browser
├── React UI (TypeScript)
│   ├── Toolbar / Menus
│   ├── Properties Panel (dimensions, material, finish)
│   ├── Scene Tree (component hierarchy)
│   ├── Cut List / Material List / Instructions panels
│   └── Viewport (canvas element → wgpu)
│
└── WASM Core (Rust)
    ├── CAD Engine (Truck kernel)
    │   ├── Parametric modeling (sketch → extrude → cut)
    │   ├── Boolean operations (union, subtract, intersect)
    │   └── Snapping & constraints
    ├── Woodworking Domain
    │   ├── Joinery system (geometry-modifying primitives for each joint type)
    │   ├── Lumber database (standard dimensions, species, imperial + metric)
    │   ├── Cut list generator (project → cut list, accounts for joinery geometry)
    │   ├── Material list generator (aggregate + optimize)
    │   ├── Instruction generator (assembly order + steps)
    │   └── Finish system (stain, paint, natural)
    ├── Structural Analysis
    │   ├── Beam deflection calculator (δ = 5wL⁴/384EI)
    │   ├── Joint strength lookup tables
    │   └── Load capacity estimator (green/yellow/red)
    ├── Unit System
    │   ├── Imperial ↔ metric conversion layer
    │   └── Display formatting (fractions vs decimals)
    └── Renderer (wgpu)
        ├── Mesh generation from B-rep
        ├── Wood grain texture rendering (pre-made + procedural hybrid)
        ├── Finish/stain preview
        └── Selection highlighting + gizmos
```

### Why Local-First (No Backend for MVP)

- **Free hosting**: Static files only = GitHub Pages, Cloudflare Pages, or Vercel free tier
- **Privacy**: Users' designs never leave their browser
- **Offline capable**: Works without internet after initial load (PWA)
- **Performance**: No network round-trips for modeling operations
- **Simplicity**: No auth, no database, no server infrastructure to maintain

Save/load via file download/upload (`.wfp` format). localStorage for auto-save.

### Future Cloud Save Strategy

When cloud save is added post-MVP:
- **Target**: Cheapest option that's secure and performant
- **Candidates**: Cloudflare R2 (no egress fees) + Workers for auth, or Supabase free tier
- **Data model consideration**: `.wfp` files are self-contained JSON — cloud save is just file storage with optional user auth, no schema changes needed
- **Design now for portability**: Keep the file format self-contained so any storage backend works

## UI Screens

### 1. Landing Page
- Purpose: Intro, "Start New Project" or "Open Existing"
- Key elements: Hero section, feature highlights, "Try Now" button
- Navigation: → Workspace

### 2. Workspace (Main Editor)
- Purpose: The 3D modeling environment — where all design happens
- Key elements:
  - **Viewport** (center): 3D scene with orbit/pan/zoom
  - **Toolbar** (top): Select, Move, Rotate, Add Board, Add Joint, Measure
  - **Scene Tree** (left panel): Hierarchical list of all components
  - **Properties Panel** (right panel): Dimensions, material, finish for selected component
  - **Joinery Palette** (popup or panel): Joint type selection with visual previews
  - **Bottom bar**: Project info, units toggle (imperial/metric), status messages
- Navigation: Contains all sub-panels

### 3. Cut List View
- Purpose: Display generated cut list with export options
- Key elements: Table of all cuts, sorted by material. Print/PDF export button.
- Navigation: Tab within workspace or overlay panel

### 4. Material List View
- Purpose: Shopping list aggregated by material type
- Key elements: Species, board feet needed, suggested stock lumber, optional cost estimate
- Navigation: Tab within workspace

### 5. Instructions View
- Purpose: Step-by-step assembly guide
- Key elements: Numbered steps with descriptions, referenced pieces highlighted in 3D view, tools needed per step
- Navigation: Tab within workspace

### 6. Structural Analysis View
- Purpose: Load capacity and deflection display
- Key elements: Color-coded model (green/yellow/red per component), load input controls, deflection values
- Navigation: Toggle or panel within workspace

## Resolved Decisions

1. **Name**: WoodForge (confirmed)
2. **Unit system**: Imperial and metric from day one. Imperial is default for North American lumber database. Full metric support with conversion layer. All UI measurements display in user's selected unit.
3. **Joinery geometry**: Joinery operations **actually cut the 3D geometry** using boolean operations on the B-rep model. This is computationally more expensive but ensures cut lists are accurate and the 3D view shows exactly what needs to be built.
4. **Texture approach**: Hybrid — pre-made textures for the 12 MVP wood species (realistic appearance), with procedural generation (Perlin noise based) as a fallback for variety, custom species, and edge cases.
5. **Hosting/Cloud save**: MVP is purely static (GitHub Pages, Cloudflare Pages, or Vercel free tier). Future cloud save targets the cheapest secure option — likely Cloudflare R2 + Workers or Supabase free tier. The `.wfp` file format is designed to be self-contained so any backend works.

---

*Generated by Sortiarius product-spec skill — 2026-02-07*
*Status: APPROVED*
