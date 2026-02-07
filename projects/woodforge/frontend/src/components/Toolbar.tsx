import { useStore } from '../store';
import type { Tool } from '../types/scene';

const tools: { id: Tool; label: string; icon: string }[] = [
  { id: 'select', label: 'Select', icon: '⊡' },
  { id: 'move', label: 'Move', icon: '✥' },
  { id: 'rotate', label: 'Rotate', icon: '↻' },
  { id: 'add', label: 'Add Board', icon: '+' },
];

export function Toolbar() {
  const activeTool = useStore((s) => s.activeTool);
  const setActiveTool = useStore((s) => s.setActiveTool);
  const setLumberPickerOpen = useStore((s) => s.setLumberPickerOpen);

  const handleClick = (tool: Tool) => {
    if (tool === 'add') {
      setLumberPickerOpen(true);
    } else {
      setActiveTool(tool);
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
    </div>
  );
}
