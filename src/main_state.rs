use ggez::GameResult;
use ggez::event;
use ggez::Context;
use ggez::graphics;

pub struct MainState {

}

impl MainState {
    pub fn new() -> GameResult<MainState> {
        let ms = MainState {};

        Ok(ms)
    }
}



impl event::EventHandler<ggez::GameError> for MainState {

    fn update(&mut self, _ctx: &mut Context) -> GameResult {


        Ok(())
    }



    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let canvas = graphics::Canvas::from_frame(ctx, None);

        


        canvas.finish(ctx)?;

        Ok(())
    }
}


