import { useState } from 'react';
import {
  generateCutList,
  generateMaterialList,
  estimateProjectCost,
  generateBuildInstructions,
  optimizeCutLayout,
  formatDim,
} from '../wasm';
import type {
  CutList,
  MaterialList,
  CostEstimate,
  BuildInstructions,
  OptimizationResult,
} from '../types/scene';

type OutputTab = 'cutlist' | 'materials' | 'cost' | 'instructions';

const TABS: { key: OutputTab; label: string }[] = [
  { key: 'cutlist', label: 'Cut List' },
  { key: 'materials', label: 'Materials' },
  { key: 'cost', label: 'Cost' },
  { key: 'instructions', label: 'Instructions' },
];

export function OutputPanel() {
  const [activeTab, setActiveTab] = useState<OutputTab>('cutlist');
  const [cutList, setCutList] = useState<CutList | null>(null);
  const [materialList, setMaterialList] = useState<MaterialList | null>(null);
  const [costEstimate, setCostEstimate] = useState<CostEstimate | null>(null);
  const [instructions, setInstructions] = useState<BuildInstructions | null>(
    null
  );
  const [optimization, setOptimization] = useState<OptimizationResult | null>(
    null
  );

  const handleGenerate = () => {
    if (activeTab === 'cutlist') {
      const result = generateCutList();
      if (result) setCutList(result);
    } else if (activeTab === 'materials') {
      const result = generateMaterialList();
      if (result) setMaterialList(result);
    } else if (activeTab === 'cost') {
      const result = estimateProjectCost();
      if (result) setCostEstimate(result);
    } else if (activeTab === 'instructions') {
      const result = generateBuildInstructions();
      if (result) setInstructions(result);
    }
  };

  const handleOptimize = () => {
    const result = optimizeCutLayout(0.125);
    if (result) setOptimization(result);
  };

  return (
    <div className="flex flex-col h-full bg-[#16213e] border-l border-gray-700">
      <div className="px-3 py-2 text-xs font-semibold text-gray-400 uppercase tracking-wider border-b border-gray-700">
        Output
      </div>

      {/* Tab Bar */}
      <div className="flex border-b border-gray-700">
        {TABS.map((tab) => (
          <button
            key={tab.key}
            onClick={() => setActiveTab(tab.key)}
            className={`flex-1 text-[10px] py-1.5 transition-colors ${
              activeTab === tab.key
                ? 'text-blue-400 border-b-2 border-blue-400 bg-gray-800/30'
                : 'text-gray-500 hover:text-gray-300'
            }`}
          >
            {tab.label}
          </button>
        ))}
      </div>

      <div className="flex-1 overflow-y-auto p-3">
        {/* Generate Button */}
        <div className="flex gap-1 mb-3">
          <button
            onClick={handleGenerate}
            className="flex-1 bg-blue-600 hover:bg-blue-700 text-white text-xs py-1 rounded transition-colors"
          >
            Generate
          </button>
          {activeTab === 'cutlist' && (
            <button
              onClick={handleOptimize}
              className="flex-1 bg-gray-700 hover:bg-gray-600 text-gray-300 text-xs py-1 rounded transition-colors"
            >
              Optimize Cuts
            </button>
          )}
        </div>

        {/* Cut List Tab */}
        {activeTab === 'cutlist' && (
          <div>
            {cutList ? (
              <div className="space-y-2">
                <div className="text-[10px] text-gray-500">
                  {cutList.total_pieces} pieces, {cutList.total_cuts} cuts
                </div>
                <div className="overflow-x-auto">
                  <table className="w-full text-[10px]">
                    <thead>
                      <tr className="text-gray-400 border-b border-gray-700">
                        <th className="text-left py-1 pr-2">ID</th>
                        <th className="text-left py-1 pr-2">Label</th>
                        <th className="text-left py-1 pr-2">Material</th>
                        <th className="text-right py-1 pr-2">L</th>
                        <th className="text-right py-1 pr-2">W</th>
                        <th className="text-right py-1 pr-2">T</th>
                        <th className="text-right py-1">Qty</th>
                      </tr>
                    </thead>
                    <tbody>
                      {cutList.entries.map((entry) => (
                        <tr
                          key={entry.piece_id}
                          className="text-gray-300 border-b border-gray-800"
                        >
                          <td className="py-1 pr-2 text-gray-500">
                            {entry.piece_id.substring(0, 6)}
                          </td>
                          <td className="py-1 pr-2">{entry.label}</td>
                          <td className="py-1 pr-2 text-gray-400">
                            {entry.material}
                          </td>
                          <td className="py-1 pr-2 text-right">
                            {formatDim(entry.length)}
                          </td>
                          <td className="py-1 pr-2 text-right">
                            {formatDim(entry.width)}
                          </td>
                          <td className="py-1 pr-2 text-right">
                            {formatDim(entry.thickness)}
                          </td>
                          <td className="py-1 text-right">
                            {entry.quantity}
                          </td>
                        </tr>
                      ))}
                    </tbody>
                  </table>
                </div>

                {/* Optimization Results */}
                {optimization && (
                  <div className="mt-3 pt-3 border-t border-gray-700">
                    <div className="text-xs font-medium text-gray-400 mb-1.5">
                      Cut Optimization
                    </div>
                    <div className="space-y-1 text-[10px]">
                      <div className="flex justify-between">
                        <span className="text-gray-500">Stock boards</span>
                        <span className="text-gray-300">
                          {optimization.total_stock_boards}
                        </span>
                      </div>
                      <div className="flex justify-between">
                        <span className="text-gray-500">Waste</span>
                        <span className="text-gray-300">
                          {formatDim(optimization.total_waste_inches)} (
                          {optimization.total_waste_percent.toFixed(1)}%)
                        </span>
                      </div>
                      <div className="flex justify-between">
                        <span className="text-gray-500">Kerf width</span>
                        <span className="text-gray-300">
                          {formatDim(optimization.kerf_width)}
                        </span>
                      </div>
                      {optimization.unplaceable.length > 0 && (
                        <div className="mt-1 text-red-400">
                          {optimization.unplaceable.length} piece(s) could not
                          be placed
                        </div>
                      )}
                    </div>
                  </div>
                )}
              </div>
            ) : (
              <div className="text-[10px] text-gray-500 text-center py-4">
                Click Generate to create cut list
              </div>
            )}
          </div>
        )}

        {/* Materials Tab */}
        {activeTab === 'materials' && (
          <div>
            {materialList ? (
              <div className="space-y-2">
                <div className="overflow-x-auto">
                  <table className="w-full text-[10px]">
                    <thead>
                      <tr className="text-gray-400 border-b border-gray-700">
                        <th className="text-left py-1 pr-2">Material</th>
                        <th className="text-left py-1 pr-2">Size</th>
                        <th className="text-right py-1 pr-2">Length</th>
                        <th className="text-right py-1 pr-2">Qty</th>
                        <th className="text-right py-1">BF</th>
                      </tr>
                    </thead>
                    <tbody>
                      {materialList.entries.map((entry, i) => (
                        <tr
                          key={i}
                          className="text-gray-300 border-b border-gray-800"
                        >
                          <td className="py-1 pr-2">{entry.material}</td>
                          <td className="py-1 pr-2 text-gray-400">
                            {entry.nominal_size}
                          </td>
                          <td className="py-1 pr-2 text-right">
                            {formatDim(entry.length)}
                          </td>
                          <td className="py-1 pr-2 text-right">
                            {entry.quantity}
                          </td>
                          <td className="py-1 text-right">
                            {entry.board_feet.toFixed(2)}
                          </td>
                        </tr>
                      ))}
                    </tbody>
                  </table>
                </div>
                <div className="flex justify-between text-xs pt-2 border-t border-gray-700">
                  <span className="text-gray-400 font-medium">
                    Total Board Feet
                  </span>
                  <span className="text-gray-200 font-medium">
                    {materialList.total_board_feet.toFixed(2)}
                  </span>
                </div>
              </div>
            ) : (
              <div className="text-[10px] text-gray-500 text-center py-4">
                Click Generate to create material list
              </div>
            )}
          </div>
        )}

        {/* Cost Tab */}
        {activeTab === 'cost' && (
          <div>
            {costEstimate ? (
              <div className="space-y-3">
                {/* Material Costs */}
                {costEstimate.materials.length > 0 && (
                  <div>
                    <div className="text-[10px] font-medium text-gray-400 mb-1">
                      Materials
                    </div>
                    {costEstimate.materials.map((m, i) => (
                      <div
                        key={i}
                        className="flex justify-between text-[10px] py-0.5"
                      >
                        <span className="text-gray-300">
                          {m.species} ({m.board_feet.toFixed(1)} BF @
                          ${m.price_per_bf.toFixed(2)}/BF)
                        </span>
                        <span className="text-gray-200">
                          ${m.subtotal.toFixed(2)}
                        </span>
                      </div>
                    ))}
                    <div className="flex justify-between text-[10px] pt-1 border-t border-gray-800">
                      <span className="text-gray-400">Subtotal</span>
                      <span className="text-gray-300">
                        ${costEstimate.material_subtotal.toFixed(2)}
                      </span>
                    </div>
                  </div>
                )}

                {/* Hardware Costs */}
                {costEstimate.hardware.length > 0 && (
                  <div>
                    <div className="text-[10px] font-medium text-gray-400 mb-1">
                      Hardware
                    </div>
                    {costEstimate.hardware.map((h, i) => (
                      <div
                        key={i}
                        className="flex justify-between text-[10px] py-0.5"
                      >
                        <span className="text-gray-300">
                          {h.item} x{h.quantity}
                        </span>
                        <span className="text-gray-200">
                          ${h.subtotal.toFixed(2)}
                        </span>
                      </div>
                    ))}
                    <div className="flex justify-between text-[10px] pt-1 border-t border-gray-800">
                      <span className="text-gray-400">Subtotal</span>
                      <span className="text-gray-300">
                        ${costEstimate.hardware_subtotal.toFixed(2)}
                      </span>
                    </div>
                  </div>
                )}

                {/* Waste & Total */}
                <div className="pt-2 border-t border-gray-700 space-y-1">
                  <div className="flex justify-between text-[10px]">
                    <span className="text-gray-500">
                      Waste ({(costEstimate.waste_factor * 100).toFixed(0)}%)
                    </span>
                    <span className="text-gray-400">
                      ${costEstimate.waste_cost.toFixed(2)}
                    </span>
                  </div>
                  <div className="flex justify-between text-xs font-medium">
                    <span className="text-gray-300">Total</span>
                    <span className="text-gray-100">
                      ${costEstimate.total.toFixed(2)}
                    </span>
                  </div>
                </div>

                {/* Notes */}
                {costEstimate.notes.length > 0 && (
                  <div className="pt-2 border-t border-gray-700">
                    <div className="text-[10px] font-medium text-gray-400 mb-1">
                      Notes
                    </div>
                    {costEstimate.notes.map((note, i) => (
                      <div key={i} className="text-[10px] text-gray-500">
                        {note}
                      </div>
                    ))}
                  </div>
                )}
              </div>
            ) : (
              <div className="text-[10px] text-gray-500 text-center py-4">
                Click Generate to estimate project cost
              </div>
            )}
          </div>
        )}

        {/* Instructions Tab */}
        {activeTab === 'instructions' && (
          <div>
            {instructions ? (
              <div className="space-y-2">
                {instructions.steps.map((step) => (
                  <div
                    key={step.step_number}
                    className="bg-gray-800/50 rounded p-2"
                  >
                    <div className="flex items-start gap-2">
                      <span className="text-[10px] text-blue-400 font-medium flex-shrink-0">
                        {step.step_number}.
                      </span>
                      <div className="flex-1 min-w-0">
                        <div className="text-xs text-gray-200">
                          {step.description}
                        </div>
                        {step.tools_needed.length > 0 && (
                          <div className="text-[10px] text-gray-500 mt-0.5">
                            Tools: {step.tools_needed.join(', ')}
                          </div>
                        )}
                      </div>
                    </div>
                  </div>
                ))}
              </div>
            ) : (
              <div className="text-[10px] text-gray-500 text-center py-4">
                Click Generate to create build instructions
              </div>
            )}
          </div>
        )}
      </div>
    </div>
  );
}
