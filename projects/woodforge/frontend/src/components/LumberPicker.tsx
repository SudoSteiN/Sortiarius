import { useState, useEffect } from 'react';
import { useStore } from '../store';
import { getLumberCatalog, addStandardBoard, addCustomBoard, formatDim } from '../wasm';
import type { LumberCatalog } from '../types/scene';

type Tab = 'dimensional' | 'board' | 'sheet' | 'custom';

export function LumberPicker() {
  const isOpen = useStore((s) => s.lumberPickerOpen);
  const setOpen = useStore((s) => s.setLumberPickerOpen);
  const setSceneTree = useStore((s) => s.setSceneTree);

  const [catalog, setCatalog] = useState<LumberCatalog | null>(null);
  const [activeTab, setActiveTab] = useState<Tab>('dimensional');
  const [customWidth, setCustomWidth] = useState('1.5');
  const [customHeight, setCustomHeight] = useState('3.5');
  const [customDepth, setCustomDepth] = useState('96');

  useEffect(() => {
    if (isOpen && !catalog) {
      const data = getLumberCatalog();
      if (data) setCatalog(data);
    }
  }, [isOpen, catalog]);

  if (!isOpen) return null;

  const handleAddStandard = (label: string) => {
    const snapshot = addStandardBoard(label);
    if (snapshot) setSceneTree(snapshot);
    setOpen(false);
  };

  const handleAddCustom = () => {
    const w = parseFloat(customWidth);
    const h = parseFloat(customHeight);
    const d = parseFloat(customDepth);
    if (isNaN(w) || isNaN(h) || isNaN(d) || w <= 0 || h <= 0 || d <= 0) return;
    const snapshot = addCustomBoard(w, h, d);
    if (snapshot) setSceneTree(snapshot);
    setOpen(false);
  };

  const dimensionalSizes = catalog?.sizes.filter((s) => s.category === 'Dimensional') ?? [];
  const boardSizes = catalog?.sizes.filter((s) => s.category === 'Board') ?? [];

  const tabs: { id: Tab; label: string }[] = [
    { id: 'dimensional', label: 'Dimensional' },
    { id: 'board', label: 'Board' },
    { id: 'sheet', label: 'Sheet' },
    { id: 'custom', label: 'Custom' },
  ];

  return (
    <div
      className="fixed inset-0 z-50 flex items-center justify-center bg-black/50"
      onClick={() => setOpen(false)}
    >
      <div
        className="bg-[#16213e] rounded-lg border border-gray-600 shadow-2xl w-[420px] max-h-[500px] flex flex-col"
        onClick={(e) => e.stopPropagation()}
      >
        {/* Header */}
        <div className="flex items-center justify-between px-4 py-3 border-b border-gray-700">
          <h2 className="text-sm font-semibold text-gray-200">Add Board</h2>
          <button
            onClick={() => setOpen(false)}
            className="text-gray-400 hover:text-gray-200 text-lg leading-none"
          >
            &times;
          </button>
        </div>

        {/* Tabs */}
        <div className="flex border-b border-gray-700">
          {tabs.map((tab) => (
            <button
              key={tab.id}
              onClick={() => setActiveTab(tab.id)}
              className={`flex-1 px-3 py-2 text-xs transition-colors ${
                activeTab === tab.id
                  ? 'text-blue-300 border-b-2 border-blue-500'
                  : 'text-gray-400 hover:text-gray-200'
              }`}
            >
              {tab.label}
            </button>
          ))}
        </div>

        {/* Content */}
        <div className="flex-1 overflow-y-auto p-3">
          {activeTab === 'dimensional' && (
            <div className="grid grid-cols-2 gap-1.5">
              {dimensionalSizes.map((size) => (
                <button
                  key={size.nominal_label}
                  onClick={() => handleAddStandard(size.nominal_label)}
                  className="flex flex-col items-start p-2 rounded bg-gray-700/30 hover:bg-gray-600/50 border border-transparent hover:border-blue-500/30 transition-colors"
                >
                  <span className="text-sm font-medium text-gray-200">
                    {size.nominal_label}
                  </span>
                  <span className="text-xs text-gray-400">
                    {formatDim(size.actual_width)} x {formatDim(size.actual_height)}
                  </span>
                </button>
              ))}
            </div>
          )}

          {activeTab === 'board' && (
            <div className="grid grid-cols-2 gap-1.5">
              {boardSizes.map((size) => (
                <button
                  key={size.nominal_label}
                  onClick={() => handleAddStandard(size.nominal_label)}
                  className="flex flex-col items-start p-2 rounded bg-gray-700/30 hover:bg-gray-600/50 border border-transparent hover:border-blue-500/30 transition-colors"
                >
                  <span className="text-sm font-medium text-gray-200">
                    {size.nominal_label}
                  </span>
                  <span className="text-xs text-gray-400">
                    {formatDim(size.actual_width)} x {formatDim(size.actual_height)}
                  </span>
                </button>
              ))}
            </div>
          )}

          {activeTab === 'sheet' && (
            <div className="space-y-1.5">
              {catalog?.sheets.map((sheet) => (
                <button
                  key={sheet.label}
                  onClick={() =>
                    addCustomBoard(sheet.thickness, sheet.width, sheet.height) &&
                    setOpen(false)
                  }
                  className="w-full flex flex-col items-start p-2 rounded bg-gray-700/30 hover:bg-gray-600/50 border border-transparent hover:border-blue-500/30 transition-colors"
                >
                  <span className="text-sm font-medium text-gray-200">
                    {sheet.label}
                  </span>
                  <span className="text-xs text-gray-400">
                    {formatDim(sheet.width)} x {formatDim(sheet.height)} x{' '}
                    {formatDim(sheet.thickness)}
                  </span>
                </button>
              ))}
            </div>
          )}

          {activeTab === 'custom' && (
            <div className="space-y-3">
              <p className="text-xs text-gray-400">
                Enter dimensions in inches:
              </p>
              <div className="space-y-2">
                <div className="flex items-center gap-2">
                  <label className="text-xs text-gray-400 w-14">Width</label>
                  <input
                    type="number"
                    value={customWidth}
                    onChange={(e) => setCustomWidth(e.target.value)}
                    className="flex-1 bg-gray-800 border border-gray-600 rounded px-2 py-1 text-sm text-gray-200 focus:border-blue-500 focus:outline-none"
                    step="0.25"
                    min="0.1"
                  />
                </div>
                <div className="flex items-center gap-2">
                  <label className="text-xs text-gray-400 w-14">Height</label>
                  <input
                    type="number"
                    value={customHeight}
                    onChange={(e) => setCustomHeight(e.target.value)}
                    className="flex-1 bg-gray-800 border border-gray-600 rounded px-2 py-1 text-sm text-gray-200 focus:border-blue-500 focus:outline-none"
                    step="0.25"
                    min="0.1"
                  />
                </div>
                <div className="flex items-center gap-2">
                  <label className="text-xs text-gray-400 w-14">Depth</label>
                  <input
                    type="number"
                    value={customDepth}
                    onChange={(e) => setCustomDepth(e.target.value)}
                    className="flex-1 bg-gray-800 border border-gray-600 rounded px-2 py-1 text-sm text-gray-200 focus:border-blue-500 focus:outline-none"
                    step="1"
                    min="0.1"
                  />
                </div>
              </div>
              <button
                onClick={handleAddCustom}
                className="w-full py-2 rounded bg-blue-600 hover:bg-blue-500 text-white text-sm font-medium transition-colors"
              >
                Add Custom Board
              </button>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
