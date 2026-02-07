import { useStore } from './store';
import { Toolbar } from './components/Toolbar';
import { SceneTree } from './components/SceneTree';
import { Viewport } from './components/Viewport';
import { PropertiesPanel } from './components/PropertiesPanel';
import { StatusBar } from './components/StatusBar';
import { LumberPicker } from './components/LumberPicker';
import { SpeciesPanel } from './components/SpeciesPanel';
import { JoineryPanel } from './components/JoineryPanel';
import { AnalysisPanel } from './components/AnalysisPanel';
import { OutputPanel } from './components/OutputPanel';
import { ProjectPanel } from './components/ProjectPanel';

function RightPanelContent() {
  const panel = useStore((s) => s.rightPanel);

  switch (panel) {
    case 'species':
      return <SpeciesPanel />;
    case 'joinery':
      return <JoineryPanel />;
    case 'analysis':
      return <AnalysisPanel />;
    case 'output':
      return <OutputPanel />;
    case 'project':
      return <ProjectPanel />;
    default:
      return <PropertiesPanel />;
  }
}

export function App() {
  return (
    <div className="flex flex-col h-full bg-[#1a1a2e]">
      {/* Toolbar */}
      <Toolbar />

      {/* Main content: 3-column layout */}
      <div className="flex flex-1 min-h-0">
        {/* Left panel: Scene tree */}
        <div className="w-60 flex-shrink-0">
          <SceneTree />
        </div>

        {/* Center: Viewport */}
        <Viewport />

        {/* Right panel: Dynamic based on tab */}
        <div className="w-72 flex-shrink-0">
          <RightPanelContent />
        </div>
      </div>

      {/* Status bar */}
      <StatusBar />

      {/* Modals */}
      <LumberPicker />
    </div>
  );
}
