use std::{collections::HashMap, path::Path, convert::Infallible};

use ggez::{
    glam::IVec2,
    graphics::{Canvas, DrawParam},
    mint::Point2,
    GameResult,
};

use super::{
    instances::{
        collectible::Collectible, floor::Floor, object::Object, wall::Wall, Layer, LayerContent,
        LayerData,
    },
    resources::Resources,
};

#[derive(Debug, Default)]
pub struct LevelData {
    objects: HashMap<IVec2, Object>,
    walls: HashMap<IVec2, Wall>,
    floors: HashMap<IVec2, Floor>,
    collectibles: HashMap<IVec2, Collectible>,
}

impl LevelData {
    pub fn new() -> LevelData {
        LevelData::default()
    }

    fn write_line(content: & mut String, name: &str, pos: IVec2, suffix: &str, properties: &[(&str, &str)]) {
        content.push_str(name);
        content.push(' ');

        content.push_str(&pos.x.to_string());
        content.push(',');
        content.push_str(&pos.y.to_string());
        content.push(' ');

        content.push_str(suffix);

        for (key, val) in properties {
            content.push(' ');
            content.push_str(key);
            content.push(':');
            content.push_str(val);
        }

        content.push('\n');
    }

    pub fn save(&self, path: &Path) -> GameResult {
        let mut contents = String::new();
        let mut dimentions = IVec2::new(0, 0);

        for (pos, floor) in &self.floors {
            if pos.x > dimentions.x {
                dimentions.x = pos.x;
            }
            if pos.y > dimentions.y {
                dimentions.y = pos.y;
            }
            

            


        }


        std::fs::write(path, contents)?;

        Ok(())
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
            resources.draw_content(canvas, LayerContent::Floor(floor.clone()), draw_param)?;
        }

        for (pos, collectible) in &self.collectibles {
            let draw_param = Self::gen_draw_param(&pos);
            resources.draw_content(
                canvas,
                LayerContent::Collectible(collectible.clone()),
                draw_param,
            )?;
        }

        for (pos, object) in &self.objects {
            let draw_param = Self::gen_draw_param(&pos);
            resources.draw_content(canvas, LayerContent::Object(object.clone()), draw_param)?;
        }

        for (pos, wall) in &self.walls {
            let draw_param = Self::gen_draw_param(&pos);
            resources.draw_content(canvas, LayerContent::Wall(wall.clone()), draw_param)?;
        }

        Ok(())
    }

    pub fn get(&self, pos: IVec2, on_layer: Layer) -> Option<LayerContent> {
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
            Layer::Collectible(()) => self
                .collectibles
                .get(&pos)
                .map(|val| LayerContent::Collectible(val.clone())),
        }
    }

    pub fn insert(&mut self, pos: IVec2, data: LayerData, right: bool) {
        match data {
            LayerData::Object(object) => {
                self.objects.insert(pos, object);
            }

            LayerData::Floor(floor) => {
                self.floors.insert(pos, floor);
            }

            LayerData::Wall(wall_data) => {
                if let None = self.walls.get(&pos) {
                    self.walls.insert(pos.clone(), Wall::new());
                }

                let mut wall;
                if right {
                    wall = Wall::new();
                    wall.right = Some(wall_data);
                } else {
                    wall = Wall::new();
                    wall.down = Some(wall_data);
                }

                self.walls.get_mut(&pos).unwrap().merge(wall);
            }

            LayerData::Collectible(collectible) => {
                self.collectibles.insert(pos, collectible);
            }
        };
    }

    pub fn remove(&mut self, pos: IVec2, layer: Layer<(), (), (), ()>, right: bool) {
        match layer {
            Layer::Object(()) => {
                self.objects.remove(&pos);
            }

            Layer::Floor(()) => {
                self.floors.remove(&pos);
            }

            Layer::Wall(()) => {
                self.walls.get_mut(&pos).map(|wall| {
                    if right {
                        wall.right = None;
                    } else {
                        wall.down = None;
                    };
                });
            }

            Layer::Collectible(()) => {
                self.collectibles.remove(&pos);
            }
        }
    }
}
