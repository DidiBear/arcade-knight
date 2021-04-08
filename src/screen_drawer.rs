//! The screen drawer for scaling things drawn to the game into the screen size.

use macroquad::prelude::*;

pub struct ScreenDrawer {
    render_target: RenderTarget,
    game_size: Vec2,
}

impl ScreenDrawer {
    pub fn new(game_size: Vec2) -> Self {
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let render_target = render_target(game_size.x as u32, game_size.y as u32);
        set_texture_filter(render_target.texture, FilterMode::Nearest);

        Self {
            render_target,
            game_size,
        }
    }

    /// Executes the given `draw` function and scale the drawn things to the screen size.
    pub fn draw_scaled(&self, draw: impl FnOnce()) {
        set_camera(Camera2D {
            zoom: 1.0 / self.game_size * 2.0,
            target: (self.game_size * 0.5).floor(),
            render_target: Some(self.render_target),
            ..Camera2D::default()
        });

        draw();

        set_default_camera();
        draw_texture_ex(
            self.render_target.texture,
            0.,
            0.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..DrawTextureParams::default()
            },
        );
    }
}

/// Load a texture that will be scaled to the screen.
pub async fn load_scalable_texture(file_name: &str) -> Texture2D {
    let texture = load_texture(file_name).await;
    set_texture_filter(texture, FilterMode::Nearest); // scale pixels without smoothing
    texture
}
