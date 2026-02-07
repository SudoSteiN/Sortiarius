// Lazy-loaded WASM module wrapper
// The actual WASM module is loaded dynamically to avoid blocking initial render

let wasmModule: any = null;

export async function initWasm(): Promise<void> {
  if (wasmModule) return;
  const wasm = await import('@wasm/woodforge_wasm_bridge');
  await wasm.default();
  wasmModule = wasm;
}

export function getWasm() {
  return wasmModule;
}

export async function initRenderer(canvas: HTMLCanvasElement, width: number, height: number): Promise<void> {
  if (!wasmModule) throw new Error('WASM not initialized');
  await wasmModule.init_renderer(canvas, width, height);
}

export function render(): void {
  wasmModule?.render();
}

export function resize(width: number, height: number): void {
  wasmModule?.resize(width, height);
}

export function cameraOrbit(dx: number, dy: number): void {
  wasmModule?.camera_orbit(dx, dy);
}

export function cameraPan(dx: number, dy: number): void {
  wasmModule?.camera_pan(dx, dy);
}

export function cameraZoom(delta: number): void {
  wasmModule?.camera_zoom(delta);
}

export function addStandardBoard(label: string): any {
  return wasmModule?.add_standard_board(label);
}

export function addCustomBoard(width: number, height: number, depth: number): any {
  return wasmModule?.add_custom_board(width, height, depth);
}

export function removeNode(id: string): any {
  return wasmModule?.remove_node(id);
}

export function setPosition(id: string, x: number, y: number, z: number): any {
  return wasmModule?.set_position(id, x, y, z);
}

export function setRotation(id: string, x: number, y: number, z: number): any {
  return wasmModule?.set_rotation(id, x, y, z);
}

export function getSceneTree(): any {
  return wasmModule?.get_scene_tree();
}

export function getLumberCatalog(): any {
  return wasmModule?.get_lumber_catalog();
}

export function getStandardLengths(): any {
  return wasmModule?.get_standard_lengths();
}

export function formatDim(inches: number): string {
  return wasmModule?.format_dim(inches) ?? `${inches}"`;
}

export function parseDim(input: string): number | null {
  return wasmModule?.parse_dim(input) ?? null;
}

export function setUnitSystem(system: string): void {
  wasmModule?.set_unit_system(system);
}

export function getUnitSystem(): string {
  return wasmModule?.get_unit_system() ?? 'imperial';
}

export function setGridSnap(enabled: boolean): void {
  wasmModule?.set_grid_snap(enabled);
}

export function computeSnapPosition(id: string, x: number, y: number, z: number): any {
  return wasmModule?.compute_snap_position(id, x, y, z);
}
