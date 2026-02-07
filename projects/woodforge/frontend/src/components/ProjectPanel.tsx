import { useRef } from 'react';
import { useStore } from '../store';
import {
  saveProject,
  loadProject,
  newProject,
  undo,
  redo,
  canUndo,
  canRedo,
} from '../wasm';

export function ProjectPanel() {
  const projectName = useStore((s) => s.projectName);
  const setProjectName = useStore((s) => s.setProjectName);
  const setSceneTree = useStore((s) => s.setSceneTree);
  const setStatusMessage = useStore((s) => s.setStatusMessage);
  const fileInputRef = useRef<HTMLInputElement>(null);

  const handleSave = () => {
    try {
      const json = saveProject(projectName, '');
      if (!json) {
        setStatusMessage('Failed to save project');
        return;
      }
      const blob = new Blob([json], { type: 'application/json' });
      const url = URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = `${projectName.replace(/\s+/g, '_')}.wfp`;
      a.click();
      URL.revokeObjectURL(url);
      setStatusMessage(`Saved "${projectName}"`);
    } catch (e) {
      setStatusMessage('Error saving project');
      console.error('Save failed:', e);
    }
  };

  const handleLoad = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (!file) return;

    const reader = new FileReader();
    reader.onload = () => {
      try {
        const json = reader.result as string;
        const snapshot = loadProject(json);
        if (snapshot) {
          setSceneTree(snapshot);
          // Extract project name from filename (strip .wfp extension)
          const name = file.name.replace(/\.wfp$/, '').replace(/_/g, ' ');
          setProjectName(name);
          setStatusMessage(`Loaded "${name}"`);
        } else {
          setStatusMessage('Failed to load project');
        }
      } catch (err) {
        setStatusMessage('Error loading project file');
        console.error('Load failed:', err);
      }
    };
    reader.readAsText(file);
    // Reset file input so same file can be re-loaded
    e.target.value = '';
  };

  const handleNew = () => {
    const snapshot = newProject();
    if (snapshot) {
      setSceneTree(snapshot);
    }
    setProjectName('Untitled Project');
    setStatusMessage('New project created');
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
    <div className="flex flex-col h-full bg-[#16213e] border-l border-gray-700">
      <div className="px-3 py-2 text-xs font-semibold text-gray-400 uppercase tracking-wider border-b border-gray-700">
        Project
      </div>
      <div className="flex-1 overflow-y-auto p-3">
        <div className="space-y-4">
          {/* Project Name */}
          <div>
            <div className="text-xs font-medium text-gray-400 mb-1.5">
              Project Name
            </div>
            <input
              type="text"
              value={projectName}
              onChange={(e) => setProjectName(e.target.value)}
              className="w-full bg-gray-800 border border-gray-600 rounded px-2 py-1 text-xs text-gray-200 focus:border-blue-500 focus:outline-none"
            />
          </div>

          {/* File Operations */}
          <div>
            <div className="text-xs font-medium text-gray-400 mb-1.5">
              File
            </div>
            <div className="space-y-1.5">
              <button
                onClick={handleNew}
                className="w-full px-3 py-1.5 text-xs rounded bg-gray-700/50 text-gray-300 border border-transparent hover:bg-gray-600/50 transition-colors text-left"
              >
                New Project
              </button>
              <button
                onClick={handleSave}
                className="w-full px-3 py-1.5 text-xs rounded bg-gray-700/50 text-gray-300 border border-transparent hover:bg-gray-600/50 transition-colors text-left"
              >
                Save Project (.wfp)
              </button>
              <button
                onClick={() => fileInputRef.current?.click()}
                className="w-full px-3 py-1.5 text-xs rounded bg-gray-700/50 text-gray-300 border border-transparent hover:bg-gray-600/50 transition-colors text-left"
              >
                Load Project (.wfp)
              </button>
              <input
                ref={fileInputRef}
                type="file"
                accept=".wfp,.json"
                onChange={handleLoad}
                className="hidden"
              />
            </div>
          </div>

          {/* Undo / Redo */}
          <div>
            <div className="text-xs font-medium text-gray-400 mb-1.5">
              History
            </div>
            <div className="flex gap-1.5">
              <button
                onClick={handleUndo}
                disabled={!canUndo()}
                className={`flex-1 px-3 py-1.5 text-xs rounded border transition-colors ${
                  canUndo()
                    ? 'bg-gray-700/50 text-gray-300 border-transparent hover:bg-gray-600/50'
                    : 'bg-gray-800/30 text-gray-600 border-transparent cursor-not-allowed'
                }`}
              >
                Undo
              </button>
              <button
                onClick={handleRedo}
                disabled={!canRedo()}
                className={`flex-1 px-3 py-1.5 text-xs rounded border transition-colors ${
                  canRedo()
                    ? 'bg-gray-700/50 text-gray-300 border-transparent hover:bg-gray-600/50'
                    : 'bg-gray-800/30 text-gray-600 border-transparent cursor-not-allowed'
                }`}
              >
                Redo
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
