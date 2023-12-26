use nogine::graphics::{pipeline::{RenderPipeline, RenderTexture, SceneRenderData, DEFAULT_RENDER_TARGET}, RenderStats, texture::TextureFiltering, BlendingMode};

use crate::ui::UI;

pub struct UIDefaultPipeline;

impl RenderPipeline for UIDefaultPipeline {
    fn render(&self, screen_rt: &mut RenderTexture, scene_data: &SceneRenderData, stats: &mut RenderStats) {
        let mut rt = RenderTexture::sized_as(&screen_rt, TextureFiltering::Linear);
        rt.clear(scene_data.clear_col());
        rt.render_scene(scene_data, DEFAULT_RENDER_TARGET, stats);

        let ui_rt = UI::render_to_texture(TextureFiltering::Linear, scene_data, stats);

        screen_rt.combine(&rt, BlendingMode::AlphaMix, stats);
        screen_rt.combine(&ui_rt, BlendingMode::AlphaMix, stats);
    }
}