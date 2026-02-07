use std::cell::RefCell;

use wasm_bindgen::prelude::*;

mod scene_api;
mod lumber_api;

use woodforge_core::{
    lumber::LumberCatalog,
    scene::SceneGraph,
    snap::SnapConfig,
    units::UnitSystem,
};

pub struct AppState {
    pub renderer: Option<woodforge_renderer::Renderer>,
    pub scene: SceneGraph,
    pub catalog: LumberCatalog,
    pub snap_config: SnapConfig,
    pub unit_system: UnitSystem,
}

thread_local! {
    pub static APP: RefCell<AppState> = RefCell::new(AppState {
        renderer: None,
        scene: SceneGraph::new(),
        catalog: LumberCatalog::new(),
        snap_config: SnapConfig::default(),
        unit_system: UnitSystem::Imperial,
    });
}

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Info).ok();
    log::info!("WoodForge WASM bridge initialized");
}

#[wasm_bindgen]
pub async fn init_renderer(canvas: web_sys::HtmlCanvasElement, width: u32, height: u32) -> Result<(), JsValue> {
    let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
        backends: wgpu::Backends::BROWSER_WEBGPU | wgpu::Backends::GL,
        ..Default::default()
    });

    let surface = instance
        .create_surface(wgpu::SurfaceTarget::Canvas(canvas))
        .map_err(|e| JsValue::from_str(&format!("Failed to create surface: {}", e)))?;

    let renderer = woodforge_renderer::Renderer::new(instance, surface, width, height).await;
    APP.with(|app| {
        app.borrow_mut().renderer = Some(renderer);
    });
    log::info!("Renderer initialized ({}x{})", width, height);
    Ok(())
}

#[wasm_bindgen]
pub fn resize(width: u32, height: u32) {
    APP.with(|app| {
        let mut state = app.borrow_mut();
        if let Some(ref mut renderer) = state.renderer {
            renderer.resize(width, height);
        }
    });
}

#[wasm_bindgen]
pub fn render() {
    APP.with(|app| {
        let mut state = app.borrow_mut();
        if let Some(ref mut renderer) = state.renderer {
            renderer.render();
        }
    });
}

#[wasm_bindgen]
pub fn camera_orbit(dx: f32, dy: f32) {
    APP.with(|app| {
        let mut state = app.borrow_mut();
        if let Some(ref mut renderer) = state.renderer {
            renderer.camera_orbit(dx, dy);
        }
    });
}

#[wasm_bindgen]
pub fn camera_pan(dx: f32, dy: f32) {
    APP.with(|app| {
        let mut state = app.borrow_mut();
        if let Some(ref mut renderer) = state.renderer {
            renderer.camera_pan(dx, dy);
        }
    });
}

#[wasm_bindgen]
pub fn camera_zoom(delta: f32) {
    APP.with(|app| {
        let mut state = app.borrow_mut();
        if let Some(ref mut renderer) = state.renderer {
            renderer.camera_zoom(delta);
        }
    });
}
