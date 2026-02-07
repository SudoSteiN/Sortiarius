declare module '@wasm/woodforge_wasm_bridge' {
  export default function init(): Promise<void>;
  export function init_renderer(canvas: HTMLCanvasElement, width: number, height: number): Promise<void>;
  export function render(): void;
  export function resize(width: number, height: number): void;
  export function camera_orbit(dx: number, dy: number): void;
  export function camera_pan(dx: number, dy: number): void;
  export function camera_zoom(delta: number): void;
  export function add_standard_board(label: string): any;
  export function add_custom_board(width: number, height: number, depth: number): any;
  export function remove_node(id: string): any;
  export function set_position(id: string, x: number, y: number, z: number): any;
  export function set_rotation(id: string, x: number, y: number, z: number): any;
  export function get_scene_tree(): any;
  export function compute_snap_position(id: string, x: number, y: number, z: number): any;
  export function get_lumber_catalog(): any;
  export function get_standard_lengths(): any;
  export function format_dim(inches: number): string;
  export function parse_dim(input: string): number | null;
  export function set_unit_system(system: string): void;
  export function get_unit_system(): string;
  export function set_grid_snap(enabled: boolean): void;
  export function get_grid_snap(): boolean;
}
