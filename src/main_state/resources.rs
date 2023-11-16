use std::collections::HashMap;

use ggez::{
    graphics::{Canvas, DrawParam, Image, Rect},
    Context, GameError, GameResult,
};

#[derive(Debug, Hash, PartialEq, Eq)]
enum Images {
    Ghost,
    BaseObj,
    Buttons,
    Walls,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum DrawId {
    Placeholder,

    Ghost,
    Box,
    
    Floor,
    Floor2,
    Floor3,
    Button,
    Teleport,

    HorizontalWallOpened,
    HorizontalWallClosed,
    VerticalWallOpened,
    VerticalWallClosed,
    
    Win,
}

#[derive(Debug, Default)]
pub struct Resources {
    images: HashMap<Images, Image>,
    draw_id: HashMap<DrawId, (Images, Rect)>,
}

impl Resources {
    pub fn new() -> Resources {
        Resources::default()
    }

    pub fn initialize(&mut self, ctx: &Context) -> GameResult {
        let ghost = Image::from_path(ctx, "GhostSprite.png")?;
        let base_obj = Image::from_path(ctx, "BaseObj.png")?;
        let buttons = Image::from_path(ctx, "Buttons.png")?;
        let walls = Image::from_path(ctx, "Walls.png")?;

        self.images.insert(Images::Ghost, ghost);
        self.images.insert(Images::BaseObj, base_obj);
        self.images.insert(Images::Buttons, buttons);
        self.images.insert(Images::Walls, walls);

        self.draw_id.insert(
            DrawId::Ghost,
            (Images::Ghost, Rect::new(0.0, 0.0, 16.0, 16.0)),
        );
        self.draw_id.insert(
            DrawId::Box,
            (Images::BaseObj, Rect::new(16.0, 0.0, 16.0, 16.0)),
        );
        self.draw_id.insert(
            DrawId::Button,
            (Images::Buttons, Rect::new(0.0, 0.0, 16.0, 16.0)),
        );
        self.draw_id.insert(
            DrawId::Floor,
            (Images::BaseObj, Rect::new(0.0, 0.0, 16.0, 16.0)),
        );
        self.draw_id.insert(
            DrawId::Floor2,
            (Images::BaseObj, Rect::new(32.0, 0.0, 16.0, 16.0)),
        );
        self.draw_id.insert(
            DrawId::Floor3,
            (Images::BaseObj, Rect::new(48.0, 0.0, 16.0, 16.0)),
        );
        self.draw_id.insert(
            DrawId::HorizontalWallOpened,
            (Images::Walls, Rect::new(0.0, 0.0, 32.0, 32.0)),
        );
        self.draw_id.insert(
            DrawId::HorizontalWallClosed,
            (Images::Walls, Rect::new(32.0, 0.0, 32.0, 32.0)),
        );
        self.draw_id.insert(
            DrawId::VerticalWallOpened,
            (Images::Walls, Rect::new(64.0, 0.0, 32.0, 32.0)),
        );
        self.draw_id.insert(
            DrawId::VerticalWallClosed,
            (Images::Walls, Rect::new(96.0, 0.0, 32.0, 32.0)),
        );

        Ok(())
    }

    pub fn draw_drawing(
        &self,
        canvas: &mut Canvas,
        draw_id: DrawId,
        mut draw_param: DrawParam,
    ) -> GameResult {
        let (image, src_rect) =
            self.draw_id
                .get(&draw_id)
                .ok_or(GameError::CustomError(format!(
                    "Draw ID is missing, {:?}",
                    draw_id
                )))?;

        let image = self
            .images
            .get(image)
            .ok_or(GameError::CustomError(format!(
                "Image is missing, {:?}",
                image
            )))?;
        
        draw_param.src = src_rect.clone();
        canvas.draw(image, draw_param);
        
        Ok(())
    }
}
