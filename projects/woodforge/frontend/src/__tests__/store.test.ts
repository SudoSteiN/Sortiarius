import { describe, it, expect, beforeEach } from 'vitest';
import { useStore } from '../store';

describe('AppStore', () => {
  beforeEach(() => {
    // Reset store to initial state before each test
    useStore.setState({
      sceneTree: null,
      selectedNodeId: null,
      activeTool: 'select',
      unitSystem: 'imperial',
      gridSnap: true,
      wasmReady: false,
      lumberPickerOpen: false,
      rightPanel: 'properties',
      projectName: 'Untitled Project',
      statusMessage: '',
    });
  });

  describe('initial state', () => {
    it('has correct default values', () => {
      const state = useStore.getState();
      expect(state.sceneTree).toBeNull();
      expect(state.selectedNodeId).toBeNull();
      expect(state.activeTool).toBe('select');
      expect(state.unitSystem).toBe('imperial');
      expect(state.gridSnap).toBe(true);
      expect(state.wasmReady).toBe(false);
      expect(state.lumberPickerOpen).toBe(false);
      expect(state.rightPanel).toBe('properties');
      expect(state.projectName).toBe('Untitled Project');
      expect(state.statusMessage).toBe('');
    });
  });

  describe('setSceneTree', () => {
    it('updates sceneTree with a snapshot', () => {
      const tree = {
        nodes: [
          {
            id: 'node-1',
            label: 'Board 1',
            parent_id: null,
            children: [],
            position: [0, 0, 0] as [number, number, number],
            rotation: [0, 0, 0] as [number, number, number],
            has_mesh: true,
            dimensions: [48, 3.5, 1.5] as [number, number, number],
            lumber_type: '2x4',
            grain_direction: 'along',
          },
        ],
      };

      useStore.getState().setSceneTree(tree);
      expect(useStore.getState().sceneTree).toEqual(tree);
    });

    it('can set sceneTree to null', () => {
      const tree = { nodes: [] };
      useStore.getState().setSceneTree(tree);
      expect(useStore.getState().sceneTree).toEqual(tree);

      useStore.getState().setSceneTree(null);
      expect(useStore.getState().sceneTree).toBeNull();
    });
  });

  describe('setSelectedNodeId', () => {
    it('sets a node id', () => {
      useStore.getState().setSelectedNodeId('node-42');
      expect(useStore.getState().selectedNodeId).toBe('node-42');
    });

    it('clears selection with null', () => {
      useStore.getState().setSelectedNodeId('node-42');
      useStore.getState().setSelectedNodeId(null);
      expect(useStore.getState().selectedNodeId).toBeNull();
    });
  });

  describe('setActiveTool', () => {
    it('sets to move', () => {
      useStore.getState().setActiveTool('move');
      expect(useStore.getState().activeTool).toBe('move');
    });

    it('sets to rotate', () => {
      useStore.getState().setActiveTool('rotate');
      expect(useStore.getState().activeTool).toBe('rotate');
    });

    it('sets to add', () => {
      useStore.getState().setActiveTool('add');
      expect(useStore.getState().activeTool).toBe('add');
    });

    it('sets back to select', () => {
      useStore.getState().setActiveTool('move');
      useStore.getState().setActiveTool('select');
      expect(useStore.getState().activeTool).toBe('select');
    });
  });

  describe('toggleGridSnap', () => {
    it('toggles from true to false', () => {
      expect(useStore.getState().gridSnap).toBe(true);
      useStore.getState().toggleGridSnap();
      expect(useStore.getState().gridSnap).toBe(false);
    });

    it('toggles from false back to true', () => {
      useStore.getState().toggleGridSnap(); // true -> false
      useStore.getState().toggleGridSnap(); // false -> true
      expect(useStore.getState().gridSnap).toBe(true);
    });

    it('toggles multiple times correctly', () => {
      const initial = useStore.getState().gridSnap;
      useStore.getState().toggleGridSnap();
      expect(useStore.getState().gridSnap).toBe(!initial);
      useStore.getState().toggleGridSnap();
      expect(useStore.getState().gridSnap).toBe(initial);
      useStore.getState().toggleGridSnap();
      expect(useStore.getState().gridSnap).toBe(!initial);
    });
  });

  describe('setUnitSystem', () => {
    it('switches to metric', () => {
      useStore.getState().setUnitSystem('metric');
      expect(useStore.getState().unitSystem).toBe('metric');
    });

    it('switches back to imperial', () => {
      useStore.getState().setUnitSystem('metric');
      useStore.getState().setUnitSystem('imperial');
      expect(useStore.getState().unitSystem).toBe('imperial');
    });
  });

  describe('setRightPanel', () => {
    it('sets to species panel', () => {
      useStore.getState().setRightPanel('species');
      expect(useStore.getState().rightPanel).toBe('species');
    });

    it('sets to joinery panel', () => {
      useStore.getState().setRightPanel('joinery');
      expect(useStore.getState().rightPanel).toBe('joinery');
    });

    it('sets to analysis panel', () => {
      useStore.getState().setRightPanel('analysis');
      expect(useStore.getState().rightPanel).toBe('analysis');
    });

    it('sets to output panel', () => {
      useStore.getState().setRightPanel('output');
      expect(useStore.getState().rightPanel).toBe('output');
    });

    it('sets to project panel', () => {
      useStore.getState().setRightPanel('project');
      expect(useStore.getState().rightPanel).toBe('project');
    });

    it('sets back to properties panel', () => {
      useStore.getState().setRightPanel('output');
      useStore.getState().setRightPanel('properties');
      expect(useStore.getState().rightPanel).toBe('properties');
    });
  });

  describe('setProjectName', () => {
    it('updates the project name', () => {
      useStore.getState().setProjectName('My Bookshelf');
      expect(useStore.getState().projectName).toBe('My Bookshelf');
    });

    it('allows empty string', () => {
      useStore.getState().setProjectName('');
      expect(useStore.getState().projectName).toBe('');
    });
  });

  describe('setStatusMessage', () => {
    it('sets a status message', () => {
      useStore.getState().setStatusMessage('Board added to scene');
      expect(useStore.getState().statusMessage).toBe('Board added to scene');
    });

    it('clears the status message', () => {
      useStore.getState().setStatusMessage('Something happened');
      useStore.getState().setStatusMessage('');
      expect(useStore.getState().statusMessage).toBe('');
    });
  });

  describe('setWasmReady', () => {
    it('sets wasm ready to true', () => {
      useStore.getState().setWasmReady(true);
      expect(useStore.getState().wasmReady).toBe(true);
    });

    it('sets wasm ready back to false', () => {
      useStore.getState().setWasmReady(true);
      useStore.getState().setWasmReady(false);
      expect(useStore.getState().wasmReady).toBe(false);
    });
  });

  describe('setLumberPickerOpen', () => {
    it('opens the lumber picker', () => {
      useStore.getState().setLumberPickerOpen(true);
      expect(useStore.getState().lumberPickerOpen).toBe(true);
    });

    it('closes the lumber picker', () => {
      useStore.getState().setLumberPickerOpen(true);
      useStore.getState().setLumberPickerOpen(false);
      expect(useStore.getState().lumberPickerOpen).toBe(false);
    });
  });
});
