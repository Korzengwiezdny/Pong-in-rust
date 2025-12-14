const SCREEN_W: f32 = 800.0;
const SCREEN_H: f32 = 600.0;
const PADDLE_W: f32 = 30.0;
const PADDLE_H: f32 = 150.0;
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
        // prostokaty przyklejone do ścian, ograniczenie tylko w pionie
        self.player1_pos.x = 0.0 + (PADDLE_W * 0.5);
        self.player2_pos.x = SCREEN_W - (PADDLE_W * 0.5);
        let half_paddle_h = PADDLE_H * 0.5;
        self.player1_pos.y = self.player1_pos.y.clamp(0.0 + half_paddle_h, SCREEN_H - half_paddle_h);
        self.player2_pos.y = self.player2_pos.y.clamp(0.0 + half_paddle_h, SCREEN_H - half_paddle_h);

        //pilka

        self.ball_pos += self.ball_vel;
        let w = SCREEN_W;
        let h = SCREEN_H;
        let r = self.ball_radius;

        // if self.ball_pos.x - r <= 0.0 {
        //     self.ball_pos.x = r;
        //     self.ball_vel.x = -self.ball_vel.x;
        // }

        // if self.ball_pos.x + r >= w{
        //     self.ball_pos.x = w - r;
        //     self.ball_vel.x = -self.ball_vel.x;
        // }


        if self.ball_pos.y - r <= 0.0 {
            self.ball_pos.y = r;
            self.ball_vel.y = -self.ball_vel.y;
        }

        if self.ball_pos.y + r >= h{
            self.ball_pos.y = h - r;
            self.ball_vel.y = -self.ball_vel.y;
        }

     
        let half_pw = PADDLE_W * 0.5;
        let half_ph = PADDLE_H * 0.5;

    //kolizja piolki z prostokątem 
        let mut collide_with_paddle = |paddle_pos: Vec2| {
            let left = paddle_pos.x - half_pw;
            let right = paddle_pos.x + half_pw;
            let top = paddle_pos.y - half_ph;
            let bottom = paddle_pos.y + half_ph;

            let closest_x = self.ball_pos.x.clamp(left, right);
            let closest_y = self.ball_pos.y.clamp(top, bottom);

            let dx = self.ball_pos.x - closest_x;
            let dy = self.ball_pos.y - closest_y;

            if dx * dx + dy * dy <= r * r {
                // odbicie w osi X (piłka odbija się od prostokata)
                if self.ball_pos.x < paddle_pos.x {
                    self.ball_vel.x = -self.ball_vel.x.abs();
                    self.ball_pos.x = left - r; // wypchnij na lewo od prostokata
                } else {
                    self.ball_vel.x = self.ball_vel.x.abs();
                    self.ball_pos.x = right + r; // wypchnij na prawo od prostokata
                }

              
                // hit = -1 (góra prostokata) ... +1 (dół prostokata)
                let hit = ((self.ball_pos.y - paddle_pos.y) / half_ph).clamp(-1.0, 1.0);
                self.ball_vel.y += hit * 2.5;
            }
        };

        collide_with_paddle(self.player1_pos);
        collide_with_paddle(self.player2_pos);


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
        let rect = graphics::Rect::new(-PADDLE_W * 0.5, -PADDLE_H * 0.5, PADDLE_W, PADDLE_H);

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