use ggez::{
    event,
    graphics::{drawable_size, DrawParam, Drawable, Mesh},
    input::keyboard::is_key_pressed,
    Context, GameResult,
};
use glam::vec2;

pub struct Player {
    pub pos: f32,
    mesh: Mesh,
}

impl Player {
    pub fn new(ctx: &mut Context, mesh: Mesh) -> Self {
        let (width, _) = drawable_size(ctx);

        Player {
            pos: width / 2.0,
            mesh,
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let (_, height) = drawable_size(ctx);
        self.mesh.draw(
            ctx,
            DrawParam::new()
                .dest(vec2(self.pos, height - 20.0))
                .offset(vec2(100.0, 0.0)),
        )
    }

    pub fn move_player(&mut self, ctx: &mut Context) {
        let (width, _) = drawable_size(ctx);

        if is_key_pressed(ctx, event::KeyCode::D) || is_key_pressed(ctx, event::KeyCode::Right) {
            self.pos += 100.0 / 30.0;
        }
        if is_key_pressed(ctx, event::KeyCode::A) || is_key_pressed(ctx, event::KeyCode::Left) {
            self.pos -= 100.0 / 30.0;
        }

        self.pos = self.pos.clamp(200.0 / 2.0, width - 200.0 / 2.0);
    }
}
