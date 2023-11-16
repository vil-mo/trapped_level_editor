use ggez::event;
use ggez::graphics;
use ggez::Context;
use ggez::GameResult;

mod object;
use ggez::graphics::Rect;
use object::Object;
mod wall;
use wall::Wall;
mod floor;
use floor::Floor;
mod win;
use win::Win;

use self::floor::FloorType;
use self::level_data::LevelData;
use self::object::ObjectType;
use self::resources::Resources;

mod level_data;
mod resources;

#[derive(Debug, Clone)]
pub enum Colors {
    None,
    Green,
    Blue,
    Red,
    Yellow,
    Cyan,
    Gray,
}

#[derive(Debug, Clone)]
pub enum Layer<O, F, L, W> {
    Object(O),
    Floor(F),
    Wall(L),
    Win(W),
}

type LayerContent = Layer<Object, Floor, Wall, Win>;
type Tool = Layer<ObjectType, FloorType, Wall, ()>;

pub struct MainState {
    screen_rect: Rect,
    level_data: LevelData,
    resources: Resources,
}

impl MainState {
    pub fn new(ctx: &Context) -> GameResult<MainState> {
        let mut ms = MainState {
            screen_rect: Rect::new(0.0, 0.0, 128.0, 128.0),
            level_data: LevelData::new(),
            resources: Resources::new(),
        };

        ms.resources.initialize(ctx);

        Ok(ms)
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
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
