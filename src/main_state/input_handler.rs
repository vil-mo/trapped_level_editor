use super::instances::{
    collectible::{Collectible, CollectibleType},
    floor::{Floor, FloorType},
    object::{Object, ObjectType},
    wall::{WallData, WallType},
    ActivatingColor, Layer, LayerData,
};
use scancode::Scancode;

const OBJECT_ITEMS: [ObjectType; 3] = [
    ObjectType::Player,  //
    ObjectType::Box,     //
    ObjectType::TeleBox, //
];

const FLOOR_ITEMS: [FloorType; 3] = [
    FloorType::Normal,   //
    FloorType::Button,   //
    FloorType::Teleport, //
];

const WALL_ITEMS: [WallType; 1] = [
    WallType::Normal, //
];

const COLLECTIBLE_ITEMS: [CollectibleType; 1] = [
    CollectibleType::Win, //
];

#[non_exhaustive]
pub struct InputHandler {
    layer: Layer,

    object_item: ObjectType,
    floor_item: FloorType,
    wall_item: WallType,
    collectible_item: CollectibleType,

    color: ActivatingColor,

    wall_input_dependent: bool,
    wall_opened: bool,

    floor_durability: i32,
    waiting_for_durability_input: bool,
}

impl InputHandler {
    pub fn new() -> Self {
        InputHandler {
            layer: Layer::Object(()),

            object_item: OBJECT_ITEMS[0],
            floor_item: FLOOR_ITEMS[0],
            wall_item: WALL_ITEMS[0],
            collectible_item: COLLECTIBLE_ITEMS[0],

            color: ActivatingColor::None,

            wall_input_dependent: false,
            wall_opened: false,

            floor_durability: 1,
            waiting_for_durability_input: false,
        }
    }

    pub fn layer(&self) -> Layer {
        self.layer
    }

    pub fn handle_input(&mut self, input: ggez::input::keyboard::KeyInput) {
        let Some(scancode) = Scancode::new(input.scancode as u8) else {
            return;
        };

        if self.waiting_for_durability_input {
            match scancode {
                Scancode::Num1 => self.floor_durability = 1,
                Scancode::Num2 => self.floor_durability = 2,
                Scancode::Num3 => self.floor_durability = 3,
                Scancode::Num4 => self.floor_durability = 4,
                Scancode::Num5 => self.floor_durability = 5,
                Scancode::Num6 => self.floor_durability = 6,
                Scancode::Num7 => self.floor_durability = 7,
                Scancode::Num8 => self.floor_durability = 8,
                Scancode::Num9 => self.floor_durability = 9,

                _ => (),
            }

            self.waiting_for_durability_input = false;
            return;
        }

        match scancode {
            Scancode::Num1 => self.layer = Layer::Object(()),
            Scancode::Num2 => self.layer = Layer::Floor(()),
            Scancode::Num3 => self.layer = Layer::Wall(()),
            Scancode::Num4 => self.layer = Layer::Collectible(()),

            Scancode::Q => self.set_current_item(0),
            Scancode::W => self.set_current_item(1),
            Scancode::E => self.set_current_item(2),
            Scancode::R => self.set_current_item(3),
            Scancode::T => self.set_current_item(4),
            Scancode::Y => self.set_current_item(5),
            Scancode::U => self.set_current_item(6),
            Scancode::I => self.set_current_item(7),
            Scancode::O => self.set_current_item(8),
            Scancode::P => self.set_current_item(9),

            Scancode::A => self.color = ActivatingColor::None,
            Scancode::S => self.color = ActivatingColor::Red,
            Scancode::D => self.color = ActivatingColor::Blue,
            Scancode::F => self.color = ActivatingColor::Green,
            Scancode::G => self.color = ActivatingColor::Yellow,
            Scancode::H => self.color = ActivatingColor::Cyan,
            Scancode::J => self.color = ActivatingColor::Pink,

            Scancode::N => self.wall_input_dependent = !self.wall_input_dependent,
            Scancode::M => self.wall_opened = !self.wall_opened,

            Scancode::B => self.waiting_for_durability_input = true,

            _ => (),
        }
    }

    pub fn set_current_item(&mut self, index: usize) {
        match self.layer {
            Layer::Object(()) => {
                if let Some(item) = OBJECT_ITEMS.get(index) {
                    self.object_item = item.clone();
                }
            }

            Layer::Floor(()) => {
                if let Some(item) = FLOOR_ITEMS.get(index) {
                    self.floor_item = item.clone();
                }
            }

            Layer::Wall(()) => {
                if let Some(item) = WALL_ITEMS.get(index) {
                    self.wall_item = item.clone();
                }
            }

            Layer::Collectible(()) => {
                if let Some(item) = COLLECTIBLE_ITEMS.get(index) {
                    self.collectible_item = item.clone();
                }
            }
        }
    }

    pub fn get_data(&self) -> LayerData {
        match self.layer {
            Layer::Object(()) => {
                let object = Object::new(self.object_item, self.color);

                LayerData::Object(object)
            }

            Layer::Floor(()) => {
                let mut floor = Floor::new(self.floor_item, self.color);
                if let FloorType::Normal = floor.floor_type {
                    floor.durability = self.floor_durability;
                }

                LayerData::Floor(floor)
            }

            Layer::Wall(()) => {
                let wall = WallData::new(
                    self.wall_item,
                    self.color,
                    self.wall_input_dependent,
                    self.wall_opened,
                );

                LayerData::Wall(wall)
            }

            Layer::Collectible(()) => {
                let collectible = Collectible::new(self.collectible_item);

                LayerData::Collectible(collectible)
            }
        }
    }
}
