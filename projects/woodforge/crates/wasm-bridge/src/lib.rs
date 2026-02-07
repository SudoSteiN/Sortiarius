use std::cell::RefCell;

use wasm_bindgen::prelude::*;

mod scene_api;
mod lumber_api;
mod species_api;
mod joinery_api;
mod analysis_api;
mod project_api;

use woodforge_core::{
    cost::PriceDatabase,
    history::History,
    joinery::JointStore,
    lumber::LumberCatalog,
    scene::SceneGraph,
    snap::SnapConfig,
    species::SpeciesDatabase,
    units::UnitSystem,
};

pub struct AppState {
    pub renderer: Option<woodforge_renderer::Renderer>,
    pub scene: SceneGraph,
    pub catalog: LumberCatalog,
    pub snap_config: SnapConfig,
    pub unit_system: UnitSystem,
    pub species_db: SpeciesDatabase,
    pub joint_store: JointStore,
    pub history: History,
    pub price_db: PriceDatabase,
    /// Per-board species assignment: NodeId string -> species_id
    pub board_species: std::collections::HashMap<String, String>,
    /// Per-board finish assignment: NodeId string -> finish JSON
    pub board_finishes: std::collections::HashMap<String, String>,
}

thread_local! {
    pub static APP: RefCell<AppState> = RefCell::new(AppState {
        renderer: None,
        scene: SceneGraph::new(),
        catalog: LumberCatalog::new(),
        snap_config: SnapConfig::default(),
        unit_system: UnitSystem::Imperial,
        species_db: SpeciesDatabase::new(),
        joint_store: JointStore::new(),
        history: History::new(),
        price_db: PriceDatabase::new(),
        board_species: std::collections::HashMap::new(),
        board_finishes: std::collections::HashMap::new(),
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

    #[cfg(target_arch = "wasm32")]
    let surface = instance
        .create_surface(wgpu::SurfaceTarget::Canvas(canvas))
        .map_err(|e| JsValue::from_str(&format!("Failed to create surface: {}", e)))?;
    #[cfg(not(target_arch = "wasm32"))]
    let surface: wgpu::Surface<'static> = {
        let _ = (&canvas, &instance);
        unreachable!("WASM bridge only runs in browser")
    };

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
