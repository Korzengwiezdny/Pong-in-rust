const SCREEN_W: f32 = 1650.0;
const SCREEN_H: f32 = 900.0;
const PADDLE_W: f32 = 15.0;
const PADDLE_H: f32 = 150.0;
const SERVE_GAP: f32 = 2.0;
const AI_ENABLED: bool = true;   // włącz / wyłącz AI
const AI_DEADZONE: f32 = 8.0;    // tolerancja w px 
const AI_SPEED: f32 = 4.5;       // prędkość paletki AI
const BALL_SPEEDUP: f32 = 1.05; // szybciej po odbiciu
use ggez::event;
use ggez::graphics::{self, Color};
use ggez::{Context, ContextBuilder, GameResult};
use ggez::glam::Vec2;
use ggez::input::keyboard::{self, KeyCode};
use ggez::graphics::DrawParam;
use ggez::conf::{WindowMode, WindowSetup};

struct GameState {
    // pozycja gracza 
    player1_pos: Vec2,
    player2_pos: Vec2,
    ball_pos: Vec2,
    ball_vel: Vec2,
    ball_radius: f32,
    score1: u32,
    score2: u32,
    serving: bool,
    serve_from_left: bool,

    // szybkość poruszania
    player_speed: f32,
}


impl GameState {
    fn new() -> GameResult<GameState> {
        Ok(GameState {
            // Start
            player1_pos: Vec2::new(PADDLE_W * 0.5, SCREEN_H * 0.5),
            player2_pos: Vec2::new(SCREEN_W - (PADDLE_W * 0.5), SCREEN_H * 0.5),

            
            ball_pos: Vec2::new(PADDLE_W + 8.0 + SERVE_GAP, SCREEN_H * 0.5),
            ball_vel: Vec2::ZERO,
            ball_radius: 8.0, //rozmar pilki 
            
            score1: 0,
            score2: 0,
            serving: true,
            serve_from_left: true,
            player_speed: 5.0,
        })
    }

    fn prepare_serve(&mut self, from_left_paddle: bool, paddle_y: f32) {
        let margin = SERVE_GAP;
        self.serving = true;
        self.serve_from_left = from_left_paddle;

        // Ustaw piłkę przy paletce strony, która ma serwować
        self.ball_pos = Vec2::new(
            if from_left_paddle {
                PADDLE_W + self.ball_radius + margin
            } else {
                SCREEN_W - (PADDLE_W + self.ball_radius + margin)
            },
            paddle_y.clamp(self.ball_radius, SCREEN_H - self.ball_radius),
        );

        // Zatrzymaj piłkę do czasu wciśnięcia SPACJI
        self.ball_vel = Vec2::ZERO;
    }
}

impl event::EventHandler<ggez::GameError> for GameState {
    /// Logika gry wywoływana co klatkę
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if self.serving {
            if keyboard::is_key_pressed(ctx, KeyCode::Space) {
                self.serving = false;
                self.ball_vel = Vec2::new(
                    if self.serve_from_left { 5.0 } else { -5.0 },
                    3.0,
                );
            }
        }
        let mut dir1 = Vec2::ZERO;

        if keyboard::is_key_pressed(ctx, KeyCode::W) {
            dir1.y -= 1.0;
        }
        if keyboard::is_key_pressed(ctx, KeyCode::S) {
            dir1.y += 1.0;
        }

        if dir1.length_squared() > 0.0 {
            dir1 = dir1.normalize();
            self.player1_pos += dir1 * self.player_speed;
        }

 
        if AI_ENABLED {
            // Cel: y piłki (możesz też celować w "przewidywane" y, ale na start nie trzeba)
            let target_y = self.ball_pos.y;
        
            let diff = target_y - self.player2_pos.y;
        
            // Opcjonalnie: AI rusza się mocniej, gdy piłka leci do niego (ball_vel.x > 0)
            let should_track = !self.serving && self.ball_vel.x > 0.0;
        
            if should_track || self.serving {
                if diff > AI_DEADZONE {
                    self.player2_pos.y += AI_SPEED;
                } else if diff < -AI_DEADZONE {
                    self.player2_pos.y -= AI_SPEED;
                }
            } else {
                // gdy piłka leci w drugą stronę, AI wraca do środka (opcjonalnie)
                let center = SCREEN_H * 0.5;
                let diff_center = center - self.player2_pos.y;
                if diff_center > AI_DEADZONE {
                    self.player2_pos.y += AI_SPEED * 0.6;
                } else if diff_center < -AI_DEADZONE {
                    self.player2_pos.y -= AI_SPEED * 0.6;
                }
            }
        } else {
            // sterowanie ręczne 
            let mut dir2 = Vec2::ZERO;
            if keyboard::is_key_pressed(ctx, KeyCode::Up) { dir2.y -= 1.0; }
            if keyboard::is_key_pressed(ctx, KeyCode::Down) { dir2.y += 1.0; }
            if dir2.length_squared() > 0.0 {
                dir2 = dir2.normalize();
                self.player2_pos += dir2 * self.player_speed;
            }
        }

       
        // prostokaty przyklejone do ścian, ograniczenie tylko w pionie
        self.player1_pos.x = 0.0 + (PADDLE_W * 0.5);
        self.player2_pos.x = SCREEN_W - (PADDLE_W * 0.5);
        let half_paddle_h = PADDLE_H * 0.5;
        self.player1_pos.y = self.player1_pos.y.clamp(0.0 + half_paddle_h, SCREEN_H - half_paddle_h);
        self.player2_pos.y = self.player2_pos.y.clamp(0.0 + half_paddle_h, SCREEN_H - half_paddle_h);

