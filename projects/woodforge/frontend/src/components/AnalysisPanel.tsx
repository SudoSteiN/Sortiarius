import { useState } from 'react';
import { useStore } from '../store';
import { analyzeShelfDeflection, analyzeCompression, analyzeJointStrength, getAllSpecies } from '../wasm';
import type { DeflectionResult, CompressionResult, JointStrengthResult, WoodSpecies } from '../types/scene';

function StatusBadge({ status }: { status: 'Green' | 'Yellow' | 'Red' }) {
  const colors = {
    Green: 'bg-green-600/30 text-green-300 border-green-500/50',
    Yellow: 'bg-yellow-600/30 text-yellow-300 border-yellow-500/50',
    Red: 'bg-red-600/30 text-red-300 border-red-500/50',
  };
  return (
    <span className={`px-2 py-0.5 text-xs rounded border ${colors[status]}`}>
      {status}
    </span>
  );
}

function ShelfAnalysis() {
  const [span, setSpan] = useState(48);
  const [width, setWidth] = useState(11.25);
  const [height, setHeight] = useState(0.75);
  const [load, setLoad] = useState(100);
  const [speciesId, setSpeciesId] = useState('pine_sy');
  const [result, setResult] = useState<DeflectionResult | null>(null);

  const species: WoodSpecies[] = getAllSpecies() ?? [];

  const run = () => {
    const r = analyzeShelfDeflection(span, width, height, speciesId, load);
    if (r) setResult(r);
  };

  return (
    <div className="space-y-2">
      <div className="text-xs font-medium text-gray-300">Shelf Deflection</div>
      <div className="grid grid-cols-2 gap-1.5">
        <label className="text-xs text-gray-400">Span (in)</label>
        <input type="number" value={span} onChange={(e) => setSpan(+e.target.value)}
          className="bg-gray-800 border border-gray-600 rounded px-2 py-0.5 text-xs text-gray-200" />
        <label className="text-xs text-gray-400">Width (in)</label>
        <input type="number" value={width} onChange={(e) => setWidth(+e.target.value)}
          className="bg-gray-800 border border-gray-600 rounded px-2 py-0.5 text-xs text-gray-200" />
        <label className="text-xs text-gray-400">Thickness (in)</label>
        <input type="number" value={height} onChange={(e) => setHeight(+e.target.value)}
          className="bg-gray-800 border border-gray-600 rounded px-2 py-0.5 text-xs text-gray-200" />
        <label className="text-xs text-gray-400">Load (lbs)</label>
        <input type="number" value={load} onChange={(e) => setLoad(+e.target.value)}
          className="bg-gray-800 border border-gray-600 rounded px-2 py-0.5 text-xs text-gray-200" />
        <label className="text-xs text-gray-400">Species</label>
        <select value={speciesId} onChange={(e) => setSpeciesId(e.target.value)}
          className="bg-gray-800 border border-gray-600 rounded px-2 py-0.5 text-xs text-gray-200">
          {species.map((s) => <option key={s.id} value={s.id}>{s.name}</option>)}
        </select>
      </div>
      <button onClick={run}
        className="w-full px-2 py-1 text-xs bg-blue-600/30 text-blue-300 border border-blue-500/50 rounded hover:bg-blue-600/50">
        Analyze
      </button>
      {result && (
        <div className="space-y-1 p-2 bg-gray-800/50 rounded">
          <div className="flex justify-between items-center">
            <span className="text-xs text-gray-400">Status</span>
            <StatusBadge status={result.status} />
          </div>
          <div className="flex justify-between">
            <span className="text-xs text-gray-400">Deflection</span>
            <span className="text-xs text-gray-200">{result.deflection_inches.toFixed(3)}"</span>
          </div>
          <div className="flex justify-between">
            <span className="text-xs text-gray-400">Max allowable</span>
            <span className="text-xs text-gray-200">{result.max_allowable.toFixed(3)}"</span>
          </div>
          <div className="flex justify-between">
            <span className="text-xs text-gray-400">Span ratio</span>
            <span className="text-xs text-gray-200">{result.span_ratio}</span>
          </div>
          <div className="flex justify-between">
            <span className="text-xs text-gray-400">Utilization</span>
            <span className="text-xs text-gray-200">{(result.ratio * 100).toFixed(1)}%</span>
          </div>
        </div>
      )}
    </div>
  );
}

