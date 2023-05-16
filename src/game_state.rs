use std::todo;

use bracket_lib::prelude::*;

enum GameMode {
    Menu,
    Playing,
    End,
}

pub struct State {
    mode: GameMode,
}

impl State {
    pub fn new() -> Self {
        State {
            mode: GameMode::Menu,
        }
    }
    fn restart(&mut self) {
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
        //TODO
        self.mode = GameMode::End;
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
