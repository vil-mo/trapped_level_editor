use std::path::Path;

use ggez::event;
use ggez::event::MouseButton;
use ggez::glam::IVec2;
use ggez::graphics;
use ggez::graphics::Color;
use ggez::graphics::Rect;
use ggez::graphics::Sampler;
use ggez::mint::Point2;
use ggez::Context;
use ggez::GameResult;

use self::input_handler::InputHandler;
use self::instances::wall::WallOrientation;
use self::instances::LayerContent;
use self::level_data::LevelData;
use self::resources::Resources;

mod input_handler;
mod instances;
mod level_data;
mod resources;
mod serialization;

pub struct MainState {
    screen_rect: Rect,
    input_handler: InputHandler,
    level_data: LevelData,
    resources: Resources,
}

impl MainState {
    pub fn new(ctx: &Context) -> GameResult<MainState> {
        let mut ms = MainState {
            screen_rect: Rect::new(0.0, 0.0, 128.0, 96.0),
            input_handler: InputHandler::new(),
            level_data: LevelData::new(),
            resources: Resources::new(),
        };

        ms.resources.initialize(ctx)?;

        Ok(ms)
    }

    const CELL_SIZE: i32 = 16;

    fn to_level_loader_coords(&self, ctx: &Context, coords: Point2<f32>) -> IVec2 {
        let (size_x, size_y) = ctx.gfx.size();
        let (size_x, size_y) = (self.screen_rect.w / size_x, self.screen_rect.h / size_y);

        let (x, y);
        unsafe {
            x = (size_x * coords.x / (Self::CELL_SIZE as f32))
                .floor()
                .to_int_unchecked();
            y = (size_y * coords.y / (Self::CELL_SIZE as f32))
                .floor()
                .to_int_unchecked();
        }

        IVec2 { x, y }
    }

    fn is_right(&self, ctx: &Context, coords: Point2<f32>) -> WallOrientation {
        let (size_x, size_y) = ctx.gfx.size();
        let (size_x, size_y) = (self.screen_rect.w / size_x, self.screen_rect.h / size_y);

        let x = size_x * coords.x % (Self::CELL_SIZE as f32);
        let y = size_y * coords.y % (Self::CELL_SIZE as f32);

        match Self::CELL_SIZE as f32 - x > Self::CELL_SIZE as f32 - y {
            true => WallOrientation::Right,
            false => WallOrientation::Down,
        }
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // Adding
        if ctx.mouse.button_pressed(MouseButton::Left) {
            let mouse_pos = ctx.mouse.position();
            let ll_coords = self.to_level_loader_coords(ctx, mouse_pos);

            self.level_data.insert(
                ll_coords,
                self.input_handler.get_data(),
                self.is_right(ctx, mouse_pos),
            )
        }

        // Deliting
        if ctx.mouse.button_pressed(MouseButton::Right) {
            let mouse_pos = ctx.mouse.position();

            self.level_data.remove(
                self.to_level_loader_coords(ctx, mouse_pos),
                self.input_handler.layer(),
                self.is_right(ctx, mouse_pos),
            )
        }

        Ok(())
    }

    fn mouse_wheel_event(
        &mut self,
        _ctx: &mut Context,
        _x: f32,
        y: f32,
    ) -> Result<(), ggez::GameError> {
        let scale_factor = 1.1_f32.powf(-y);
        self.screen_rect.scale(scale_factor, scale_factor);

        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        input: ggez::input::keyboard::KeyInput,
        _repeated: bool,
    ) -> Result<(), ggez::GameError> {
        self.input_handler.handle_input(input);

        if self.input_handler.request_save {
            serialization::save(
                &self.level_data,
                Path::new("/home/vilmo/Documents/trapped/level"),
            )?;
            self.input_handler.request_save = false;
        }
        if self.input_handler.request_load {
            self.level_data =
                serialization::load(Path::new("/home/vilmo/Documents/trapped/level"))?;
            self.input_handler.request_load = false;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Some(Color::BLACK));
        canvas.set_screen_coordinates(self.screen_rect);
        canvas.set_sampler(Sampler::nearest_clamp());

        let mouse_pos = ctx.mouse.position();

        let cntntn =
            LayerContent::new(self.input_handler.get_data(), self.is_right(ctx, mouse_pos));

        self.level_data.draw_with(
            cntntn,
            &self.to_level_loader_coords(ctx, mouse_pos),
            &mut canvas,
            &self.resources,
        )?;

        canvas.finish(ctx)?;

        Ok(())
    }
}
