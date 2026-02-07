---
name: learning-mode
description: Explain the WHY behind changes, generate visual explanations, architecture diagrams, and HTML presentations
triggers:
  - explain this
  - teach me
  - why does this work
  - learning mode
  - draw diagram
  - presentation
  - architecture diagram
  - visualize
pipeline: []
---

# Learning Mode Skill

## When to Use
- When Justin asks "why" not just "what"
- When explaining architecture or complex systems
- When onboarding to a new codebase
- Boris tip #10: "Enable 'Explanatory' style. Have Claude generate visual HTML presentations, draw ASCII diagrams of codebases."

## Process

### Explanatory Mode
When activated, every code change gets a brief explanation:
```
// Changed: HashMap → BTreeMap
// Why: We iterate in key order for the cut list.
// BTreeMap maintains sorted keys, HashMap doesn't.
// Tradeoff: O(log n) insert vs O(1), but n < 1000 so negligible.
```

### ASCII Architecture Diagrams
For system overviews, draw ASCII diagrams:

```
┌─────────────────────────────────────────┐
│                Browser                   │
│  ┌──────────┐  ┌──────────────────────┐ │
│  │ React UI │──│ WASM Bridge (bindgen)│ │
│  │ Toolbar  │  │                      │ │
│  │ Panels   │  │  ┌────────────────┐  │ │
│  │ Tree     │  │  │ Rust Core      │  │ │
│  └──────────┘  │  │  ├─ CAD Engine │  │ │
│                │  │  ├─ Domain     │  │ │
│                │  │  ├─ Analysis   │  │ │
│                │  │  └─ Renderer   │  │ │
│                │  └────────────────┘  │ │
│                └──────────────────────┘ │
└─────────────────────────────────────────┘
```

### Data Flow Diagrams
Show how data moves through the system:
```
User Action → React Event → wasm-bindgen → Rust Handler
    ↓              ↓             ↓              ↓
  Click      onChange()    bridge_fn()    process()
                                              ↓
                                         Update State
                                              ↓
                                         Re-render
                                              ↓
                                    wgpu Draw Call → Canvas
```

### Self-Contained HTML Presentations
For complex explanations, generate a single HTML file with:
- No external dependencies (inline CSS/JS)
- Slide-based navigation (arrow keys)
- Syntax-highlighted code blocks
- SVG diagrams where ASCII isn't enough
- Save to `workspace/scratch/presentation-<topic>.html`

Template:
```html
<!DOCTYPE html>
<html><head>
<style>
  body { font-family: system-ui; margin: 0; background: #1a1a2e; color: #eee; }
  .slide { display: none; padding: 2em 4em; min-height: 100vh; }
  .slide.active { display: flex; flex-direction: column; justify-content: center; }
  h1 { color: #e94560; } h2 { color: #0f3460; }
  pre { background: #16213e; padding: 1em; border-radius: 8px; overflow-x: auto; }
  code { color: #a8e6cf; }
  .nav { position: fixed; bottom: 1em; right: 1em; color: #666; }
</style>
</head><body>
<div class="slide active" id="s1"><h1>Title</h1><p>Content</p></div>
<div class="slide" id="s2"><h2>Slide 2</h2><p>Content</p></div>
<div class="nav">← → to navigate</div>
<script>
let current = 0;
const slides = document.querySelectorAll('.slide');
document.addEventListener('keydown', e => {
  if (e.key === 'ArrowRight' && current < slides.length-1) { slides[current].classList.remove('active'); slides[++current].classList.add('active'); }
  if (e.key === 'ArrowLeft' && current > 0) { slides[current].classList.remove('active'); slides[--current].classList.add('active'); }
});
</script></body></html>
```

### Codebase Walkthroughs
When asked to explain a codebase:
1. Start with the entry point (main.rs, index.ts, etc.)
2. Draw the module dependency graph
3. Explain each layer's responsibility
4. Identify the key data structures and how they flow
5. Call out design patterns used (and why)
6. Note tech debt or areas that could be improved

### Concept Logging
When teaching a concept, optionally log it for spaced repetition:
```json
// workspace/scratch/learning-queue.json
{
  "concepts": [
    {
      "topic": "B-rep vs CSG",
      "summary": "B-rep stores boundary surfaces, CSG stores construction tree. B-rep is faster for rendering, CSG for editing.",
      "learned": "2026-02-07",
      "review_after": "2026-02-14"
    }
  ]
}
```

## Rules
- Default to brief explanations unless Justin asks for depth
- Use ASCII diagrams over external tools — they're instantly readable
- HTML presentations only when complexity warrants it (architecture overviews, onboarding)
- Don't over-explain trivial changes

## Changelog
- 2026-02-07: Initial creation — explanatory mode, diagrams, HTML presentations
