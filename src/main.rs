use ggez;
use ggez::event;
use ggez::graphics;
use ggez::input::keyboard::{self, KeyCode};
use ggez::{Context, GameResult};
use rand::seq::SliceRandom;

#[allow(dead_code)]
const PADDING: f32 = 40.0;
const RACKET_WIDTH: f32 = 20.0;
const RACKET_HEIGHT: f32 = 100.0;
const RACKET_SPEED: f32 = 500.0;
const BALL_RADIUS: f32 = 15.0;
const BALL_VELOCITY: f32 = 300.0;

const LINE_WIDTH: f32 = 2.0;
const SPEED_INCREASE_PER_BOUNCE: f32 = 5.0;

struct MainState {
    player_one_pos: Point<f32>,
    player_two_pos: Point<f32>,
    ball_pos: Point<f32>,
    ball_vel: Point<f32>,
    player_one_score: u32,
    player_two_score: u32,
    number_of_bounces: u32,
}

struct Point<T> {
    x: T,
    y: T,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> Self {
        let (screen_width, screen_height) = graphics::drawable_size(ctx);
        MainState {
            player_one_pos: Point {
                x: RACKET_WIDTH / 2. + PADDING,
                y: screen_height / 2.,
            },
            player_two_pos: Point {
                x: screen_width - ((RACKET_WIDTH / 2.) + PADDING),
                y: screen_height / 2.,
            },

            ball_pos: Point {
                x: screen_width / 2.,
                y: screen_height / 2.,
            },
            ball_vel: Point {
                x: vec![-1., 1.].choose(&mut rand::thread_rng()).unwrap() * BALL_VELOCITY,
                y: vec![-1., 1.].choose(&mut rand::thread_rng()).unwrap() * BALL_VELOCITY,
            },
            player_one_score: 0,
            player_two_score: 0,
            number_of_bounces: 0,
        }
    }
}

fn move_racket(pos: &mut Point<f32>, key_code: KeyCode, y_dir: f32, ctx: &mut Context) {
    let dt = ggez::timer::delta(ctx).as_secs_f32();
    if keyboard::is_key_pressed(ctx, key_code) {
        pos.y += y_dir * RACKET_SPEED * dt;
    }
    let screen_height = graphics::drawable_size(ctx).1 as f32;
    pos.y = *clamp(
        &pos.y,
        &(RACKET_HEIGHT / 2.),
        &(screen_height - RACKET_HEIGHT / 2.),
    );
}

fn clamp<T: PartialOrd>(val: T, min: T, max: T) -> T {
    if val < min {
        min
    } else if val > max {
        max
    } else {
        val
    }
}

fn move_ball(
    ball_pos: &mut Point<f32>,
    ball_vel: &mut Point<f32>,
    number_of_bounces: &u32,
    ctx: &mut Context,
) {
    let (screen_width, screen_height) = graphics::drawable_size(ctx);
    let dt = ggez::timer::delta(ctx).as_secs_f32();

    ball_pos.y +=
        ball_vel.y * dt * (100. + *number_of_bounces as f32 * SPEED_INCREASE_PER_BOUNCE) / 100.;
    ball_pos.x +=
        ball_vel.x * dt * (100. + *number_of_bounces as f32 * SPEED_INCREASE_PER_BOUNCE) / 100.;

    ball_pos.x = *clamp(&ball_pos.x, &(BALL_RADIUS), &(screen_width - BALL_RADIUS));

    if ball_pos.y <= 0. + BALL_RADIUS || ball_pos.y >= screen_height - BALL_RADIUS {
        flip_ball_velocity(ctx, ball_vel, ball_pos, false);
    }
}

