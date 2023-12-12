use ggez::event;
use ggez::graphics;
use ggez::graphics::Rect;
use ggez::Context;
use ggez::GameResult;

use self::input_handler::InputHandler;
use self::level_data::LevelData;
use self::resources::Resources;

mod input_handler;
mod instances;
mod level_data;
mod resources;

pub struct MainState {
    screen_rect: Rect,
    input_handler: InputHandler,
    level_data: LevelData,
    resources: Resources,
}

impl MainState {
    pub fn new(ctx: &Context) -> GameResult<MainState> {
        let mut ms = MainState {
            screen_rect: Rect::new(0.0, 0.0, 128.0, 128.0),
            input_handler: InputHandler::new(),
            level_data: LevelData::new(),
            resources: Resources::new(),
        };

        ms.resources.initialize(ctx)?;

        Ok(ms)
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        input: ggez::input::keyboard::KeyInput,
        _repeated: bool,
    ) -> Result<(), ggez::GameError> {
        println!("{}", input.scancode);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, None);

        self.level_data.draw(&mut canvas, &self.resources)?;

        canvas.set_screen_coordinates(self.screen_rect);

        canvas.finish(ctx)?;

        Ok(())
    }
}
