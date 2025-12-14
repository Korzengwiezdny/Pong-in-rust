use ggez::event;
use ggez::graphics::{self, Color};
use ggez::{Context, ContextBuilder, GameResult};
use ggez::glam::Vec2;
use ggez::input::keyboard::{self, KeyCode};

struct GameState {
    // pozycja gracza (nasz kwadrat)
    player1_pos: Vec2,
    player2_pos: Vec2,

    // szybkość poruszania
    player_speed: f32,
}

impl GameState {
    fn new() -> GameResult<GameState> {
        Ok(GameState {
            // start na środku mniej więcej
            player1_pos: Vec2::new(400.0, 300.0),
            player2_pos: Vec2::new(500.0, 300.0),
            player_speed: 5.0,
        })
    }
}

// Tu mówimy ggez, że GameState ma obsługiwać pętlę gry
impl event::EventHandler<ggez::GameError> for GameState {
    /// Logika gry – wywoływana co klatkę
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let mut dir = Vec2::ZERO;

        // sprawdzamy wciśnięte klawisze
        if keyboard::is_key_pressed(ctx, KeyCode::Up) {
            dir.y -= 1.0;
        }
        if keyboard::is_key_pressed(ctx, KeyCode::Down) {
            dir.y += 1.0;
        }
        


        // normalizacja, żeby po skosie nie był szybszy
        if dir.length_squared() > 0.0 {
            dir = dir.normalize();
            self.player_pos += dir * self.player_speed;
        }

        // proste ograniczenie do (przybliżonego) rozmiaru okna 800x600
        let half_size = 25.0; // połowa wielkości kwadratu
        self.player_pos.x = self.player_pos.x.clamp(0.0 + half_size, 800.0 - half_size);
        self.player_pos.y = self.player_pos.y.clamp(0.0 + half_size, 600.0 - half_size);

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
        let rect = graphics::Rect::new(-25.0, -25.0, 50.0, 50.0);

        let player = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            rect,
            Color::from([0.9, 0.9, 0.2, 1.0]), // żółtawy kwadrat
        )?;

        // rysujemy kwadrat w miejscu player_pos
        canvas.draw(&player, self.player_pos);

        // wysyłamy ramkę na ekran
        canvas.finish(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult {
    // Tworzymy kontekst i pętlę zdarzeń
    let cb = ContextBuilder::new("simple_game", "twoje_imie");
    let (ctx, event_loop) = cb.build()?;

    // Tworzymy stan gry
    let state = GameState::new()?;

    // Odpalamy pętlę gry
    event::run(ctx, event_loop, state)
}