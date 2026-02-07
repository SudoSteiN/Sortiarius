import { useState, useEffect, useCallback } from 'react';
import { useStore } from '../store';
import {
  getJointTypes,
  addJoint,
  removeJoint,
  getAllJoints,
} from '../wasm';
import type { JointInfo, SceneNodeSnapshot } from '../types/scene';

const FACE_OPTIONS = ['front', 'back', 'top', 'bottom', 'left', 'right'];

export function JoineryPanel() {
  const sceneTree = useStore((s) => s.sceneTree);
  const selectedNodeId = useStore((s) => s.selectedNodeId);
  const setSceneTree = useStore((s) => s.setSceneTree);

  const [jointTypes, setJointTypes] = useState<string[]>([]);
  const [joints, setJoints] = useState<JointInfo[]>([]);
  const [jointType, setJointType] = useState('');
  const [boardA, setBoardA] = useState('');
  const [boardB, setBoardB] = useState('');
  const [faceA, setFaceA] = useState('front');
  const [faceB, setFaceB] = useState('front');

  const boards: SceneNodeSnapshot[] = sceneTree?.nodes.filter((n) => n.has_mesh) ?? [];

  const refreshJoints = useCallback(() => {
    const all = getAllJoints();
    if (all) setJoints(all);
  }, []);

  useEffect(() => {
    const types = getJointTypes();
    if (types) {
      setJointTypes(types);
      if (types.length > 0) setJointType(types[0]);
    }
    refreshJoints();
  }, [refreshJoints]);

  useEffect(() => {
    if (selectedNodeId && !boardA) {
      setBoardA(selectedNodeId);
    }
  }, [selectedNodeId, boardA]);

  const handleAddJoint = () => {
    if (!jointType || !boardA || !boardB || boardA === boardB) return;
    const jointId = addJoint(jointType, boardA, boardB, faceA, faceB);
    if (jointId) {
      refreshJoints();
    }
  };

  const handleRemoveJoint = (jointId: string) => {
    const removed = removeJoint(jointId);
    if (removed) {
      refreshJoints();
    }
  };

  const getBoardLabel = (id: string): string => {
    const node = boards.find((n) => n.id === id);
    return node?.label ?? id.substring(0, 8);
  };

  return (
    <div className="flex flex-col h-full bg-[#16213e] border-l border-gray-700">
      <div className="px-3 py-2 text-xs font-semibold text-gray-400 uppercase tracking-wider border-b border-gray-700">
        Joinery
      </div>
      <div className="flex-1 overflow-y-auto p-3">
        <div className="space-y-4">
          {/* Add Joint */}
          <div>
            <div className="text-xs font-medium text-gray-400 mb-1.5">
              Add Joint
            </div>
            <div className="space-y-1.5">
              <div className="flex items-center gap-2">
                <span className="text-[10px] text-gray-400 w-10">Type</span>
                <select
                  value={jointType}
                  onChange={(e) => setJointType(e.target.value)}
                  className="flex-1 bg-gray-800 border border-gray-600 rounded px-2 py-0.5 text-xs text-gray-200 focus:border-blue-500 focus:outline-none"
                >
                  {jointTypes.map((t) => (
                    <option key={t} value={t}>
                      {t}
                    </option>
                  ))}
                </select>
              </div>
              <div className="flex items-center gap-2">
                <span className="text-[10px] text-gray-400 w-10">Board A</span>
                <select
                  value={boardA}
                  onChange={(e) => setBoardA(e.target.value)}
                  className="flex-1 bg-gray-800 border border-gray-600 rounded px-2 py-0.5 text-xs text-gray-200 focus:border-blue-500 focus:outline-none"
                >
                  <option value="">-- Select --</option>
                  {boards.map((b) => (
                    <option key={b.id} value={b.id}>
                      {b.label}
                    </option>
                  ))}
                </select>
              </div>
              <div className="flex items-center gap-2">
                <span className="text-[10px] text-gray-400 w-10">Face A</span>
                <select
                  value={faceA}
                  onChange={(e) => setFaceA(e.target.value)}
                  className="flex-1 bg-gray-800 border border-gray-600 rounded px-2 py-0.5 text-xs text-gray-200 focus:border-blue-500 focus:outline-none"
                >
                  {FACE_OPTIONS.map((f) => (
                    <option key={f} value={f}>
                      {f}
                    </option>
                  ))}
                </select>
              </div>
              <div className="flex items-center gap-2">
                <span className="text-[10px] text-gray-400 w-10">Board B</span>
                <select
                  value={boardB}
                  onChange={(e) => setBoardB(e.target.value)}
                  className="flex-1 bg-gray-800 border border-gray-600 rounded px-2 py-0.5 text-xs text-gray-200 focus:border-blue-500 focus:outline-none"
                >
                  <option value="">-- Select --</option>
                  {boards.map((b) => (
                    <option key={b.id} value={b.id}>
                      {b.label}
                    </option>
                  ))}
                </select>
              </div>
              <div className="flex items-center gap-2">
                <span className="text-[10px] text-gray-400 w-10">Face B</span>
                <select
                  value={faceB}
                  onChange={(e) => setFaceB(e.target.value)}
                  className="flex-1 bg-gray-800 border border-gray-600 rounded px-2 py-0.5 text-xs text-gray-200 focus:border-blue-500 focus:outline-none"
                >
                  {FACE_OPTIONS.map((f) => (
                    <option key={f} value={f}>
                      {f}
                    </option>
                  ))}
                </select>
              </div>
              <button
                onClick={handleAddJoint}
                disabled={!jointType || !boardA || !boardB || boardA === boardB}
                className="w-full bg-blue-600 hover:bg-blue-700 disabled:bg-gray-700 disabled:text-gray-500 text-white text-xs py-1 rounded transition-colors"
              >
                Add Joint
              </button>
            </div>
          </div>

          {/* Existing Joints */}
          <div>
            <div className="text-xs font-medium text-gray-400 mb-1.5">
              Joints ({joints.length})
            </div>
            {joints.length === 0 ? (
              <div className="text-[10px] text-gray-500 text-center py-2">
                No joints in project
              </div>
            ) : (
              <div className="space-y-1">
                {joints.map((j) => (
                  <div
                    key={j.id}
                    className="flex items-center gap-1.5 bg-gray-800/50 rounded px-2 py-1.5 group"
                  >
                    <div className="flex-1 min-w-0">
                      <div className="text-xs text-gray-200 truncate">
                        {j.joint_type}
                      </div>
                      <div className="text-[10px] text-gray-500 truncate">
                        {getBoardLabel(j.board_a)} ({j.face_a}) &mdash;{' '}
                        {getBoardLabel(j.board_b)} ({j.face_b})
                      </div>
                    </div>
                    <button
                      onClick={() => handleRemoveJoint(j.id)}
                      className="text-gray-600 hover:text-red-400 text-xs opacity-0 group-hover:opacity-100 transition-opacity flex-shrink-0"
                      title="Remove joint"
                    >
                      x
                    </button>
                  </div>
                ))}
              </div>
            )}
          </div>
        </div>
      </div>
    </div>
  );
}
