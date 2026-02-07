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

export type Tool = 'select' | 'move' | 'rotate' | 'add';
export type UnitSystem = 'imperial' | 'metric';
