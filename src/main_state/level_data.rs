use std::{collections::HashMap, path::Path};

use ggez::{
    glam::IVec2,
    graphics::{Canvas, DrawParam},
    mint::Point2,
    GameResult,
};

use super::{
    floor::{Floor, FloorType},
    object::{Object, ObjectType},
    resources::{DrawId, Resources},
    wall::Wall,
    win::Win,
    Colors, Layer, LayerContent, Tool,
};

#[derive(Debug, Default)]
pub struct LevelData {
    objects: HashMap<IVec2, Object>,
    walls: HashMap<IVec2, Wall>,
    floors: HashMap<IVec2, Floor>,
    wins: HashMap<IVec2, Win>,
}

impl LevelData {
    pub fn new() -> LevelData {
        LevelData::default()
    }

    pub fn load(path: &Path) -> LevelData {
        todo!()
    }

    fn gen_draw_param(pos: &IVec2) -> DrawParam {
        let draw_param = DrawParam::new();
        draw_param.offset(Point2 {
            x: (pos.x as f32) * 16.0,
            y: (pos.y as f32) * 16.0,
        });

        draw_param
    }

    pub fn draw(&self, canvas: &mut Canvas, resources: &Resources) -> GameResult {
        for (pos, floor) in &self.floors {
            let draw_param = Self::gen_draw_param(&pos);

            let draw_id = match floor.floor_type {
                FloorType::Normal => DrawId::Floor,
                FloorType::Button => DrawId::Button,

                _ => DrawId::Placeholder,
            };

            resources.draw_drawing(canvas, draw_id, draw_param)?;
        }

        for (pos, win) in &self.wins {
            let draw_param = Self::gen_draw_param(&pos);

            let draw_id = DrawId::Win;

            resources.draw_drawing(canvas, draw_id, draw_param)?;
        }

        for (pos, object) in &self.objects {
            let draw_param = Self::gen_draw_param(&pos);

            let draw_id = match object.object_type {
                ObjectType::Player => DrawId::Ghost,
                ObjectType::Box => DrawId::Box,
            };

            resources.draw_drawing(canvas, draw_id, draw_param)?;
        }

        for (pos, wall) in &self.walls {
            let draw_param = Self::gen_draw_param(&pos);

            
            if wall.orientation.down {
                match wall.opened {
                    true => resources.draw_drawing(canvas, DrawId::VerticalWallOpened, draw_param.clone())?,
                    false => resources.draw_drawing(canvas, DrawId::VerticalWallClosed, draw_param.clone())?,
                }
            }

            if wall.orientation.right {
                match wall.opened {
                    true => resources.draw_drawing(canvas, DrawId::HorizontalWallOpened, draw_param.clone())?,
                    false => resources.draw_drawing(canvas, DrawId::HorizontalWallClosed, draw_param.clone())?,
                }
            }
        }

        Ok(())
    }

    pub fn get(&self, pos: IVec2, on_layer: Layer<(), (), (), ()>) -> Option<LayerContent> {
        match on_layer {
            Layer::Object(()) => self
                .objects
                .get(&pos)
                .map(|val| LayerContent::Object(val.clone())),
            Layer::Wall(()) => self
                .walls
                .get(&pos)
                .map(|val| LayerContent::Wall(val.clone())),
            Layer::Floor(()) => self
                .floors
                .get(&pos)
                .map(|val| LayerContent::Floor(val.clone())),
            Layer::Win(()) => self
                .wins
                .get(&pos)
                .map(|val| LayerContent::Win(val.clone())),
        }
    }

    pub fn insert(&mut self, pos: IVec2, data: Tool, color: Option<Colors>) {
        match data {
            Tool::Object(object_type) => {
                self.objects
                    .insert(pos, Object::new(object_type, color.unwrap_or(Colors::None)));
            }

            Tool::Floor(floor_type) => {
                self.floors
                    .insert(pos, Floor::new(floor_type, color.unwrap_or(Colors::None)));
            }

            Tool::Wall(wall) => {
                self.walls.insert(pos, wall.clone());
            }

            Tool::Win(_) => {
                self.wins.insert(pos, Win::new());
            }
        };
    }

    pub fn remove(&mut self, pos: IVec2, layer: Layer<(), (), (), ()>) {
        match layer {
            Layer::Object(()) => {
                self.objects.remove(&pos);
            }

            Layer::Floor(()) => {
                self.floors.remove(&pos);
            }

            Layer::Wall(()) => {
                self.walls.remove(&pos);
            }

            Layer::Win(()) => {
                self.wins.remove(&pos);
            }
        }
    }
}
