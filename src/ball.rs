use ggez::{
    event,
    graphics::{drawable_size, DrawParam, Drawable, Mesh, Rect},
    input::keyboard::is_key_pressed,
    Context, GameResult,
};
use glam::{vec2, Vec2};

pub struct Ball {
    vel: Vec2,
    pub pos: Vec2,
    mesh: Mesh,
    is_playing: bool,
}

impl Ball {
    pub fn new(ctx: &mut Context, mesh: Mesh) -> Self {
        let (width, height) = drawable_size(ctx);
        Ball {
            vel: vec2(
                -(std::f32::consts::TAU / -3.0).cos() * 200.0,
                (std::f32::consts::TAU / -3.0).sin() * 200.0,
            ),
            pos: vec2(width / 2.0, height - 40.0),
            mesh,
            is_playing: false,
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.mesh.draw(ctx, DrawParam::new().dest(self.pos))
    }

    pub fn move_ball(&mut self, ctx: &mut Context) {
        if is_key_pressed(ctx, event::KeyCode::Space) {
            self.is_playing = true;
        }

        if self.is_playing {
            self.pos += self.vel / 50.0;
        }
    }

    pub fn collision(
        &mut self,
        ctx: &mut Context,
        blocks: &mut Vec<Rect>,
        paddle: Rect,
        score: &mut u32,
    ) {
        let (width, _) = drawable_size(ctx);

        if paddle.overlaps_circle(self.pos, 10.0) {
            self.on_collide(paddle.x + paddle.w / 2.0);
        }

        let mut deref = Vec::new();

        for (idx, block) in blocks.iter().enumerate() {
            if block.overlaps_circle(self.pos, 10.0) {
                self.on_collide(block.x + block.w / 2.0);

                deref.push(idx);

                *score += 1;
                self.vel.x *= -1.03; //Increment Speed
            }
        }

        for idx in deref.drain(..).rev() {
            blocks.remove(idx);
        }

        if self.pos.x >= width - 10.0 || self.pos.x <= 0.0 + 10.0 {
            self.vel.x *= -1.0;
        }

        if self.pos.y <= 0.0 {
            self.vel.y *= -1.0;
        }
    }

    fn on_collide(&mut self, middle: f32) {
        self.vel.y *= -1.0;
        self.vel.x *= (middle - self.pos.x).signum();
    }
}
