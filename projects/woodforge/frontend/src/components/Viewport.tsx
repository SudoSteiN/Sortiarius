import { useRef, useEffect, useCallback } from 'react';
import { useStore } from '../store';
import {
  initWasm,
  initRenderer,
  render,
  resize,
  cameraOrbit,
  cameraPan,
  cameraZoom,
  setPosition,
  computeSnapPosition,
  selectNode,
  deselectAll,
  pickObject,
} from '../wasm';

export function Viewport() {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const containerRef = useRef<HTMLDivElement>(null);
  const animRef = useRef<number>(0);
  const isDragging = useRef(false);
  const dragButton = useRef(-1);
  const lastMouse = useRef({ x: 0, y: 0 });
  const mouseDownPos = useRef({ x: 0, y: 0 });
  const mouseMoved = useRef(false);

  const activeTool = useStore((s) => s.activeTool);
  const selectedNodeId = useStore((s) => s.selectedNodeId);
  const setSelectedNodeId = useStore((s) => s.setSelectedNodeId);
  const setWasmReady = useStore((s) => s.setWasmReady);
  const setSceneTree = useStore((s) => s.setSceneTree);

  // Render loop
  const renderLoop = useCallback(() => {
    render();
    animRef.current = requestAnimationFrame(renderLoop);
  }, []);

  // Initialize WASM and renderer
  useEffect(() => {
    const canvas = canvasRef.current;
    const container = containerRef.current;
    if (!canvas || !container) return;

    let cancelled = false;

    const init = async () => {
      try {
        await initWasm();
        if (cancelled) return;

        const rect = container.getBoundingClientRect();
        const w = Math.max(rect.width, 1);
        const h = Math.max(rect.height, 1);

        canvas.width = w * devicePixelRatio;
        canvas.height = h * devicePixelRatio;
        canvas.style.width = `${w}px`;
        canvas.style.height = `${h}px`;

        await initRenderer(canvas, canvas.width, canvas.height);
        if (cancelled) return;

        setWasmReady(true);
        animRef.current = requestAnimationFrame(renderLoop);
      } catch (e) {
        console.error('Failed to initialize WoodForge:', e);
      }
    };

    init();

    return () => {
      cancelled = true;
      if (animRef.current) cancelAnimationFrame(animRef.current);
    };
  }, [renderLoop, setWasmReady]);

  // ResizeObserver
  useEffect(() => {
    const container = containerRef.current;
    const canvas = canvasRef.current;
    if (!container || !canvas) return;

    const observer = new ResizeObserver((entries) => {
      for (const entry of entries) {
        const { width, height } = entry.contentRect;
        const w = Math.max(width, 1);
        const h = Math.max(height, 1);
        canvas.width = w * devicePixelRatio;
        canvas.height = h * devicePixelRatio;
        canvas.style.width = `${w}px`;
        canvas.style.height = `${h}px`;
        resize(canvas.width, canvas.height);
      }
    });

    observer.observe(container);
    return () => observer.disconnect();
  }, []);

  // Mouse handlers
  const handleMouseDown = useCallback(
    (e: React.MouseEvent) => {
      isDragging.current = true;
      dragButton.current = e.button;
      lastMouse.current = { x: e.clientX, y: e.clientY };
      mouseDownPos.current = { x: e.clientX, y: e.clientY };
      mouseMoved.current = false;
      e.preventDefault();
    },
    []
  );

  const handleMouseMove = useCallback(
    (e: React.MouseEvent) => {
      if (!isDragging.current) return;

      const dx = e.clientX - lastMouse.current.x;
      const dy = e.clientY - lastMouse.current.y;
      lastMouse.current = { x: e.clientX, y: e.clientY };

      // Track if mouse has moved beyond a small threshold (3px)
      const totalDx = e.clientX - mouseDownPos.current.x;
      const totalDy = e.clientY - mouseDownPos.current.y;
      if (Math.abs(totalDx) > 3 || Math.abs(totalDy) > 3) {
        mouseMoved.current = true;
      }

      // Move tool: drag selected object
      if (activeTool === 'move' && selectedNodeId && dragButton.current === 0) {
        // Simple: move in XZ plane based on mouse delta
        const sensitivity = 0.05;
        const snap = computeSnapPosition(
          selectedNodeId,
          dx * sensitivity,
          0,
          -dy * sensitivity
        );
        if (snap) {
          const snapshot = setPosition(
            selectedNodeId,
            snap.snapped_position.x,
            snap.snapped_position.y,
            snap.snapped_position.z
          );
          if (snapshot) setSceneTree(snapshot);
        }
        return;
      }

      // Camera controls (always available with right-click or when not in move/rotate mode)
      if (dragButton.current === 2 || (dragButton.current === 0 && activeTool === 'select')) {
        // Orbit
        cameraOrbit(dx, dy);
      } else if (dragButton.current === 1 || (e.shiftKey && dragButton.current === 0)) {
        // Pan
        cameraPan(dx, dy);
      }
    },
    [activeTool, selectedNodeId, setSceneTree]
  );

  const handleMouseUp = useCallback(
    (e: React.MouseEvent) => {
      // Click-to-select: select tool, left button, no drag
      if (
        activeTool === 'select' &&
        dragButton.current === 0 &&
        !mouseMoved.current
      ) {
        const nodeId = pickObject(e.clientX, e.clientY);
        if (nodeId) {
          selectNode(nodeId);
          setSelectedNodeId(nodeId);
        } else {
          deselectAll();
          setSelectedNodeId(null);
        }
      }

      isDragging.current = false;
      dragButton.current = -1;
    },
    [activeTool, setSelectedNodeId]
  );

  const handleWheel = useCallback((e: React.WheelEvent) => {
    cameraZoom(-e.deltaY * 0.01);
    e.preventDefault();
  }, []);

  const handleContextMenu = useCallback((e: React.MouseEvent) => {
    e.preventDefault();
  }, []);

  return (
    <div ref={containerRef} className="relative flex-1 overflow-hidden">
      <canvas
        ref={canvasRef}
        onMouseDown={handleMouseDown}
        onMouseMove={handleMouseMove}
        onMouseUp={handleMouseUp}
        onMouseLeave={handleMouseUp}
        onWheel={handleWheel}
        onContextMenu={handleContextMenu}
        className="block"
      />
    </div>
  );
}
