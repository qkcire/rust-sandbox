use bracket_lib::prelude::*;

enum GameMode {
    Menu,
    Playing,
    End,
}

struct State {
    mode: GameMode,
}

impl State {
    fn new() -> Self {
        State {
            mode: GameMode::Menu,
        }
    }
}

impl GameState for State {
    fn tick(&self , ctx: &mut BTerm) {
        match self.mode {
            GameMode:: => self.main_menu(ctx),
            GameMode:: => self.dead(ctx),
            GameMode:: => self.play(ctx),
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        // TODO: Fill in this stub later
        self.mode = GameMode::End;
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;
    main_loop(context, State::new())
}
