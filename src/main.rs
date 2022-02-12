use ball::Ball;
use ggez::conf::WindowSetup;
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color, DrawParam, Drawable, Mesh, Rect, TextFragment};
use ggez::timer::check_update_time;
use ggez::{Context, ContextBuilder, GameResult};
use glam::{vec2, Vec2};
use player::Player;

mod ball;
mod player;

enum GameState {
    Play,
    Over,
}

fn main() {
    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("Atari Breakout", "TheJayDuck")
        .window_setup(WindowSetup {
            title: "TheJayDuck's Atari Breakout".to_owned(),
            ..Default::default()
        })
        .build()
        .expect("aieee, could not create ggez context!");

    let my_game = MyGame::new(&mut ctx).unwrap();

    // Run!
    event::run(ctx, event_loop, my_game);
}

struct MyGame {
    // Your state here...
    game_state: GameState,
    score: u32,

    player: Player,
    ball: Ball,
    blocks: Vec<Rect>,
}

impl MyGame {
    pub fn new(ctx: &mut Context) -> GameResult<MyGame> {
        // Load/create resources such as images here.
        let (width, _) = graphics::drawable_size(ctx);

        let mesh = Mesh::new_rounded_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            Rect {
                x: 0.0,
                y: 0.0,
                w: 200.0,
                h: 10.0,
            },
            20.0,
            Color::WHITE,
        )?;

        let mesh_ball = Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            vec2(0.0, 0.0),
            10.0,
            0.01,
            Color::WHITE,
        )?;

        let block_count = (width / 82.0) as u32;
        let empty_width = width - block_count as f32 * 82.0;

        Ok(MyGame {
            game_state: GameState::Play,
            score: 0,
            player: Player::new(ctx, mesh),
            ball: Ball::new(ctx, mesh_ball),
            blocks: (0..block_count)
                .flat_map(|x| {
                    (0..6).map(move |y| {
                        Rect::new(
                            82.0 * x as f32 + empty_width / 2.0,
                            22.0 * y as f32,
                            80.0,
                            20.0,
                        )
                    })
                })
                .collect(),
        })
    }

    pub fn check_state(&mut self, height: f32) -> bool {
        self.ball.pos.y >= height
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        // Update code here...
        let (_, height) = graphics::drawable_size(ctx);

        if self.check_state(height) {
            self.game_state = GameState::Over;
        }

        match self.game_state {
            GameState::Play => {
                while check_update_time(ctx, 50) {
                    self.ball.collision(
                        ctx,
                        &mut self.blocks,
                        Rect::new(self.player.pos - 100.0, height - 20.0, 200.0, 10.0),
                        &mut self.score,
                    );
                    self.player.move_player(ctx);
                    self.ball.move_ball(ctx);
                }
            }
            GameState::Over => {}
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let (width, height) = graphics::drawable_size(ctx);

        graphics::clear(ctx, Color::BLACK);

        for block in &self.blocks {
            Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                *block,
                Color::from_rgb(
                    ((block.y as f32 * std::f32::consts::TAU / 10.0 + 0.0).sin() * 127.0 + 128.0)
                        as u8,
                    ((block.y as f32 * std::f32::consts::TAU / 10.0 + 2.0).sin() * 127.0 + 128.0)
                        as u8,
                    ((block.y as f32 * std::f32::consts::TAU / 10.0 + 4.0).sin() * 127.0 + 128.0)
                        as u8,
                ),
            )?
            .draw(ctx, DrawParam::new())?;

            //Collision Visualizer - Block
            // Mesh::new_rectangle(
            //     ctx,
            //     graphics::DrawMode::fill(),
            //     *block,
            //     Color::new(255.0, 0.0, 0.0, 0.9),
            // )?
            // .draw(ctx, DrawParam::new())?;
        }

        let score_text =
            graphics::Text::new(TextFragment::new(format!("Score: {0}", self.score)).scale(20.0));

        score_text.draw(
            ctx,
            DrawParam::new()
                .dest([0.0, height])
                .offset(vec2(0.0, score_text.dimensions(ctx).h)),
        )?;

        self.player.draw(ctx)?;
        self.ball.draw(ctx)?;

        if let GameState::Over = self.game_state {
            graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                Rect {
                    x: 0.0,
                    y: 0.0,
                    w: width,
                    h: height,
                },
                Color {
                    a: 0.5,
                    ..Color::RED
                },
            )?
            .draw(ctx, DrawParam::new())?;

            let win_text = graphics::Text::new(TextFragment::new("Game OVER!").scale(20.0));

            let win_text_dim = win_text.dimensions(ctx);

            win_text.draw(
                ctx,
                DrawParam::new()
                    .dest([width / 2.0, height / 2.0])
                    .offset(vec2(win_text_dim.w, win_text_dim.h) / Vec2::splat(2.0)),
            )?;
        }

        //Collision Visualizer - Paddle
        // Mesh::new_rectangle(
        //     ctx,
        //     graphics::DrawMode::fill(),
        //     Rect::new(self.player.pos - 100.0, height - 20.0, 200.0, 10.0),
        //     Color::new(0.0, 0.0, 255.0, 0.9),
        // )?
        // .draw(ctx, DrawParam::new())?;

        // for x in 0..(width / 82.0) as u32 {
        //     for y in 0..3 {
        //         Mesh::new_rectangle(
        //             ctx,
        //             graphics::DrawMode::fill(),
        //             Rect {
        //                 x: 82.0 * x as f32,
        //                 y: 22.0 * y as f32,
        //                 w: 80.0,
        //                 h: 20.0,
        //             },
        //             Color::from_rgb(
        //                 ((y as f32 * std::f32::consts::TAU / 10.0 + 0.0).sin() * 127.0 + 128.0)
        //                     as u8,
        //                 ((y as f32 * std::f32::consts::TAU / 10.0 + 2.0).sin() * 127.0 + 128.0)
        //                     as u8,
        //                 ((y as f32 * std::f32::consts::TAU / 10.0 + 4.0).sin() * 127.0 + 128.0)
        //                     as u8,
        //             ),
        //         )?
        //         .draw(ctx, DrawParam::new())?;
        //     }
        // }

        graphics::present(ctx)
    }
}
