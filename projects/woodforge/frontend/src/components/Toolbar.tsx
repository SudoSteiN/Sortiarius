import { useStore } from '../store';
import type { Tool, RightPanel } from '../types/scene';
import { undo, redo, canUndo, canRedo } from '../wasm';

const tools: { id: Tool; label: string; icon: string }[] = [
  { id: 'select', label: 'Select', icon: '⊡' },
  { id: 'move', label: 'Move', icon: '✥' },
  { id: 'rotate', label: 'Rotate', icon: '↻' },
  { id: 'add', label: 'Add Board', icon: '+' },
];

const panelTabs: { id: RightPanel; label: string }[] = [
  { id: 'properties', label: 'Properties' },
  { id: 'species', label: 'Species' },
  { id: 'joinery', label: 'Joinery' },
  { id: 'analysis', label: 'Analysis' },
  { id: 'output', label: 'Output' },
  { id: 'project', label: 'Project' },
];

export function Toolbar() {
  const activeTool = useStore((s) => s.activeTool);
  const setActiveTool = useStore((s) => s.setActiveTool);
  const setLumberPickerOpen = useStore((s) => s.setLumberPickerOpen);
  const rightPanel = useStore((s) => s.rightPanel);
  const setRightPanel = useStore((s) => s.setRightPanel);
  const setSceneTree = useStore((s) => s.setSceneTree);

  const handleClick = (tool: Tool) => {
    if (tool === 'add') {
      setLumberPickerOpen(true);
    } else {
      setActiveTool(tool);
    }
  };

  const handleUndo = () => {
    const result = undo();
    if (result) {
      const snapshot = typeof result === 'string' ? JSON.parse(result) : result;
      setSceneTree(snapshot);
    }
  };

  const handleRedo = () => {
    const result = redo();
    if (result) {
      const snapshot = typeof result === 'string' ? JSON.parse(result) : result;
      setSceneTree(snapshot);
    }
  };

  return (
    <div className="flex items-center gap-1 px-3 py-1.5 bg-[#16213e] border-b border-gray-700">
      <span className="text-sm font-bold text-gray-300 mr-3">WoodForge</span>
      <div className="h-4 w-px bg-gray-600 mx-1" />
      {tools.map((tool) => (
        <button
          key={tool.id}
          onClick={() => handleClick(tool.id)}
          className={`px-3 py-1 text-sm rounded transition-colors ${
            activeTool === tool.id && tool.id !== 'add'
              ? 'bg-blue-600/30 text-blue-300 border border-blue-500/50'
              : 'bg-gray-700/50 text-gray-300 border border-transparent hover:bg-gray-600/50'
          }`}
          title={tool.label}
        >
          <span className="mr-1">{tool.icon}</span>
          {tool.label}
        </button>
      ))}

      {/* Separator */}
      <div className="h-4 w-px bg-gray-600 mx-1" />

      {/* Undo / Redo */}
      <button
        onClick={handleUndo}
        disabled={!canUndo()}
        className={`px-3 py-1 text-sm rounded transition-colors ${
          canUndo()
            ? 'bg-gray-700/50 text-gray-300 border border-transparent hover:bg-gray-600/50'
            : 'bg-gray-800/30 text-gray-600 border border-transparent cursor-not-allowed'
        }`}
        title="Undo"
      >
        Undo
      </button>
      <button
        onClick={handleRedo}
        disabled={!canRedo()}
        className={`px-3 py-1 text-sm rounded transition-colors ${
          canRedo()
            ? 'bg-gray-700/50 text-gray-300 border border-transparent hover:bg-gray-600/50'
            : 'bg-gray-800/30 text-gray-600 border border-transparent cursor-not-allowed'
        }`}
        title="Redo"
      >
        Redo
      </button>

      {/* Separator */}
      <div className="h-4 w-px bg-gray-600 mx-1" />

      {/* Right Panel Tabs */}
      {panelTabs.map((tab) => (
        <button
          key={tab.id}
          onClick={() => setRightPanel(tab.id)}
          className={`px-2 py-1 text-xs rounded transition-colors ${
            rightPanel === tab.id
              ? 'bg-blue-600/30 text-blue-300 border border-blue-500/50'
              : 'bg-gray-700/50 text-gray-300 border border-transparent hover:bg-gray-600/50'
          }`}
          title={tab.label}
        >
          {tab.label}
        </button>
      ))}
    </div>
  );
}
