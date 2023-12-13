use std::collections::HashMap;

use ggez::{
    graphics::{Canvas, DrawParam, Image, Rect},
    Context, GameError, GameResult,
};

use super::instances::{
    collectible::CollectibleType, floor::FloorType, object::ObjectType, wall::Wall, LayerContent,
};

#[derive(Debug, Hash, PartialEq, Eq)]
enum Images {
    Box,
    Button,
    Floor,
    Ghost,
    Wall,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum DrawId {
    Ghost,
    Box,
    TeleBox,

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
        let box_img = Image::from_path(ctx, "/Box.png")?;
        let button_img = Image::from_path(ctx, "/Button.png")?;
        let floor_img = Image::from_path(ctx, "/Floor.png")?;
        let ghost_img = Image::from_path(ctx, "/Ghost.png")?;
        let wall_img = Image::from_path(ctx, "/Wall.png")?;

        self.images.insert(Images::Box, box_img);
        self.images.insert(Images::Button, button_img);
        self.images.insert(Images::Floor, floor_img);
        self.images.insert(Images::Ghost, ghost_img);
        self.images.insert(Images::Wall, wall_img);

        self.draw_id.insert(
            DrawId::Ghost,
            (Images::Ghost, Rect::new(0.0, 0.0, 16.0, 16.0)),
        );
        self.draw_id
            .insert(DrawId::Box, (Images::Box, Rect::new(0.0, 0.0, 16.0, 16.0)));
        // self.draw_id.insert(
        //     DrawId::TeleBox,
        //     ()
        // );

        self.draw_id.insert(
            DrawId::Floor,
            (Images::Floor, Rect::new(0.0, 0.0, 16.0, 16.0)),
        );
        self.draw_id.insert(
            DrawId::Floor2,
            (Images::Floor, Rect::new(0.0, 16.0, 16.0, 16.0)),
        );
        self.draw_id.insert(
            DrawId::Floor3,
            (Images::Floor, Rect::new(0.0, 32.0, 16.0, 16.0)),
        );

        self.draw_id.insert(
            DrawId::Button,
            (Images::Button, Rect::new(0.0, 0.0, 16.0, 16.0)),
        );
        // self.draw_id.insert(
        //     DrawId::Teleport,
        //     ()
        // )

        self.draw_id.insert(
            DrawId::HorizontalWallOpened,
            (Images::Wall, Rect::new(0.0, 0.0, 32.0, 32.0)),
        );
        self.draw_id.insert(
            DrawId::HorizontalWallClosed,
            (Images::Wall, Rect::new(32.0, 0.0, 32.0, 32.0)),
        );
        self.draw_id.insert(
            DrawId::VerticalWallOpened,
            (Images::Wall, Rect::new(64.0, 0.0, 32.0, 32.0)),
        );
        self.draw_id.insert(
            DrawId::VerticalWallClosed,
            (Images::Wall, Rect::new(96.0, 0.0, 32.0, 32.0)),
        );

        // self.draw_id.insert(
        //     DrawId::Win,
        //     ()
        // )

        Ok(())
    }

    pub fn draw_content(
        &self,
        canvas: &mut Canvas,
        content: LayerContent,
        draw_param: DrawParam,
    ) -> GameResult {
        let draw_id = match content {
            LayerContent::Object(obj) => match obj.object_type {
                ObjectType::Player => DrawId::Ghost,
                ObjectType::Box => DrawId::Box,
                ObjectType::TeleBox => DrawId::TeleBox,
            },

            LayerContent::Floor(flr) => match flr.floor_type {
                FloorType::Normal => match flr.durability {
                    1 => DrawId::Floor,
                    2 => DrawId::Floor2,
                    3 => DrawId::Floor3,

                    _ => return Ok(()),
                },

                FloorType::Button => DrawId::Button,
                FloorType::Teleport => DrawId::Teleport,
            },

            super::instances::Layer::Wall(wl) => {
                self.draw_wall(canvas, wl, draw_param)?;

                return Ok(());
            }

            LayerContent::Collectible(clct) => match clct.collectible_type {
                CollectibleType::Win => DrawId::Win,
            },
        };

        self.draw_drawing(canvas, draw_id, draw_param)?;

        Ok(())
    }

    pub fn draw_wall(&self, canvas: &mut Canvas, wall: Wall, draw_param: DrawParam) -> GameResult {
        let draw_id_option = wall.right.map(|wl| match wl.opened {
            true => DrawId::VerticalWallOpened,
            false => DrawId::VerticalWallClosed,
        });

        if let Some(draw_id) = draw_id_option {
            self.draw_drawing(canvas, draw_id, draw_param)?;
        }

        let draw_id_option = wall.down.map(|wl| match wl.opened {
            true => DrawId::HorizontalWallOpened,
            false => DrawId::HorizontalWallClosed,
        });

        if let Some(draw_id) = draw_id_option {
            self.draw_drawing(canvas, draw_id, draw_param)?;
        }

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
                    "Draw ID does not added in Resources, {:?}",
                    draw_id
                )))?;

        let image = self
            .images
            .get(image)
            .ok_or(GameError::CustomError(format!(
                "Image does not added in Resources, {:?}",
                image
            )))?;

        draw_param.src = src_rect.clone();
        canvas.draw(image, draw_param);

        Ok(())
    }
}
