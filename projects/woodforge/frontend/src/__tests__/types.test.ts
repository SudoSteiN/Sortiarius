import { describe, it, expect } from 'vitest';
import type {
  SceneNodeSnapshot,
  SceneTreeSnapshot,
  SnapResult,
  LumberSize,
  SheetSize,
  LumberCatalog,
  WoodSpecies,
  JointInfo,
  DeflectionResult,
  CompressionResult,
  JointStrengthResult,
  CutListEntry,
  CutList,
  MaterialListEntry,
  MaterialList,
  BuildStep,
  BuildInstructions,
  CostEstimate,
  OptimizationResult,
  Tool,
  UnitSystem,
  RightPanel,
} from '../types/scene';

describe('Type definitions', () => {
  describe('SceneNodeSnapshot', () => {
    it('can create a valid scene node', () => {
      const node: SceneNodeSnapshot = {
        id: 'node-1',
        label: 'Side Panel',
        parent_id: null,
        children: ['node-2', 'node-3'],
        position: [0, 0, 0],
        rotation: [0, 90, 0],
        has_mesh: true,
        dimensions: [36, 11.25, 0.75],
        lumber_type: '1x12',
        grain_direction: 'along',
      };

      expect(node.id).toBe('node-1');
      expect(node.label).toBe('Side Panel');
      expect(node.parent_id).toBeNull();
      expect(node.children).toHaveLength(2);
      expect(node.position).toEqual([0, 0, 0]);
      expect(node.rotation).toEqual([0, 90, 0]);
      expect(node.has_mesh).toBe(true);
      expect(node.dimensions).toEqual([36, 11.25, 0.75]);
      expect(node.lumber_type).toBe('1x12');
      expect(node.grain_direction).toBe('along');
    });

    it('supports nullable fields', () => {
      const node: SceneNodeSnapshot = {
        id: 'node-root',
        label: 'Root',
        parent_id: null,
        children: [],
        position: [0, 0, 0],
        rotation: [0, 0, 0],
        has_mesh: false,
        dimensions: null,
        lumber_type: null,
        grain_direction: null,
      };

      expect(node.dimensions).toBeNull();
      expect(node.lumber_type).toBeNull();
      expect(node.grain_direction).toBeNull();
    });
  });

  describe('SceneTreeSnapshot', () => {
    it('can create an empty tree', () => {
      const tree: SceneTreeSnapshot = { nodes: [] };
      expect(tree.nodes).toHaveLength(0);
    });

    it('can create a tree with multiple nodes', () => {
      const tree: SceneTreeSnapshot = {
        nodes: [
          {
            id: 'a',
            label: 'A',
            parent_id: null,
            children: ['b'],
            position: [0, 0, 0],
            rotation: [0, 0, 0],
            has_mesh: false,
            dimensions: null,
            lumber_type: null,
            grain_direction: null,
          },
          {
            id: 'b',
            label: 'B',
            parent_id: 'a',
            children: [],
            position: [10, 0, 0],
            rotation: [0, 0, 0],
            has_mesh: true,
            dimensions: [48, 3.5, 1.5],
            lumber_type: '2x4',
            grain_direction: 'along',
          },
        ],
      };

      expect(tree.nodes).toHaveLength(2);
      expect(tree.nodes[1].parent_id).toBe('a');
    });
  });

  describe('SnapResult', () => {
    it('can create snap results for each snap type', () => {
      const gridSnap: SnapResult = {
        snapped_position: { x: 1, y: 0, z: 2 },
        snap_type: 'Grid',
      };
      expect(gridSnap.snap_type).toBe('Grid');

      const faceSnap: SnapResult = {
        snapped_position: { x: 3.5, y: 1.5, z: 0 },
        snap_type: 'Face',
      };
      expect(faceSnap.snap_type).toBe('Face');

      const edgeSnap: SnapResult = {
        snapped_position: { x: 0, y: 0, z: 0 },
        snap_type: 'Edge',
      };
      expect(edgeSnap.snap_type).toBe('Edge');

      const noSnap: SnapResult = {
        snapped_position: { x: 1.337, y: 2.718, z: 3.14 },
        snap_type: 'None',
      };
      expect(noSnap.snap_type).toBe('None');
    });
  });

  describe('LumberSize', () => {
    it('can represent dimensional lumber', () => {
      const twoByFour: LumberSize = {
        nominal_label: '2x4',
        category: 'Dimensional',
        actual_width: 3.5,
        actual_height: 1.5,
      };
      expect(twoByFour.nominal_label).toBe('2x4');
      expect(twoByFour.category).toBe('Dimensional');
      expect(twoByFour.actual_width).toBe(3.5);
      expect(twoByFour.actual_height).toBe(1.5);
    });

    it('can represent board lumber', () => {
      const oneByTwelve: LumberSize = {
        nominal_label: '1x12',
        category: 'Board',
        actual_width: 11.25,
        actual_height: 0.75,
      };
      expect(oneByTwelve.category).toBe('Board');
    });

    it('can represent sheet goods', () => {
      const plywood: LumberSize = {
        nominal_label: '4x8 3/4"',
        category: 'Sheet',
        actual_width: 48,
        actual_height: 96,
      };
      expect(plywood.category).toBe('Sheet');
    });
  });

  describe('SheetSize', () => {
    it('can represent a plywood sheet', () => {
      const sheet: SheetSize = {
        label: '4x8 3/4"',
        width: 48,
        height: 96,
        thickness: 0.75,
      };
      expect(sheet.label).toBe('4x8 3/4"');
      expect(sheet.thickness).toBe(0.75);
    });
  });

  describe('LumberCatalog', () => {
    it('can create a catalog with sizes, sheets, and standard lengths', () => {
      const catalog: LumberCatalog = {
        sizes: [
          { nominal_label: '2x4', category: 'Dimensional', actual_width: 3.5, actual_height: 1.5 },
        ],
        sheets: [
          { label: '4x8 3/4"', width: 48, height: 96, thickness: 0.75 },
        ],
        standard_lengths: [72, 96, 120, 144],
      };

      expect(catalog.sizes).toHaveLength(1);
      expect(catalog.sheets).toHaveLength(1);
      expect(catalog.standard_lengths).toContain(96);
    });
  });

  describe('WoodSpecies', () => {
    it('can represent a wood species with full properties', () => {
      const oak: WoodSpecies = {
        id: 'red-oak',
        name: 'Red Oak',
        density_lb_ft3: 44,
        modulus_of_elasticity: 1820000,
        modulus_of_rupture: 14300,
        janka_hardness: 1290,
        compressive_strength: 6760,
        color: [180, 130, 80],
        grain_scale: 1.0,
        workability: 'Moderate',
      };

      expect(oak.id).toBe('red-oak');
      expect(oak.name).toBe('Red Oak');
      expect(oak.density_lb_ft3).toBe(44);
      expect(oak.color).toEqual([180, 130, 80]);
      expect(oak.workability).toBe('Moderate');
    });

    it('supports all workability levels', () => {
      const easy: WoodSpecies = {
        id: 'pine',
        name: 'Pine',
        density_lb_ft3: 28,
        modulus_of_elasticity: 1200000,
        modulus_of_rupture: 8600,
        janka_hardness: 380,
        compressive_strength: 4800,
        color: [220, 200, 160],
        grain_scale: 0.8,
        workability: 'Easy',
      };
      expect(easy.workability).toBe('Easy');

      const difficult: WoodSpecies = {
        id: 'ipe',
        name: 'Ipe',
        density_lb_ft3: 69,
        modulus_of_elasticity: 3140000,
        modulus_of_rupture: 25400,
        janka_hardness: 3684,
        compressive_strength: 13640,
        color: [90, 60, 30],
        grain_scale: 0.5,
        workability: 'Difficult',
      };
      expect(difficult.workability).toBe('Difficult');
    });
  });

  describe('JointInfo', () => {
    it('can represent a joint between two boards', () => {
      const joint: JointInfo = {
        id: 'joint-1',
        joint_type: 'mortise-and-tenon',
        board_a: 'node-1',
        board_b: 'node-2',
        face_a: 'end',
        face_b: 'face',
      };

      expect(joint.joint_type).toBe('mortise-and-tenon');
      expect(joint.board_a).toBe('node-1');
      expect(joint.board_b).toBe('node-2');
    });
  });

  describe('DeflectionResult', () => {
    it('can represent a passing deflection analysis', () => {
      const result: DeflectionResult = {
        deflection_inches: 0.05,
        max_allowable: 0.25,
        ratio: 0.2,
        status: 'Green',
        span_ratio: 'L/360',
      };

      expect(result.status).toBe('Green');
      expect(result.ratio).toBeLessThan(1);
    });

    it('can represent a failing deflection analysis', () => {
      const result: DeflectionResult = {
        deflection_inches: 0.5,
        max_allowable: 0.25,
        ratio: 2.0,
        status: 'Red',
        span_ratio: 'L/180',
      };

      expect(result.status).toBe('Red');
      expect(result.deflection_inches).toBeGreaterThan(result.max_allowable);
    });
  });

  describe('CompressionResult', () => {
    it('can represent a compression analysis', () => {
      const result: CompressionResult = {
        applied_load_lbs: 500,
        capacity_lbs: 2000,
        utilization: 0.25,
        status: 'Green',
        buckling_risk: false,
      };

      expect(result.utilization).toBe(0.25);
      expect(result.buckling_risk).toBe(false);
    });
  });

  describe('JointStrengthResult', () => {
    it('can represent a joint strength analysis', () => {
      const result: JointStrengthResult = {
        base_strength: 1000,
        species_factor: 0.85,
        adjusted_strength: 850,
        status: 'Yellow',
        recommendation: 'Consider reinforcement with dowels',
      };

      expect(result.adjusted_strength).toBe(result.base_strength * result.species_factor);
      expect(result.status).toBe('Yellow');
    });
  });

  describe('CutList and CutListEntry', () => {
    it('can create a cut list with entries', () => {
      const entry: CutListEntry = {
        piece_id: 'piece-1',
        label: 'Shelf',
        material: 'Red Oak',
        length: 36,
        width: 11.25,
        thickness: 0.75,
        quantity: 4,
        grain_direction: 'along',
        joint_notes: ['dado on each end'],
        node_ids: ['node-3', 'node-4', 'node-5', 'node-6'],
      };

      const cutList: CutList = {
        entries: [entry],
        total_pieces: 4,
        total_cuts: 4,
      };

      expect(cutList.entries).toHaveLength(1);
      expect(cutList.total_pieces).toBe(4);
      expect(entry.joint_notes).toContain('dado on each end');
    });
  });

  describe('MaterialList and MaterialListEntry', () => {
    it('can create a material list', () => {
      const entry: MaterialListEntry = {
        material: 'Red Oak',
        nominal_size: '1x12',
        length: 96,
        quantity: 2,
        board_feet: 16,
      };

      const list: MaterialList = {
        entries: [entry],
        total_board_feet: 16,
      };

      expect(list.entries).toHaveLength(1);
      expect(list.total_board_feet).toBe(16);
    });
  });

  describe('BuildStep and BuildInstructions', () => {
    it('can create build instructions', () => {
      const step: BuildStep = {
        step_number: 1,
        description: 'Cut all shelves to length',
        tools_needed: ['miter saw', 'tape measure'],
      };

      const instructions: BuildInstructions = {
        steps: [step],
      };

      expect(instructions.steps).toHaveLength(1);
      expect(instructions.steps[0].tools_needed).toContain('miter saw');
    });
  });

  describe('CostEstimate', () => {
    it('can create a cost estimate', () => {
      const estimate: CostEstimate = {
        materials: [
          { species: 'Red Oak', board_feet: 16, price_per_bf: 8.5, subtotal: 136 },
        ],
        hardware: [
          { item: 'Wood Screws #8 x 2"', quantity: 24, unit_price: 0.15, subtotal: 3.6 },
        ],
        material_subtotal: 136,
        hardware_subtotal: 3.6,
        waste_factor: 0.15,
        waste_cost: 20.4,
        total: 160,
        notes: ['Prices based on average retail'],
      };

      expect(estimate.total).toBe(160);
      expect(estimate.materials).toHaveLength(1);
      expect(estimate.hardware).toHaveLength(1);
      expect(estimate.waste_factor).toBe(0.15);
    });
  });

  describe('OptimizationResult', () => {
    it('can create an optimization result', () => {
      const result: OptimizationResult = {
        layouts: [
          {
            stock: { label: '1x12 x 8ft', length: 96, width: 11.25, height: 0.75 },
            placements: [
              { piece_id: 'p1', piece_label: 'Shelf A', start_offset: 0, length: 36 },
              { piece_id: 'p2', piece_label: 'Shelf B', start_offset: 36.125, length: 36 },
            ],
            used_length: 72.125,
            waste_length: 23.875,
            utilization: 0.751,
          },
        ],
        total_stock_boards: 1,
        total_waste_inches: 23.875,
        total_waste_percent: 24.9,
        kerf_width: 0.125,
        unplaceable: [],
      };

      expect(result.layouts).toHaveLength(1);
      expect(result.layouts[0].placements).toHaveLength(2);
      expect(result.total_stock_boards).toBe(1);
      expect(result.unplaceable).toHaveLength(0);
    });
  });

  describe('Type aliases', () => {
    it('Tool type accepts valid values', () => {
      const tools: Tool[] = ['select', 'move', 'rotate', 'add'];
      expect(tools).toHaveLength(4);
      expect(tools).toContain('select');
      expect(tools).toContain('move');
      expect(tools).toContain('rotate');
      expect(tools).toContain('add');
    });

    it('UnitSystem type accepts valid values', () => {
      const systems: UnitSystem[] = ['imperial', 'metric'];
      expect(systems).toHaveLength(2);
    });

    it('RightPanel type accepts valid values', () => {
      const panels: RightPanel[] = [
        'properties',
        'species',
        'joinery',
        'analysis',
        'output',
        'project',
      ];
      expect(panels).toHaveLength(6);
    });
  });
});
