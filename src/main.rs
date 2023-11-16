use ggez::GameResult;
use ggez::event;
use main_state::MainState;

mod main_state;



pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("level-editor", "vil'mo");
    let (ctx, event_loop) = cb.build()?;
    let state = MainState::new()?;
    event::run(ctx, event_loop, state)
}
