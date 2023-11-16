use ggez::event;
use ggez::GameResult;
use main_state::MainState;

mod main_state;


pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("level-editor", "vil'mo");
    let (ctx, event_loop) = cb.build()?;
    let state = MainState::new(&ctx)?;
    event::run(ctx, event_loop, state)
}