fn flip_ball_velocity(
    ctx: &Context,
    ball_vel: &mut Point<f32>,
    ball_pos: &mut Point<f32>,
    flip_x: bool,
) {
    let (screen_width, screen_height) = graphics::drawable_size(ctx);
    if flip_x {
        if ball_pos.x > screen_width / 2. {
            ball_vel.x = -ball_vel.x.abs();
        } else {
            ball_vel.x = ball_vel.x.abs();
        }
    } else {
        if ball_pos.y > screen_height / 2. {
            ball_vel.y = -ball_vel.y.abs();
        } else {
            ball_vel.y = ball_vel.y.abs();
        }
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let (screen_width, screen_height) = graphics::drawable_size(ctx);

        // Move player one
        move_racket(&mut self.player_one_pos, KeyCode::W, -1., ctx);
        move_racket(&mut self.player_one_pos, KeyCode::S, 1., ctx);
        // Move player two
        move_racket(&mut self.player_two_pos, KeyCode::Up, -1., ctx);
        move_racket(&mut self.player_two_pos, KeyCode::Down, 1., ctx);

        // Move ball
        move_ball(
            &mut self.ball_pos,
            &mut self.ball_vel,
            &mut self.number_of_bounces,
            ctx,
        );

        // Check if the ball will hit a racket
        let ball_intersects_player_one = {
            self.ball_pos.x - BALL_RADIUS < self.player_one_pos.x + RACKET_WIDTH / 2.
                && self.ball_pos.x + BALL_RADIUS > self.player_one_pos.x - RACKET_WIDTH / 2.
                && self.ball_pos.y - BALL_RADIUS < self.player_one_pos.y + RACKET_HEIGHT / 2.
                && self.ball_pos.y + BALL_RADIUS > self.player_one_pos.y - RACKET_HEIGHT / 2.
        };

        let ball_intersects_player_two = {
            self.ball_pos.x - BALL_RADIUS < self.player_two_pos.x + RACKET_WIDTH / 2.
                && self.ball_pos.x + BALL_RADIUS > self.player_two_pos.x - RACKET_WIDTH / 2.
                && self.ball_pos.y - BALL_RADIUS < self.player_two_pos.y + RACKET_HEIGHT / 2.
                && self.ball_pos.y + BALL_RADIUS > self.player_two_pos.y - RACKET_HEIGHT / 2.
        };

        if ball_intersects_player_one || ball_intersects_player_two {
            flip_ball_velocity(ctx, &mut self.ball_vel, &mut self.ball_pos, true);
            self.number_of_bounces += 1;
        }

        if self.ball_pos.x <= BALL_RADIUS || self.ball_pos.x >= screen_width - BALL_RADIUS {
            match self.ball_pos.x <= BALL_RADIUS {
                true => self.player_one_score += 1,
                false => self.player_two_score += 1,
            }

            self.ball_pos = Point {
                x: screen_width / 2.,
                y: screen_height / 2.,
            };

            self.ball_vel = Point {
                x: vec![-1., 1.].choose(&mut rand::thread_rng()).unwrap() * BALL_VELOCITY,
                y: vec![-1., 1.].choose(&mut rand::thread_rng()).unwrap() * BALL_VELOCITY,
            };
            self.number_of_bounces = 0;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let (screen_width, screen_height) = graphics::drawable_size(ctx);

        graphics::clear(ctx, graphics::Color::BLACK);

        let line = graphics::Rect::new(
            screen_width / 2. - LINE_WIDTH / 2.,
            0.,
            LINE_WIDTH,
            screen_height,
        );

        let line_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            line,
            graphics::Color::new(1., 1., 1., 0.5),
        )?;

        let racket = graphics::Rect::new(
            -RACKET_WIDTH / 2.,
            -RACKET_HEIGHT / 2.,
            RACKET_WIDTH,
            RACKET_HEIGHT,
        );
        let racket_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            racket,
            graphics::Color::WHITE,
        )?;

        let ball_rect = graphics::Rect::new(
            -BALL_RADIUS,
            -BALL_RADIUS,
            BALL_RADIUS * 2.,
            BALL_RADIUS * 2.,
        );

        let ball_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            ball_rect,
            graphics::Color::WHITE,
        )?;

        let player_one_params =
            graphics::DrawParam::default().dest([self.player_one_pos.x, self.player_one_pos.y]);

        let player_two_params =
            graphics::DrawParam::default().dest([self.player_two_pos.x, self.player_two_pos.y]);

        let ball_params = graphics::DrawParam::default().dest([self.ball_pos.x, self.ball_pos.y]);

        graphics::draw(ctx, &line_mesh, graphics::DrawParam::default())?;
        graphics::draw(ctx, &racket_mesh, player_one_params)?;
        graphics::draw(ctx, &racket_mesh, player_two_params)?;
        graphics::draw(ctx, &ball_mesh, ball_params)?;

        let score_text = graphics::Text::new(format!(
            "{}            {}",
            self.player_one_score, self.player_two_score
        ));

        let (text_width, text_height) = (score_text.width(ctx), score_text.height(ctx));
        let score_params = graphics::DrawParam::default().dest([
            (screen_width - text_width) / 2.,
            screen_height / 4. - text_height / 2.,
        ]);

        graphics::draw(ctx, &score_text, score_params)?;
        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    let (mut ctx, event_loop) = ggez::ContextBuilder::new("Pong", "Alan").build()?;

    graphics::set_window_title(&mut ctx, "Pong");

    let state = MainState::new(&mut ctx);

    event::run(ctx, event_loop, state);
}
