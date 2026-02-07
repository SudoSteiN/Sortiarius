export interface SceneNodeSnapshot {
  id: string;
  label: string;
  parent_id: string | null;
  children: string[];
  position: [number, number, number];
  rotation: [number, number, number];
  has_mesh: boolean;
  dimensions: [number, number, number] | null;
  lumber_type: string | null;
  grain_direction: string | null;
}

export interface SceneTreeSnapshot {
  nodes: SceneNodeSnapshot[];
}

export interface SnapResult {
  snapped_position: { x: number; y: number; z: number };
  snap_type: 'Grid' | 'Face' | 'Edge' | 'None';
}

export interface LumberSize {
  nominal_label: string;
  category: 'Dimensional' | 'Board' | 'Sheet';
  actual_width: number;
  actual_height: number;
}

export interface SheetSize {
  label: string;
  width: number;
  height: number;
  thickness: number;
}

export interface LumberCatalog {
  sizes: LumberSize[];
  sheets: SheetSize[];
  standard_lengths: number[];
}

export interface WoodSpecies {
  id: string;
  name: string;
  density_lb_ft3: number;
  modulus_of_elasticity: number;
  modulus_of_rupture: number;
  janka_hardness: number;
  compressive_strength: number;
  color: [number, number, number];
  grain_scale: number;
  workability: 'Easy' | 'Moderate' | 'Difficult';
}

export interface JointInfo {
  id: string;
  joint_type: string;
  board_a: string;
  board_b: string;
  face_a: string;
  face_b: string;
}

export interface DeflectionResult {
  deflection_inches: number;
  max_allowable: number;
  ratio: number;
  status: 'Green' | 'Yellow' | 'Red';
  span_ratio: string;
}

export interface CompressionResult {
  applied_load_lbs: number;
  capacity_lbs: number;
  utilization: number;
  status: 'Green' | 'Yellow' | 'Red';
  buckling_risk: boolean;
}

export interface JointStrengthResult {
  base_strength: number;
  species_factor: number;
  adjusted_strength: number;
  status: 'Green' | 'Yellow' | 'Red';
  recommendation: string;
}

export interface CutListEntry {
  piece_id: string;
  label: string;
  material: string;
  length: number;
  width: number;
  thickness: number;
  quantity: number;
  grain_direction: string;
  joint_notes: string[];
  node_ids: string[];
}

export interface CutList {
  entries: CutListEntry[];
  total_pieces: number;
  total_cuts: number;
}

export interface MaterialListEntry {
  material: string;
  nominal_size: string;
  length: number;
  quantity: number;
  board_feet: number;
}

export interface MaterialList {
  entries: MaterialListEntry[];
  total_board_feet: number;
}

export interface BuildStep {
  step_number: number;
  description: string;
  tools_needed: string[];
}

export interface BuildInstructions {
  steps: BuildStep[];
}

export interface CostEstimate {
  materials: { species: string; board_feet: number; price_per_bf: number; subtotal: number }[];
  hardware: { item: string; quantity: number; unit_price: number; subtotal: number }[];
  material_subtotal: number;
  hardware_subtotal: number;
  waste_factor: number;
  waste_cost: number;
  total: number;
  notes: string[];
}

export interface OptimizationResult {
  layouts: {
    stock: { label: string; length: number; width: number; height: number };
    placements: { piece_id: string; piece_label: string; start_offset: number; length: number }[];
    used_length: number;
    waste_length: number;
    utilization: number;
  }[];
  total_stock_boards: number;
  total_waste_inches: number;
  total_waste_percent: number;
  kerf_width: number;
  unplaceable: { id: string; label: string; length: number; width: number; material: string }[];
}

export type Tool = 'select' | 'move' | 'rotate' | 'add';
export type UnitSystem = 'imperial' | 'metric';
export type RightPanel = 'properties' | 'species' | 'joinery' | 'analysis' | 'output' | 'project';
