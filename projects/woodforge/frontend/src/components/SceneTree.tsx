import { useStore } from '../store';
import type { SceneNodeSnapshot } from '../types/scene';
import { removeNode } from '../wasm';

function TreeNode({ node }: { node: SceneNodeSnapshot }) {
  const selectedNodeId = useStore((s) => s.selectedNodeId);
  const setSelectedNodeId = useStore((s) => s.setSelectedNodeId);
  const setSceneTree = useStore((s) => s.setSceneTree);
  const isSelected = selectedNodeId === node.id;

  const handleDelete = (e: React.MouseEvent) => {
    e.stopPropagation();
    const snapshot = removeNode(node.id);
    if (snapshot) {
      setSceneTree(snapshot);
      if (isSelected) setSelectedNodeId(null);
    }
  };

  return (
    <div
      className={`flex items-center justify-between px-2 py-1 cursor-pointer rounded text-sm ${
        isSelected
          ? 'bg-blue-600/20 text-blue-300 border border-blue-500/30'
          : 'text-gray-300 hover:bg-gray-700/50 border border-transparent'
      }`}
      onClick={() => setSelectedNodeId(node.id)}
    >
      <div className="flex items-center gap-1.5 min-w-0">
        <span className="text-xs opacity-60">&#9638;</span>
        <span className="truncate">{node.label}</span>
      </div>
      <button
        onClick={handleDelete}
        className="text-gray-500 hover:text-red-400 text-xs px-1 opacity-0 group-hover:opacity-100 hover:opacity-100 transition-opacity"
        title="Remove"
      >
        x
      </button>
    </div>
  );
}

export function SceneTree() {
  const sceneTree = useStore((s) => s.sceneTree);
  const nodes = sceneTree?.nodes ?? [];
  const rootNodes = nodes.filter((n) => !n.parent_id);

  return (
    <div className="flex flex-col h-full bg-[#16213e] border-r border-gray-700">
      <div className="px-3 py-2 text-xs font-semibold text-gray-400 uppercase tracking-wider border-b border-gray-700">
        Scene
      </div>
      <div className="flex-1 overflow-y-auto p-1.5 space-y-0.5">
        {rootNodes.length === 0 ? (
          <div className="text-xs text-gray-500 text-center py-8 px-2">
            No objects. Add a board to get started.
          </div>
        ) : (
          rootNodes.map((node) => <TreeNode key={node.id} node={node} />)
        )}
      </div>
      <div className="px-3 py-1.5 text-xs text-gray-500 border-t border-gray-700">
        {nodes.length} object{nodes.length !== 1 ? 's' : ''}
      </div>
    </div>
  );
}
