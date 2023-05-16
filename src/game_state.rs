use crate::player::Player;

use bracket_lib::prelude::*;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FRAME_DURATION: f32 = 75.0;

enum GameMode {
    Menu,
    Playing,
    End,
}

pub struct State {
    player: Player,
    frame_time: f32,
    mode: GameMode,
}

impl State {
    pub fn new() -> Self {
        State {
            player: Player::new(2, 25),
            frame_time: 0.0,
            mode: GameMode::Menu,
        }
    }
    fn restart(&mut self) {
        self.player = Player::new(5, 25);
        self.frame_time = 0.0;
        self.mode = GameMode::Playing;
    }
    fn start_screen(&mut self, ctx: &mut BTerm) {
        self.menu(ctx, "Welcome to Flappy Dragon");
    }
    fn dead(&mut self, ctx: &mut BTerm) {
        self.menu(ctx, "You are dead!")
    }
    fn menu(&mut self, ctx: &mut BTerm, message: &str) {
        ctx.cls();
        ctx.print_centered(5, message);
        ctx.print_centered(8, "(P) Play");
        ctx.print_centered(9, "(Q) Quit");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }
    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(NAVY);
        self.frame_time += ctx.frame_time_ms;
        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
            self.player.gravity_and_move();
        }
        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap();
        }
        self.player.render(ctx);
        ctx.print(0, 0, "Press Space to flap.");
        if self.player.y > SCREEN_HEIGHT {
            self.mode = GameMode::End;
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.start_screen(ctx),
            GameMode::End => self.dead(ctx),
            GameMode::Playing => self.play(ctx),
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}
