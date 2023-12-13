use ggez::event;
use ggez::event::MouseButton;
use ggez::glam::IVec2;
use ggez::graphics;
use ggez::graphics::Rect;
use ggez::mint::Point2;
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

    fn to_level_loader_coords(coords: Point2<f32>) -> IVec2 {
        let x;
        let y;
        unsafe {
            x = (coords.x / 32.0).to_int_unchecked();
            y = (coords.y / 32.0).to_int_unchecked();
        }

        IVec2 { x, y }
    }

    fn is_right(coords: Point2<f32>) -> bool {
        coords.x % 32.0 > coords.y % 32.0
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if ctx.mouse.button_pressed(MouseButton::Left) {
            let mouse_pos = ctx.mouse.position();

            self.level_data.insert(
                Self::to_level_loader_coords(mouse_pos),
                self.input_handler.get_data(),
                Self::is_right(mouse_pos),
            )
        }

        if ctx.mouse.button_pressed(MouseButton::Right) {
            let mouse_pos = ctx.mouse.position();

            self.level_data.remove(
                Self::to_level_loader_coords(mouse_pos),
                self.input_handler.layer(),
                Self::is_right(mouse_pos),
            )
        }

        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        input: ggez::input::keyboard::KeyInput,
        _repeated: bool,
    ) -> Result<(), ggez::GameError> {
        self.input_handler.handle_input(input);

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
