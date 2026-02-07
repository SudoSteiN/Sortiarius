import { create } from 'zustand';
import type { SceneTreeSnapshot, Tool, UnitSystem } from '../types/scene';

interface AppStore {
  sceneTree: SceneTreeSnapshot | null;
  selectedNodeId: string | null;
  activeTool: Tool;
  unitSystem: UnitSystem;
  gridSnap: boolean;
  wasmReady: boolean;
  lumberPickerOpen: boolean;

  setSceneTree: (tree: SceneTreeSnapshot | null) => void;
  setSelectedNodeId: (id: string | null) => void;
  setActiveTool: (tool: Tool) => void;
  setUnitSystem: (system: UnitSystem) => void;
  toggleGridSnap: () => void;
  setWasmReady: (ready: boolean) => void;
  setLumberPickerOpen: (open: boolean) => void;
}

export const useStore = create<AppStore>((set) => ({
  sceneTree: null,
  selectedNodeId: null,
  activeTool: 'select',
  unitSystem: 'imperial',
  gridSnap: true,
  wasmReady: false,
  lumberPickerOpen: false,

  setSceneTree: (tree) => set({ sceneTree: tree }),
  setSelectedNodeId: (id) => set({ selectedNodeId: id }),
  setActiveTool: (tool) => set({ activeTool: tool }),
  setUnitSystem: (system) => set({ unitSystem: system }),
  toggleGridSnap: () => set((s) => ({ gridSnap: !s.gridSnap })),
  setWasmReady: (ready) => set({ wasmReady: ready }),
  setLumberPickerOpen: (open) => set({ lumberPickerOpen: open }),
}));
