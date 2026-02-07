import { useState, useEffect } from 'react';
import { useStore } from '../store';
import {
  getAllSpecies,
  setBoardSpecies,
  clearBoardSpecies,
  getBoardSpecies,
  applyStainFinish,
  applyPaintFinish,
  applyOilFinish,
  clearFinish,
} from '../wasm';
import type { WoodSpecies } from '../types/scene';

const OIL_PRESETS = ['Tung Oil', 'Danish Oil', 'Linseed Oil', 'Teak Oil'];
const SHEEN_OPTIONS = ['Flat', 'Satin', 'Semi-Gloss', 'Gloss'];

function hexToRgb(hex: string): [number, number, number] {
  const h = hex.replace('#', '');
  return [
    parseInt(h.substring(0, 2), 16),
    parseInt(h.substring(2, 4), 16),
    parseInt(h.substring(4, 6), 16),
  ];
}

function rgbToHex(r: number, g: number, b: number): string {
  return (
    '#' +
    [r, g, b].map((c) => Math.round(c).toString(16).padStart(2, '0')).join('')
  );
}

function SpeciesCard({
  species,
  selected,
  onSelect,
}: {
  species: WoodSpecies;
  selected: boolean;
  onSelect: () => void;
}) {
  return (
    <button
      onClick={onSelect}
      className={`w-full flex items-center gap-2 px-2 py-1.5 rounded text-left transition-colors ${
        selected
          ? 'bg-blue-600/30 border border-blue-500'
          : 'bg-gray-800/50 border border-transparent hover:bg-gray-700/50'
      }`}
    >
      <div
        className="w-4 h-4 rounded-sm border border-gray-600 flex-shrink-0"
        style={{
          backgroundColor: rgbToHex(
            species.color[0],
            species.color[1],
            species.color[2]
          ),
        }}
      />
      <div className="flex-1 min-w-0">
        <div className="text-xs text-gray-200 truncate">{species.name}</div>
        <div className="text-[10px] text-gray-500">{species.workability}</div>
      </div>
    </button>
  );
}

