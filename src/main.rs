use ggez::{
    graphics,
    ContextBuilder,
    Context,
    GameResult,
    event::{self, EventHandler}
};

fn main() {
    let (mut ctx, mut event_loop) = ContextBuilder::new("Breakout", "James A. Sral")
        .build()
        .expect("Unaable to create ggez context");

    let mut breakout_game = BreakoutGame::new(&mut ctx);

    match event::run(&mut ctx, &mut event_loop, &mut breakout_game) {
        Ok(_) => println!("Play again soon"),
        Err(e) => println!("Error occured: {}", e)
    }
}

struct Position {
    x: i32,
    y: i32,
}

struct BreakoutGame {
    paddle_pos: Position,
    should_quit: bool,
}

impl BreakoutGame {
    pub fn new(_ctx: &mut Context) -> BreakoutGame {
        BreakoutGame {
            paddle_pos: Position { x: 20, y: 20 },
            should_quit: false,
        }
    }
}

impl EventHandler for BreakoutGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::Color { r: 0.2, g: 0.4, b: 0.5, a: 1.0 });
        graphics::present(ctx)
    }
}
