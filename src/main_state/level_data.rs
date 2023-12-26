use std::collections::HashMap;

use ggez::{
    glam::IVec2,
    graphics::{Canvas, DrawParam},
    mint::Point2,
    Context, GameResult,
};

use super::{
    instances::{
        collectible::Collectible,
        floor::Floor,
        object::Object,
        wall::{Wall, WallOrientation},
        Layer, LayerContent, LayerData,
    },
    resources::Resources,
};

#[derive(Debug, Default)]
pub struct LevelData {
    pub objects: HashMap<IVec2, Object>,
    pub walls: HashMap<IVec2, Wall>,
    pub floors: HashMap<IVec2, Floor>,
    pub collectibles: HashMap<IVec2, Collectible>,
}

impl LevelData {
    pub fn new() -> LevelData {
        LevelData::default()
    }
    fn gen_draw_param(pos: &IVec2) -> DrawParam {
        let draw_param = DrawParam::new();
        draw_param.dest(Point2 {
            x: (pos.x as f32) * 16.0,
            y: (pos.y as f32) * 16.0,
        })
    }
    //
    // pub fn draw(&self, canvas: &mut Canvas, resources: &Resources) -> GameResult {
    //     for (pos, floor) in &self.floors {
    //         let draw_param = Self::gen_draw_param(&pos);
    //         resources.draw_content(canvas, LayerContent::Floor(floor.clone()), draw_param)?;
    //     }
    //
    //     for (pos, collectible) in &self.collectibles {
    //         let draw_param = Self::gen_draw_param(&pos);
    //         resources.draw_content(
    //             canvas,
    //             LayerContent::Collectible(collectible.clone()),
    //             draw_param,
    //         )?;
    //     }
    //
    //     for (pos, object) in &self.objects {
    //         let draw_param = Self::gen_draw_param(&pos);
    //         resources.draw_content(canvas, LayerContent::Object(object.clone()), draw_param)?;
    //     }
    //
    //     for (pos, wall) in &self.walls {
    //         let draw_param = Self::gen_draw_param(&pos);
    //         resources.draw_content(canvas, LayerContent::Wall(wall.clone()), draw_param)?;
    //     }
    //
    //     Ok(())
    // }
    //
    pub fn draw_with(
        &self,
        ctx: &Context,
        data_with: LayerContent,
        pos_with: &IVec2,
        canvas: &mut Canvas,
        resources: &Resources,
    ) -> GameResult {
        let mut consumed = false;

        for (pos, floor) in &self.floors {
            let mut content = floor.clone();
            if pos == pos_with {
                if let LayerContent::Floor(dt) = data_with.clone() {
                    content = dt;
                    consumed = true;
                }
            }
            let draw_param = Self::gen_draw_param(&pos);
            resources.draw_content(ctx, canvas, LayerContent::Floor(content), draw_param)?;
        }

        if !consumed {
            if let LayerContent::Floor(_) = data_with {
                let draw_param = Self::gen_draw_param(&pos_with);
                resources.draw_content(ctx, canvas, data_with.clone(), draw_param)?;
            }
        }

        for (pos, collectible) in &self.collectibles {
            let mut content = collectible.clone();
            if pos == pos_with {
                if let LayerContent::Collectible(dt) = data_with.clone() {
                    content = dt;
                    consumed = true;
                }
            }
            let draw_param = Self::gen_draw_param(&pos);
            resources.draw_content(ctx, canvas, LayerContent::Collectible(content), draw_param)?;
        }

        if !consumed {
            if let LayerContent::Collectible(_) = data_with {
                let draw_param = Self::gen_draw_param(&pos_with);
                resources.draw_content(ctx, canvas, data_with.clone(), draw_param)?;
            }
        }

        for (pos, object) in &self.objects {
            let mut content = object.clone();
            if pos == pos_with {
                if let LayerContent::Object(dt) = data_with.clone() {
                    content = dt;
                    consumed = true;
                }
            }
            let draw_param = Self::gen_draw_param(&pos);
            resources.draw_content(ctx, canvas, LayerContent::Object(content), draw_param)?;
        }

        if !consumed {
            if let LayerContent::Object(_) = data_with {
                let draw_param = Self::gen_draw_param(&pos_with);
                resources.draw_content(ctx, canvas, data_with.clone(), draw_param)?;
            }
        }

        for (pos, wall) in &self.walls {
            let mut content = wall.clone();
            if pos == pos_with {
                if let LayerContent::Wall(dt) = data_with.clone() {
                    content.merge(dt);
                    consumed = true;
                }
            }
            let draw_param = Self::gen_draw_param(&pos);
            resources.draw_content(ctx, canvas, LayerContent::Wall(content), draw_param)?;
        }

        if !consumed {
            if let LayerContent::Wall(_) = data_with {
                let draw_param = Self::gen_draw_param(&pos_with);
                resources.draw_content(ctx, canvas, data_with.clone(), draw_param)?;
            }
        }

        Ok(())
    }

    pub fn insert(&mut self, pos: IVec2, data: LayerData, orientation: WallOrientation) {
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

                self.walls
                    .get_mut(&pos)
                    .unwrap()
                    .merge_data(wall_data, orientation);
            }

            LayerData::Collectible(collectible) => {
                self.collectibles.insert(pos, collectible);
            }
        };
    }

    pub fn remove(
        &mut self,
        pos: IVec2,
        layer: Layer<(), (), (), ()>,
        orientation: WallOrientation,
    ) {
        match layer {
            Layer::Object(()) => {
                self.objects.remove(&pos);
            }

            Layer::Floor(()) => {
                self.floors.remove(&pos);
            }

            Layer::Wall(()) => {
                self.walls.get_mut(&pos).map(|wall| {
                    match orientation {
                        WallOrientation::Right => wall.right = None,
                        WallOrientation::Down => wall.down = None,
                    };
                });
            }

            Layer::Collectible(()) => {
                self.collectibles.remove(&pos);
            }
        }
    }
}
