use ggez::event;
use ggez::graphics::{self, Color};
use ggez::{Context, ContextBuilder, GameResult};
use ggez::glam::Vec2;
use ggez::input::keyboard::{self, KeyCode};

struct GameState {
    // pozycja gracza (nasz kwadrat)
    player1_pos: Vec2,
    player2_pos: Vec2,
    ball_pos: Vec2,
    ball_vel: Vec2,
    ball_radius: f32,

    // szybkość poruszania
    player_speed: f32,
}


impl GameState {
    fn new() -> GameResult<GameState> {
        Ok(GameState {
            // start na środku mniej więcej
            player1_pos: Vec2::new(25.0, 300.0),
            player2_pos: Vec2::new(775.0, 300.0),
            ball_pos: Vec2::new(400.0, 300.0),
            ball_vel: Vec2::new(4.0, 3.0),
            ball_radius: 12.0,
        
            player_speed: 5.0,
        })
    }
}


impl event::EventHandler<ggez::GameError> for GameState {
    /// Logika gry – wywoływana co klatkę
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // ---- PLAYER 1 (strzałki) ----
        let mut dir1 = Vec2::ZERO;

        // if keyboard::is_key_pressed(ctx, KeyCode::Left) {
        //     dir1.x -= 1.0;
        // }
        // if keyboard::is_key_pressed(ctx, KeyCode::Right) {
        //     dir1.x += 1.0;
        // }
        if keyboard::is_key_pressed(ctx, KeyCode::Up) {
            dir1.y -= 1.0;
        }
        if keyboard::is_key_pressed(ctx, KeyCode::Down) {
            dir1.y += 1.0;
        }

        if dir1.length_squared() > 0.0 {
            dir1 = dir1.normalize();
            self.player1_pos += dir1 * self.player_speed;
        }

        // ---- PLAYER 2 (WASD) ----
        let mut dir2 = Vec2::ZERO;

        // if keyboard::is_key_pressed(ctx, KeyCode::A) {
        //     dir2.x -= 1.0;
        // }
        // if keyboard::is_key_pressed(ctx, KeyCode::D) {
        //     dir2.x += 1.0;
        // }
        if keyboard::is_key_pressed(ctx, KeyCode::W) {
            dir2.y -= 1.0;
        }
        if keyboard::is_key_pressed(ctx, KeyCode::S) {
            dir2.y += 1.0;
        }

        if dir2.length_squared() > 0.0 {
            dir2 = dir2.normalize();
            self.player2_pos += dir2 * self.player_speed;
        }

        // ograniczenie do rozmiaru okna 800x600 (dla obu graczy)
        let half_size = 25.0;
        self.player1_pos.x = self.player1_pos.x.clamp(0.0 + half_size, 800.0 - half_size);
        self.player1_pos.y = self.player1_pos.y.clamp(0.0 + half_size, 600.0 - half_size);
        self.player2_pos.x = self.player2_pos.x.clamp(0.0 + half_size, 800.0 - half_size);
        self.player2_pos.y = self.player2_pos.y.clamp(0.0 + half_size, 600.0 - half_size);

        //pilka

        self.ball_pos += self.ball_vel;
        let w = 800.0;
        let h = 600.0;
        let r = self.ball_radius;

        if self.ball_pos.x - r <= 0.0 {
            self.ball_pos.x = r;
            self.ball_vel.x = -self.ball_vel.x;
        }

        if self.ball_pos.x + r >= w{
            self.ball_pos.x = w - r;
            self.ball_vel.x = -self.ball_vel.x;
        }


        if self.ball_pos.y - r <= 0.0 {
            self.ball_pos.y = r;
            self.ball_vel.y = -self.ball_vel.y;
        }

        if self.ball_pos.y + r >= h{
            self.ball_pos.y = h - r;
            self.ball_vel.y = -self.ball_vel.y;

        }


        Ok(())
    }

    /// Rysowanie – wywoływane co klatkę po update()
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // czyścimy ekran kolorem tła
        let mut canvas = graphics::Canvas::from_frame(
            ctx,
            Color::from([0.1, 0.2, 0.3, 1.0]), // ciemne tło
        );

        // prostokąt (kwadrat) o środku w (0,0) i rozmiarze 50x50
        let rect = graphics::Rect::new(-25.0, -25.0, 30.0, 150.0);

        let player1 = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            rect,
            Color::WHITE, // żółtawy kwadrat
        )?;

        let player2 = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            rect,
            Color::WHITE,
        )?;

        //pilka bedize tutaj
        let ball = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Vec2::ZERO,
            self.ball_radius,
            1.0,
            Color::WHITE,

        )?;
       

        // rysujemy kwadrat w miejscu player_pos
        canvas.draw(&player1, self.player1_pos);
        canvas.draw(&player2, self.player2_pos);

        //rysujemy pilke 
        canvas.draw(&ball, self.ball_pos);

        // wysyłamy ramkę na ekran
        canvas.finish(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult {
    // Tworzymy kontekst i pętlę zdarzeń
    let cb = ContextBuilder::new("pong", "gracz");
    let (ctx, event_loop) = cb.build()?;

    // Tworzymy stan gry
    let state = GameState::new()?;

    // Odpalamy pętlę gry
    event::run(ctx, event_loop, state)
}