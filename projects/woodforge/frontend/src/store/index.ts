import { create } from 'zustand';
import type { SceneTreeSnapshot, Tool, UnitSystem, RightPanel } from '../types/scene';

interface AppStore {
  sceneTree: SceneTreeSnapshot | null;
  selectedNodeId: string | null;
  activeTool: Tool;
  unitSystem: UnitSystem;
  gridSnap: boolean;
  wasmReady: boolean;
  lumberPickerOpen: boolean;
  rightPanel: RightPanel;
  projectName: string;
  statusMessage: string;

  setSceneTree: (tree: SceneTreeSnapshot | null) => void;
  setSelectedNodeId: (id: string | null) => void;
  setActiveTool: (tool: Tool) => void;
  setUnitSystem: (system: UnitSystem) => void;
  toggleGridSnap: () => void;
  setWasmReady: (ready: boolean) => void;
  setLumberPickerOpen: (open: boolean) => void;
  setRightPanel: (panel: RightPanel) => void;
  setProjectName: (name: string) => void;
  setStatusMessage: (msg: string) => void;
}

export const useStore = create<AppStore>((set) => ({
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

  setSceneTree: (tree) => set({ sceneTree: tree }),
  setSelectedNodeId: (id) => set({ selectedNodeId: id }),
  setActiveTool: (tool) => set({ activeTool: tool }),
  setUnitSystem: (system) => set({ unitSystem: system }),
  toggleGridSnap: () => set((s) => ({ gridSnap: !s.gridSnap })),
  setWasmReady: (ready) => set({ wasmReady: ready }),
  setLumberPickerOpen: (open) => set({ lumberPickerOpen: open }),
  setRightPanel: (panel) => set({ rightPanel: panel }),
  setProjectName: (name) => set({ projectName: name }),
  setStatusMessage: (msg) => set({ statusMessage: msg }),
}));
