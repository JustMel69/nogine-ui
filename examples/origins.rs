use nogine::{window::WindowCfg, unwrap_res, graphics::Graphics, math::Vector2, color::{Color4, Color}};
use nogine_ui::ui::{init_ui, UI, Origin};

const UI_RENDER_TARGET: u8 = 0xFF;

fn main() {
    let mut window = unwrap_res!(WindowCfg::default().res((1280, 720)).title("Origins example").init());
    
    let pipeline = init_ui(UI_RENDER_TARGET);

    Graphics::set_cam(Vector2::ZERO, Vector2(3.0 * window.aspect_ratio(), 3.0));
    UI::set_res(window.get_size());

    while window.is_running() {
        window.pre_tick(Some(&pipeline));
        
        Graphics::set_cam(Vector2::ZERO, Vector2(3.0 * window.aspect_ratio(), 3.0));
        UI::set_res(window.get_size());

        draw_world();
        draw_ui();

        window.post_tick();
    }
}

fn draw_world() {
    Graphics::draw_rect(Vector2::ZERO, Vector2::ONE, Color4::GREEN);
}

fn draw_                ui() {
    UI::set_pivot_origin(Vector2(0.0, 1.0), Origin::Center);
    UI::draw_rect(Vector2::ZERO, Vector2::one(128.0), Color4::RED);
}