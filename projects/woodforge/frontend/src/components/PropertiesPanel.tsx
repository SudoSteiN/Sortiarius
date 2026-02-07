import { useState, useEffect } from 'react';
import { useStore } from '../store';
import { setPosition, setRotation, formatDim } from '../wasm';
import type { SceneNodeSnapshot } from '../types/scene';

function NumberInput({
  label,
  value,
  onChange,
}: {
  label: string;
  value: number;
  onChange: (v: number) => void;
}) {
  const [text, setText] = useState(value.toFixed(2));

  useEffect(() => {
    setText(value.toFixed(2));
  }, [value]);

  const handleBlur = () => {
    const parsed = parseFloat(text);
    if (!isNaN(parsed)) {
      onChange(parsed);
    } else {
      setText(value.toFixed(2));
    }
  };

  return (
    <div className="flex items-center gap-2">
      <span className="text-xs text-gray-400 w-4">{label}</span>
      <input
        type="text"
        value={text}
        onChange={(e) => setText(e.target.value)}
        onBlur={handleBlur}
        onKeyDown={(e) => e.key === 'Enter' && handleBlur()}
        className="flex-1 bg-gray-800 border border-gray-600 rounded px-2 py-0.5 text-xs text-gray-200 focus:border-blue-500 focus:outline-none"
      />
    </div>
  );
}

function DimensionRow({ label, value }: { label: string; value: number }) {
  return (
    <div className="flex items-center justify-between">
      <span className="text-xs text-gray-400">{label}</span>
      <span className="text-xs text-gray-200">{formatDim(value)}</span>
    </div>
  );
}

export function PropertiesPanel() {
  const sceneTree = useStore((s) => s.sceneTree);
  const selectedNodeId = useStore((s) => s.selectedNodeId);
  const setSceneTree = useStore((s) => s.setSceneTree);

  const node: SceneNodeSnapshot | undefined = sceneTree?.nodes.find(
    (n) => n.id === selectedNodeId
  );

  const handlePositionChange = (axis: number, value: number) => {
    if (!node) return;
    const pos = [...node.position] as [number, number, number];
    pos[axis] = value;
    const snapshot = setPosition(node.id, pos[0], pos[1], pos[2]);
    if (snapshot) setSceneTree(snapshot);
  };

  const handleRotationChange = (axis: number, value: number) => {
    if (!node) return;
    const rot = [...node.rotation] as [number, number, number];
    rot[axis] = (value * Math.PI) / 180; // Convert degrees to radians
    const snapshot = setRotation(node.id, rot[0], rot[1], rot[2]);
    if (snapshot) setSceneTree(snapshot);
  };

  return (
    <div className="flex flex-col h-full bg-[#16213e] border-l border-gray-700">
      <div className="px-3 py-2 text-xs font-semibold text-gray-400 uppercase tracking-wider border-b border-gray-700">
        Properties
      </div>
      <div className="flex-1 overflow-y-auto p-3">
        {!node ? (
          <div className="text-xs text-gray-500 text-center py-8">
            Select an object to view properties
          </div>
        ) : (
          <div className="space-y-4">
            {/* Label */}
            <div>
              <div className="text-xs font-medium text-gray-300 mb-1">
                {node.label}
              </div>
              {node.lumber_type && (
                <div className="text-xs text-gray-500">
                  {node.lumber_type === 'Custom' ? 'Custom board' : `Standard ${node.lumber_type}`}
                </div>
              )}
            </div>

            {/* Dimensions */}
            {node.dimensions && (
              <div>
                <div className="text-xs font-medium text-gray-400 mb-1.5">
                  Dimensions
                </div>
                <div className="space-y-1">
                  <DimensionRow label="W" value={node.dimensions[0]} />
                  <DimensionRow label="H" value={node.dimensions[1]} />
                  <DimensionRow label="D" value={node.dimensions[2]} />
                </div>
              </div>
            )}

            {/* Position */}
            <div>
              <div className="text-xs font-medium text-gray-400 mb-1.5">
                Position (inches)
              </div>
              <div className="space-y-1">
                <NumberInput
                  label="X"
                  value={node.position[0]}
                  onChange={(v) => handlePositionChange(0, v)}
                />
                <NumberInput
                  label="Y"
                  value={node.position[1]}
                  onChange={(v) => handlePositionChange(1, v)}
                />
                <NumberInput
                  label="Z"
                  value={node.position[2]}
                  onChange={(v) => handlePositionChange(2, v)}
                />
              </div>
            </div>

            {/* Rotation */}
            <div>
              <div className="text-xs font-medium text-gray-400 mb-1.5">
                Rotation (degrees)
              </div>
              <div className="space-y-1">
                <NumberInput
                  label="X"
                  value={(node.rotation[0] * 180) / Math.PI}
                  onChange={(v) => handleRotationChange(0, v)}
                />
                <NumberInput
                  label="Y"
                  value={(node.rotation[1] * 180) / Math.PI}
                  onChange={(v) => handleRotationChange(1, v)}
                />
                <NumberInput
                  label="Z"
                  value={(node.rotation[2] * 180) / Math.PI}
                  onChange={(v) => handleRotationChange(2, v)}
                />
              </div>
            </div>

            {/* Grain direction */}
            {node.grain_direction && (
              <div>
                <div className="text-xs font-medium text-gray-400 mb-1">
                  Grain
                </div>
                <div className="text-xs text-gray-300">
                  {node.grain_direction}
                </div>
              </div>
            )}
          </div>
        )}
      </div>
    </div>
  );
}
