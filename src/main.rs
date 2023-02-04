use bracket_lib::prelude::*;

struct State {}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        // ctx (context) provides functions for interacting with the
        // game display; cls() clears the window
        ctx.cls();
        ctx.print(1, 1, "Hello, Bracket Terminal!");
    }
}

fn main() {
    println!("Hello, world!");
}
