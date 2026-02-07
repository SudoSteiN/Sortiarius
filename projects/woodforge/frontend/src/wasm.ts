// Lazy-loaded WASM module wrapper
let wasmModule: any = null;

export async function initWasm(): Promise<void> {
  if (wasmModule) return;
  const wasm = await import('@wasm/woodforge_wasm_bridge');
  await wasm.default();
  wasmModule = wasm;
}

export function getWasm() { return wasmModule; }

// ========== Renderer ==========
export async function initRenderer(canvas: HTMLCanvasElement, width: number, height: number): Promise<void> {
  if (!wasmModule) throw new Error('WASM not initialized');
  await wasmModule.init_renderer(canvas, width, height);
}
export function render(): void { wasmModule?.render(); }
export function resize(width: number, height: number): void { wasmModule?.resize(width, height); }

// ========== Camera ==========
export function cameraOrbit(dx: number, dy: number): void { wasmModule?.camera_orbit(dx, dy); }
export function cameraPan(dx: number, dy: number): void { wasmModule?.camera_pan(dx, dy); }
export function cameraZoom(delta: number): void { wasmModule?.camera_zoom(delta); }

// ========== Scene ==========
export function addStandardBoard(label: string): any { return wasmModule?.add_standard_board(label); }
export function addCustomBoard(width: number, height: number, depth: number): any { return wasmModule?.add_custom_board(width, height, depth); }
export function removeNode(id: string): any { return wasmModule?.remove_node(id); }
export function setPosition(id: string, x: number, y: number, z: number): any { return wasmModule?.set_position(id, x, y, z); }
export function setRotation(id: string, x: number, y: number, z: number): any { return wasmModule?.set_rotation(id, x, y, z); }
export function getSceneTree(): any { return wasmModule?.get_scene_tree(); }
export function selectNode(id: string): any { return wasmModule?.select_node(id); }
export function deselectAll(): void { wasmModule?.deselect_all(); }
export function pickObject(screenX: number, screenY: number): string | null { return wasmModule?.pick_object(screenX, screenY) ?? null; }
export function computeSnapPosition(id: string, x: number, y: number, z: number): any { return wasmModule?.compute_snap_position(id, x, y, z); }

// ========== Lumber & Units ==========
export function getLumberCatalog(): any { return wasmModule?.get_lumber_catalog(); }
export function getStandardLengths(): any { return wasmModule?.get_standard_lengths(); }
export function formatDim(inches: number): string { return wasmModule?.format_dim(inches) ?? `${inches}"`; }
export function parseDim(input: string): number | null { return wasmModule?.parse_dim(input) ?? null; }
export function setUnitSystem(system: string): void { wasmModule?.set_unit_system(system); }
export function getUnitSystem(): string { return wasmModule?.get_unit_system() ?? 'imperial'; }
export function setGridSnap(enabled: boolean): void { wasmModule?.set_grid_snap(enabled); }

// ========== Species & Finish ==========
export function getAllSpecies(): any { return wasmModule?.get_all_species(); }
export function getSpecies(speciesId: string): any { return wasmModule?.get_species(speciesId); }
export function setBoardSpecies(boardId: string, speciesId: string): any { return wasmModule?.set_board_species(boardId, speciesId); }
export function clearBoardSpecies(boardId: string): boolean { return wasmModule?.clear_board_species(boardId) ?? false; }
export function getBoardSpecies(boardId: string): any { return wasmModule?.get_board_species(boardId); }
export function applyStainFinish(boardId: string, name: string, r: number, g: number, b: number, opacity: number): any { return wasmModule?.apply_stain_finish(boardId, name, r, g, b, opacity); }
export function applyPaintFinish(boardId: string, name: string, r: number, g: number, b: number, sheen: string): any { return wasmModule?.apply_paint_finish(boardId, name, r, g, b, sheen); }
export function applyOilFinish(boardId: string, name: string): any { return wasmModule?.apply_oil_finish(boardId, name); }
export function clearFinish(boardId: string): boolean { return wasmModule?.clear_finish(boardId) ?? false; }

// ========== Joinery ==========
export function getJointTypes(): any { return wasmModule?.get_joint_types(); }
export function addJoint(type: string, boardA: string, boardB: string, faceA: string, faceB: string): string | null { return wasmModule?.add_joint(type, boardA, boardB, faceA, faceB) ?? null; }
export function removeJoint(jointId: string): boolean { return wasmModule?.remove_joint(jointId) ?? false; }
export function getJointsForBoard(boardId: string): any { return wasmModule?.get_joints_for_board(boardId); }
export function getAllJoints(): any { return wasmModule?.get_all_joints(); }

// ========== Analysis ==========
export function analyzeShelfDeflection(span: number, width: number, height: number, speciesId: string, load: number): any { return wasmModule?.analyze_shelf_deflection(span, width, height, speciesId, load); }
export function analyzeCompression(length: number, width: number, height: number, speciesId: string, load: number): any { return wasmModule?.analyze_compression(length, width, height, speciesId, load); }
export function analyzeJointStrength(jointType: string, speciesId: string): any { return wasmModule?.analyze_joint_strength(jointType, speciesId); }

// ========== Output ==========
export function generateCutList(): any { return wasmModule?.generate_cut_list(); }
export function generateMaterialList(): any { return wasmModule?.generate_material_list(); }
export function generateBuildInstructions(): any { return wasmModule?.generate_build_instructions(); }
export function optimizeCutLayout(kerfWidth: number): any { return wasmModule?.optimize_cut_layout(kerfWidth); }
export function estimateProjectCost(): any { return wasmModule?.estimate_project_cost(); }

// ========== Project ==========
export function undo(): string | null { return wasmModule?.undo() ?? null; }
export function redo(): string | null { return wasmModule?.redo() ?? null; }
export function canUndo(): boolean { return wasmModule?.can_undo() ?? false; }
export function canRedo(): boolean { return wasmModule?.can_redo() ?? false; }
export function saveProject(name: string, description: string): string { return wasmModule?.save_project(name, description) ?? ''; }
export function loadProject(json: string): any { return wasmModule?.load_project(json); }
export function newProject(): any { return wasmModule?.new_project(); }
