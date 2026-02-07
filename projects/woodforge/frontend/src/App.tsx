import { Toolbar } from './components/Toolbar';
import { SceneTree } from './components/SceneTree';
import { Viewport } from './components/Viewport';
import { PropertiesPanel } from './components/PropertiesPanel';
import { StatusBar } from './components/StatusBar';
import { LumberPicker } from './components/LumberPicker';

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

        {/* Right panel: Properties */}
        <div className="w-72 flex-shrink-0">
          <PropertiesPanel />
        </div>
      </div>

      {/* Status bar */}
      <StatusBar />

      {/* Modals */}
      <LumberPicker />
    </div>
  );
}