export function SpeciesPanel() {
  const selectedNodeId = useStore((s) => s.selectedNodeId);
  const setSceneTree = useStore((s) => s.setSceneTree);

  const [speciesList, setSpeciesList] = useState<WoodSpecies[]>([]);
  const [currentSpecies, setCurrentSpecies] = useState<WoodSpecies | null>(
    null
  );
  const [finishType, setFinishType] = useState<
    'none' | 'stain' | 'paint' | 'oil'
  >('none');
  const [stainColor, setStainColor] = useState('#8B4513');
  const [stainOpacity, setStainOpacity] = useState(0.5);
  const [paintColor, setPaintColor] = useState('#FFFFFF');
  const [paintSheen, setPaintSheen] = useState('Satin');
  const [oilPreset, setOilPreset] = useState('Tung Oil');

  useEffect(() => {
    const species = getAllSpecies();
    if (species) setSpeciesList(species);
  }, []);

  useEffect(() => {
    if (selectedNodeId) {
      const sp = getBoardSpecies(selectedNodeId);
      setCurrentSpecies(sp ?? null);
    } else {
      setCurrentSpecies(null);
    }
  }, [selectedNodeId]);

  const handleSelectSpecies = (species: WoodSpecies) => {
    if (!selectedNodeId) return;
    const snapshot = setBoardSpecies(selectedNodeId, species.id);
    if (snapshot) setSceneTree(snapshot);
    setCurrentSpecies(species);
  };

  const handleClearSpecies = () => {
    if (!selectedNodeId) return;
    clearBoardSpecies(selectedNodeId);
    setCurrentSpecies(null);
  };

  const handleApplyFinish = () => {
    if (!selectedNodeId) return;
    let snapshot: any = null;
    if (finishType === 'stain') {
      const [r, g, b] = hexToRgb(stainColor);
      snapshot = applyStainFinish(
        selectedNodeId,
        'Custom Stain',
        r,
        g,
        b,
        stainOpacity
      );
    } else if (finishType === 'paint') {
      const [r, g, b] = hexToRgb(paintColor);
      snapshot = applyPaintFinish(
        selectedNodeId,
        'Custom Paint',
        r,
        g,
        b,
        paintSheen
      );
    } else if (finishType === 'oil') {
      snapshot = applyOilFinish(selectedNodeId, oilPreset);
    }
    if (snapshot) setSceneTree(snapshot);
  };

  const handleClearFinish = () => {
    if (!selectedNodeId) return;
    clearFinish(selectedNodeId);
  };

  return (
    <div className="flex flex-col h-full bg-[#16213e] border-l border-gray-700">
      <div className="px-3 py-2 text-xs font-semibold text-gray-400 uppercase tracking-wider border-b border-gray-700">
        Species & Finish
      </div>
      <div className="flex-1 overflow-y-auto p-3">
        {!selectedNodeId ? (
          <div className="text-xs text-gray-500 text-center py-8">
            Select a board to assign species and finish
          </div>
        ) : (
          <div className="space-y-4">
            {/* Current Species Properties */}
            {currentSpecies && (
              <div>
                <div className="text-xs font-medium text-gray-400 mb-1.5">
                  Current Species
                </div>
                <div className="bg-gray-800/50 rounded p-2 space-y-1">
                  <div className="flex items-center gap-2">
                    <div
                      className="w-3 h-3 rounded-sm border border-gray-600"
                      style={{
                        backgroundColor: rgbToHex(
                          currentSpecies.color[0],
                          currentSpecies.color[1],
                          currentSpecies.color[2]
                        ),
                      }}
                    />
                    <span className="text-xs text-gray-200 font-medium">
                      {currentSpecies.name}
                    </span>
                  </div>
                  <div className="flex justify-between text-[10px]">
                    <span className="text-gray-500">Janka Hardness</span>
                    <span className="text-gray-300">
                      {currentSpecies.janka_hardness} lbf
                    </span>
                  </div>
                  <div className="flex justify-between text-[10px]">
                    <span className="text-gray-500">Density</span>
                    <span className="text-gray-300">
                      {currentSpecies.density_lb_ft3} lb/ft3
                    </span>
                  </div>
                  <div className="flex justify-between text-[10px]">
                    <span className="text-gray-500">MoE</span>
                    <span className="text-gray-300">
                      {(currentSpecies.modulus_of_elasticity / 1e6).toFixed(2)}{' '}
                      Mpsi
                    </span>
                  </div>
                  <div className="flex justify-between text-[10px]">
                    <span className="text-gray-500">Workability</span>
                    <span className="text-gray-300">
                      {currentSpecies.workability}
                    </span>
                  </div>
                </div>
                <button
                  onClick={handleClearSpecies}
                  className="mt-1.5 w-full text-[10px] text-red-400 hover:text-red-300 py-0.5"
                >
                  Clear Species
                </button>
              </div>
            )}

            {/* Species List */}
            <div>
              <div className="text-xs font-medium text-gray-400 mb-1.5">
                Wood Species
              </div>
              <div className="space-y-1 max-h-48 overflow-y-auto">
                {speciesList.map((sp) => (
                  <SpeciesCard
                    key={sp.id}
                    species={sp}
                    selected={currentSpecies?.id === sp.id}
                    onSelect={() => handleSelectSpecies(sp)}
                  />
                ))}
                {speciesList.length === 0 && (
                  <div className="text-[10px] text-gray-500 text-center py-2">
                    No species available
                  </div>
                )}
              </div>
            </div>

            {/* Finish Controls */}
            <div>
              <div className="text-xs font-medium text-gray-400 mb-1.5">
                Finish
              </div>
              <div className="flex gap-1 mb-2">
                {(
                  ['none', 'stain', 'paint', 'oil'] as const
                ).map((type) => (
                  <button
                    key={type}
                    onClick={() => setFinishType(type)}
                    className={`flex-1 text-[10px] py-1 rounded transition-colors ${
                      finishType === type
                        ? 'bg-blue-600 text-white'
                        : 'bg-gray-800 text-gray-400 hover:bg-gray-700'
                    }`}
                  >
                    {type.charAt(0).toUpperCase() + type.slice(1)}
                  </button>
                ))}
              </div>

              {finishType === 'stain' && (
                <div className="space-y-2">
                  <div className="flex items-center gap-2">
                    <span className="text-[10px] text-gray-400 w-10">
                      Color
                    </span>
                    <input
                      type="color"
                      value={stainColor}
                      onChange={(e) => setStainColor(e.target.value)}
                      className="w-6 h-6 rounded cursor-pointer border-0 bg-transparent"
                    />
                    <span className="text-[10px] text-gray-500">
                      {stainColor}
                    </span>
                  </div>
                  <div className="flex items-center gap-2">
                    <span className="text-[10px] text-gray-400 w-10">
                      Opacity
                    </span>
                    <input
                      type="range"
                      min="0"
                      max="1"
                      step="0.05"
                      value={stainOpacity}
                      onChange={(e) =>
                        setStainOpacity(parseFloat(e.target.value))
                      }
                      className="flex-1 h-1 accent-blue-500"
                    />
                    <span className="text-[10px] text-gray-400 w-8 text-right">
                      {Math.round(stainOpacity * 100)}%
                    </span>
                  </div>
                </div>
              )}

              {finishType === 'paint' && (
                <div className="space-y-2">
                  <div className="flex items-center gap-2">
                    <span className="text-[10px] text-gray-400 w-10">
                      Color
                    </span>
                    <input
                      type="color"
                      value={paintColor}
                      onChange={(e) => setPaintColor(e.target.value)}
                      className="w-6 h-6 rounded cursor-pointer border-0 bg-transparent"
                    />
                    <span className="text-[10px] text-gray-500">
                      {paintColor}
                    </span>
                  </div>
                  <div className="flex items-center gap-2">
                    <span className="text-[10px] text-gray-400 w-10">
                      Sheen
                    </span>
                    <select
                      value={paintSheen}
                      onChange={(e) => setPaintSheen(e.target.value)}
                      className="flex-1 bg-gray-800 border border-gray-600 rounded px-2 py-0.5 text-xs text-gray-200 focus:border-blue-500 focus:outline-none"
                    >
                      {SHEEN_OPTIONS.map((s) => (
                        <option key={s} value={s}>
                          {s}
                        </option>
                      ))}
                    </select>
                  </div>
                </div>
              )}

              {finishType === 'oil' && (
                <div className="flex items-center gap-2">
                  <span className="text-[10px] text-gray-400 w-10">Type</span>
                  <select
                    value={oilPreset}
                    onChange={(e) => setOilPreset(e.target.value)}
                    className="flex-1 bg-gray-800 border border-gray-600 rounded px-2 py-0.5 text-xs text-gray-200 focus:border-blue-500 focus:outline-none"
                  >
                    {OIL_PRESETS.map((o) => (
                      <option key={o} value={o}>
                        {o}
                      </option>
                    ))}
                  </select>
                </div>
              )}

              {finishType !== 'none' && (
                <div className="flex gap-1 mt-2">
                  <button
                    onClick={handleApplyFinish}
                    className="flex-1 bg-blue-600 hover:bg-blue-700 text-white text-[10px] py-1 rounded transition-colors"
                  >
                    Apply Finish
                  </button>
                  <button
                    onClick={handleClearFinish}
                    className="flex-1 bg-gray-700 hover:bg-gray-600 text-gray-300 text-[10px] py-1 rounded transition-colors"
                  >
                    Clear Finish
                  </button>
                </div>
              )}
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