function CompressionAnalysis() {
  const [length, setLength] = useState(30);
  const [width, setWidth] = useState(3.5);
  const [height, setHeight] = useState(3.5);
  const [load, setLoad] = useState(200);
  const [speciesId, setSpeciesId] = useState('pine_sy');
  const [result, setResult] = useState<CompressionResult | null>(null);

  const species: WoodSpecies[] = getAllSpecies() ?? [];

  const run = () => {
    const r = analyzeCompression(length, width, height, speciesId, load);
    if (r) setResult(r);
  };

  return (
    <div className="space-y-2">
      <div className="text-xs font-medium text-gray-300">Compression (Leg/Post)</div>
      <div className="grid grid-cols-2 gap-1.5">
        <label className="text-xs text-gray-400">Length (in)</label>
        <input type="number" value={length} onChange={(e) => setLength(+e.target.value)}
          className="bg-gray-800 border border-gray-600 rounded px-2 py-0.5 text-xs text-gray-200" />
        <label className="text-xs text-gray-400">Width (in)</label>
        <input type="number" value={width} onChange={(e) => setWidth(+e.target.value)}
          className="bg-gray-800 border border-gray-600 rounded px-2 py-0.5 text-xs text-gray-200" />
        <label className="text-xs text-gray-400">Height (in)</label>
        <input type="number" value={height} onChange={(e) => setHeight(+e.target.value)}
          className="bg-gray-800 border border-gray-600 rounded px-2 py-0.5 text-xs text-gray-200" />
        <label className="text-xs text-gray-400">Load (lbs)</label>
        <input type="number" value={load} onChange={(e) => setLoad(+e.target.value)}
          className="bg-gray-800 border border-gray-600 rounded px-2 py-0.5 text-xs text-gray-200" />
        <label className="text-xs text-gray-400">Species</label>
        <select value={speciesId} onChange={(e) => setSpeciesId(e.target.value)}
          className="bg-gray-800 border border-gray-600 rounded px-2 py-0.5 text-xs text-gray-200">
          {species.map((s) => <option key={s.id} value={s.id}>{s.name}</option>)}
        </select>
      </div>
      <button onClick={run}
        className="w-full px-2 py-1 text-xs bg-blue-600/30 text-blue-300 border border-blue-500/50 rounded hover:bg-blue-600/50">
        Analyze
      </button>
      {result && (
        <div className="space-y-1 p-2 bg-gray-800/50 rounded">
          <div className="flex justify-between items-center">
            <span className="text-xs text-gray-400">Status</span>
            <StatusBadge status={result.status} />
          </div>
          <div className="flex justify-between">
            <span className="text-xs text-gray-400">Capacity</span>
            <span className="text-xs text-gray-200">{result.capacity_lbs.toFixed(0)} lbs</span>
          </div>
          <div className="flex justify-between">
            <span className="text-xs text-gray-400">Utilization</span>
            <span className="text-xs text-gray-200">{(result.utilization * 100).toFixed(1)}%</span>
          </div>
          {result.buckling_risk && (
            <div className="text-xs text-red-400">Warning: Buckling risk</div>
          )}
        </div>
      )}
    </div>
  );
}