        // serwis
        if self.serving {
            let margin = SERVE_GAP;
            let y = if self.serve_from_left {
                self.player1_pos.y
            } else {
                self.player2_pos.y
            };

            self.ball_pos = Vec2::new(
                if self.serve_from_left {
                    PADDLE_W + self.ball_radius + margin
                } else {
                    SCREEN_W - (PADDLE_W + self.ball_radius + margin)
                },
                y.clamp(self.ball_radius, SCREEN_H - self.ball_radius),
            );
        }

        //pilka

        if !self.serving {
            self.ball_pos += self.ball_vel;
        }
        let w = SCREEN_W;
        let h = SCREEN_H;
        let r = self.ball_radius;

        if self.ball_pos.y - r <= 0.0 {
            self.ball_pos.y = r;
            self.ball_vel.y = -self.ball_vel.y;
            //v2
            //self.ball_vel.y = self.ball_vel.y.clamp(-8.0, 8.0);
        }

        if self.ball_pos.y + r >= h{
            self.ball_pos.y = h - r;
            self.ball_vel.y = -self.ball_vel.y;
            //wersjav2
            //self.ball_vel.y = self.ball_vel.y.clamp(-8.0, 8.0);
        }

     
        let half_pw = PADDLE_W * 0.5;
        let half_ph = PADDLE_H * 0.5;

         // score
         if self.ball_pos.x + r < 0.0 {
            self.score2 +=1;
            self.prepare_serve(true, self.player1_pos.y);
        }
        
        if self.ball_pos.x - r > w {
            self.score1 += 1;
            self.prepare_serve(false, self.player2_pos.y);
        }
        
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
                // odbicie w osi X piłka odbija się
                if self.ball_pos.x < paddle_pos.x {
                    self.ball_vel.x = -self.ball_vel.x.abs();
                    self.ball_pos.x = left - r; // odbij na lewo
                } else {
                    self.ball_vel.x = self.ball_vel.x.abs();
                    self.ball_pos.x = right + r; // odbij na prawo
                }
                self.ball_vel *= BALL_SPEEDUP;

              
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

        // prostokąt  
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



        let score_text = graphics::Text::new(
            format!("{}    {}", self.score1,self.score2)
        );
    
        canvas.draw(
            &score_text,
            graphics::DrawParam::default()
            .dest(Vec2::new(SCREEN_W * 0.5, 20.0))
            .offset(Vec2::new(0.5, 0.0)),
        );

        if self.serving {
            let prompt = graphics::Text::new("Press SPACE to serve");
            canvas.draw(
                &prompt,
                graphics::DrawParam::default()
                    .dest(Vec2::new(SCREEN_W * 0.5, SCREEN_H * 0.5))
                    .offset(Vec2::new(0.5, 0.5)),
            );
        }

        // wysyłamy ramkę na ekran
        canvas.finish(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult {
    // Tworzymy kontekst i pętlę zdarzeń
    let cb = ContextBuilder::new("pong", "gracz")
        .window_setup(WindowSetup::default().title("An easy, good game"))
        .window_mode(
            WindowMode::default()
                .dimensions(SCREEN_W, SCREEN_H)
                .resizable(false),
        );
    let (ctx, event_loop) = cb.build()?;

    // Tworzymy stan gry
    let state = GameState::new()?;

    // Odpalamy pętlę gry
    event::run(ctx, event_loop, state)
}