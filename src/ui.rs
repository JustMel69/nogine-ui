use std::sync::RwLock;

use nogine::{log_info, graphics::{Graphics, pipeline::{RenderTexture, SceneRenderData}, texture::TextureFiltering, CamData, BlendingMode, RenderStats}, assert_expr, math::Vector2, color::Color4};

use crate::default_pipeline::UIDefaultPipeline;

/// Initializes the UI system. This function must be called once after initializing the window.
pub fn init_ui(render_target: u8) -> UIDefaultPipeline {
    let mut writer = UI_SINGLETON.write().unwrap();
    assert_expr!(!writer.initialized, "init_ui called twice!");

    writer.initialize(render_target);
    return UIDefaultPipeline;
}


static UI_SINGLETON: RwLock<UI> = RwLock::new(UI::new());

pub struct UI {
    initialized: bool,
    render_target: u8,
    res: (u32, u32),
    pivot: Vector2,
    origin: Origin,
}

impl UI {
    const fn new() -> Self {
        Self { initialized: false, render_target: 0, res: (0, 0), pivot: Vector2::ZERO, origin: Origin::TopLeft }
    }

    fn initialize(&mut self, render_target: u8) {
        self.initialized = true;
        self.render_target = render_target;

        log_info!("UI initialized.")
    }

    /// Renders the UI to a texture. This is the intended way to render the UI.
    pub fn render_to_texture(filtering: TextureFiltering, scene_data: &SceneRenderData, stats: &mut RenderStats) -> RenderTexture {
        let (res, target) = {
            let reader = UI_SINGLETON.read().unwrap();
            (reader.res, reader.render_target)
        };
        
        let mut rt = RenderTexture::new(res, filtering);
        rt.clear(Color4::CLEAR);
        rt.render_scene(scene_data, target, stats);
        return rt;
    }

    pub fn draw_rect(pos: Vector2, size: Vector2, color: Color4) {
        let old_state = Self::swap_state();
        
        let (res, origin) = {
            let reader = UI_SINGLETON.read().unwrap();
            (reader.res, reader.origin)
        };

        let pos = origin.bake_pos(pos, size, res);
        Graphics::draw_rect(pos, size, color);

        Self::restore_state(old_state);
    }

    /// Sets the pivot and origin of the UI elements.
    pub fn set_pivot_origin(pivot: Vector2, origin: Origin) {
        let mut writer = UI_SINGLETON.write().unwrap();
        writer.pivot = pivot;
        writer.origin = origin;
    }

    /// Returns the pivot and origin of the UI elements.
    pub fn get_pivot_origin() -> (Vector2, Origin) {
        let reader = UI_SINGLETON.read().unwrap();
        return (reader.pivot, reader.origin);
    }

    /// Sets the UI resolution.
    pub fn set_res(res: (u32, u32)) {
        let mut writer = UI_SINGLETON.write().unwrap();
        writer.res = res;
    }

    /// Returns the UI resolution.
    pub fn get_res() -> (u32, u32) {
        let reader = UI_SINGLETON.read().unwrap();
        return reader.res;
    }

    fn swap_state() -> RenderState {
        let reader = UI_SINGLETON.read().unwrap();
        assert_expr!(reader.initialized, "UI is not initialized");

        let old_target = Graphics::get_render_target();
        let old_pivot = Graphics::get_pivot();
        let old_cam = Graphics::get_cam_data();
        let old_ppu = Graphics::get_pixels_per_unit();
        let old_blending = Graphics::get_blending_mode();

        Graphics::set_render_target(reader.render_target);
        Graphics::set_pivot(reader.pivot);
        Graphics::set_pixels_per_unit(1.0);
        Graphics::set_blending_mode(BlendingMode::AlphaMix);

        let half_res = Vector2(reader.res.0 as f32 * 0.5, reader.res.1 as f32 * 0.5);
        unsafe { Graphics::force_cam_temp(half_res, half_res) };

        return RenderState { old_target, old_pivot, old_cam, old_ppu, old_blending };
    }

    fn restore_state(state: RenderState) {
        Graphics::set_render_target(state.old_target);
        Graphics::set_pivot(state.old_pivot);
        unsafe { Graphics::force_cam_temp(state.old_cam.pos(), state.old_cam.half_size()) };
        Graphics::set_pixels_per_unit(state.old_ppu);
        Graphics::set_blending_mode(state.old_blending);
    }
}

struct RenderState {
    old_target: u8,
    old_pivot: Vector2,
    old_cam: CamData,
    old_ppu: f32,
    old_blending: BlendingMode,
}


#[derive(Debug, Clone, Copy)]
pub enum Origin {
    TopLeft, Top, TopRight,
    Left, Center, Right,
    BottomLeft, Bottom, BottomRight,
}

impl Origin {
    fn bake_pos(&self, pos: Vector2, size: Vector2, res: (u32, u32)) -> Vector2 {
        let origin = self.pivot(res);
        let pos = Vector2(pos.0, res.1 as f32 - pos.1 - size.1);

        return origin + pos;
    }

    fn pivot(&self, res: (u32, u32)) -> Vector2 {
        match self {
            Origin::TopLeft => Vector2::ZERO,
            Origin::Top => Vector2::right(res.0 as f32 * 0.5),
            Origin::TopRight => Vector2::right(res.0 as f32),
            Origin::Left => Vector2::down(res.1 as f32 * 0.5),
            Origin::Center => Vector2(res.0 as f32 * 0.5, -(res.1 as f32) * 0.5),
            Origin::Right => Vector2(res.0 as f32, -(res.1 as f32) * 0.5),
            Origin::BottomLeft => Vector2::down(res.1 as f32),
            Origin::Bottom => Vector2(res.0 as f32 * 0.5, -(res.1 as f32)),
            Origin::BottomRight => Vector2(res.0 as f32, -(res.1 as f32)),
        }
    }
}