function JointStrengthAnalysis() {
  const [jointType, setJointType] = useState('mortise_tenon');
  const [speciesId, setSpeciesId] = useState('pine_sy');
  const [result, setResult] = useState<JointStrengthResult | null>(null);

  const species: WoodSpecies[] = getAllSpecies() ?? [];
  const jointTypes = [
    ['butt', 'Butt'], ['miter', 'Miter'], ['pocket_hole', 'Pocket Hole'],
    ['dado', 'Dado'], ['rabbet', 'Rabbet'], ['half_lap', 'Half Lap'],
    ['mortise_tenon', 'Mortise & Tenon'], ['dovetail_through', 'Dovetail'],
    ['dowel', 'Dowel'], ['biscuit', 'Biscuit'],
  ];

  const run = () => {
    const r = analyzeJointStrength(jointType, speciesId);
    if (r) setResult(r);
  };

  return (
    <div className="space-y-2">
      <div className="text-xs font-medium text-gray-300">Joint Strength</div>
      <div className="grid grid-cols-2 gap-1.5">
        <label className="text-xs text-gray-400">Joint type</label>
        <select value={jointType} onChange={(e) => setJointType(e.target.value)}
          className="bg-gray-800 border border-gray-600 rounded px-2 py-0.5 text-xs text-gray-200">
          {jointTypes.map(([val, label]) => <option key={val} value={val}>{label}</option>)}
        </select>
        <label className="text-xs text-gray-400">Species</label>
        <select value={speciesId} onChange={(e) => setSpeciesId(e.target.value)}
          className="bg-gray-800 border border-gray-600 rounded px-2 py-0.5 text-xs text-gray-200">
          {species.map((s) => <option key={s.id} value={s.id}>{s.name}</option>)}
        </select>
      </div>
      <button onClick={run}
        className="w-full px-2 py-1 text-xs bg-blue-600/30 text-blue-300 border border-blue-500/50 rounded hover:bg-blue-600/50">
        Analyze
      </button>
      {result && (
        <div className="space-y-1 p-2 bg-gray-800/50 rounded">
          <div className="flex justify-between items-center">
            <span className="text-xs text-gray-400">Status</span>
            <StatusBadge status={result.status} />
          </div>
          <div className="flex justify-between">
            <span className="text-xs text-gray-400">Base strength</span>
            <span className="text-xs text-gray-200">{result.base_strength}/100</span>
          </div>
          <div className="flex justify-between">
            <span className="text-xs text-gray-400">Species factor</span>
            <span className="text-xs text-gray-200">{result.species_factor.toFixed(2)}x</span>
          </div>
          <div className="flex justify-between">
            <span className="text-xs text-gray-400">Adjusted</span>
            <span className="text-xs text-gray-200">{result.adjusted_strength}/100</span>
          </div>
          <div className="text-xs text-gray-400 mt-1">{result.recommendation}</div>
        </div>
      )}
    </div>
  );
}

export function AnalysisPanel() {
  const [tab, setTab] = useState<'shelf' | 'compression' | 'joint'>('shelf');

  const tabs: { id: typeof tab; label: string }[] = [
    { id: 'shelf', label: 'Shelf' },
    { id: 'compression', label: 'Post' },
    { id: 'joint', label: 'Joint' },
  ];

  return (
    <div className="flex flex-col h-full bg-[#16213e] border-l border-gray-700">
      <div className="px-3 py-2 text-xs font-semibold text-gray-400 uppercase tracking-wider border-b border-gray-700">
        Structural Analysis
      </div>
      <div className="flex border-b border-gray-700">
        {tabs.map((t) => (
          <button
            key={t.id}
            onClick={() => setTab(t.id)}
            className={`flex-1 px-2 py-1.5 text-xs transition-colors ${
              tab === t.id
                ? 'text-blue-300 border-b-2 border-blue-400'
                : 'text-gray-500 hover:text-gray-300'
            }`}
          >
            {t.label}
          </button>
        ))}
      </div>
      <div className="flex-1 overflow-y-auto p-3">
        {tab === 'shelf' && <ShelfAnalysis />}
        {tab === 'compression' && <CompressionAnalysis />}
        {tab === 'joint' && <JointStrengthAnalysis />}
      </div>
    </div>
  );
}
