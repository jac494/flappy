use bracket_lib::prelude::*;

enum GameMode {
    Menu,
    Playing,
    End,
}

struct State {}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        // ctx (context) provides functions for interacting with the
        // game display; cls() clears the window
        ctx.cls();
        ctx.print(1, 1, "Hello, Bracket Terminal!");
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;

    main_loop(context, State {})
}
