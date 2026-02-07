import { useStore } from '../store';
import { setUnitSystem, setGridSnap } from '../wasm';

export function StatusBar() {
  const sceneTree = useStore((s) => s.sceneTree);
  const unitSystem = useStore((s) => s.unitSystem);
  const gridSnap = useStore((s) => s.gridSnap);
  const wasmReady = useStore((s) => s.wasmReady);
  const statusMessage = useStore((s) => s.statusMessage);
  const store = useStore();

  const nodeCount = sceneTree?.nodes.length ?? 0;

  const handleUnitToggle = () => {
    const newSystem = unitSystem === 'imperial' ? 'metric' : 'imperial';
    store.setUnitSystem(newSystem);
    setUnitSystem(newSystem);
  };

  const handleGridToggle = () => {
    store.toggleGridSnap();
    setGridSnap(!gridSnap);
  };

  return (
    <div className="flex items-center justify-between px-3 py-1 bg-[#0f0f23] border-t border-gray-700 text-xs">
      <div className="flex items-center gap-3 text-gray-400">
        <span>
          {wasmReady ? (
            `${nodeCount} object${nodeCount !== 1 ? 's' : ''}`
          ) : (
            'Loading WASM...'
          )}
        </span>
        {statusMessage && (
          <span className="text-gray-500 ml-2">{statusMessage}</span>
        )}
      </div>
      <div className="flex items-center gap-2">
        <button
          onClick={handleUnitToggle}
          className="px-2 py-0.5 rounded bg-gray-700/50 text-gray-300 hover:bg-gray-600/50 transition-colors"
        >
          {unitSystem === 'imperial' ? 'Imperial' : 'Metric'}
        </button>
        <button
          onClick={handleGridToggle}
          className={`px-2 py-0.5 rounded transition-colors ${
            gridSnap
              ? 'bg-blue-600/30 text-blue-300'
              : 'bg-gray-700/50 text-gray-500'
          }`}
        >
          Grid {gridSnap ? 'ON' : 'OFF'}
        </button>
      </div>
    </div>
  );
}
