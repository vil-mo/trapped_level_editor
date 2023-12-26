use super::instances::{
    collectible::{Collectible, CollectibleType},
    floor::{Floor, FloorType},
    object::{Object, ObjectType},
    wall::{WallData, WallType},
    ActivatingColor, Layer, LayerData,
};
use ggez::winit::event::VirtualKeyCode;

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

    pub request_save: bool,
    pub request_load: bool,
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

            request_save: false,
            request_load: false,
        }
    }

    pub fn layer(&self) -> Layer {
        self.layer
    }

    pub fn handle_input(&mut self, input: ggez::input::keyboard::KeyInput) {
        println!(
            "value: {:?}, scancode: {}, keycode: {:?}",
            "nowe", input.scancode, input.keycode
        );

        let Some(keycode) = input.keycode else {
            return;
        };

        if self.waiting_for_durability_input {
            match keycode {
                VirtualKeyCode::Key1 => self.floor_durability = 1,
                VirtualKeyCode::Key2 => self.floor_durability = 2,
                VirtualKeyCode::Key3 => self.floor_durability = 3,
                VirtualKeyCode::Key4 => self.floor_durability = 4,
                VirtualKeyCode::Key5 => self.floor_durability = 5,
                VirtualKeyCode::Key6 => self.floor_durability = 6,
                VirtualKeyCode::Key7 => self.floor_durability = 7,
                VirtualKeyCode::Key8 => self.floor_durability = 8,
                VirtualKeyCode::Key9 => self.floor_durability = 9,

                _ => (),
            }

            self.waiting_for_durability_input = false;
            return;
        }

        match keycode {
            VirtualKeyCode::Key1 => self.layer = Layer::Object(()),
            VirtualKeyCode::Key2 => self.layer = Layer::Floor(()),
            VirtualKeyCode::Key3 => self.layer = Layer::Wall(()),
            VirtualKeyCode::Key4 => self.layer = Layer::Collectible(()),

            VirtualKeyCode::Q => self.set_current_item(0),
            VirtualKeyCode::W => self.set_current_item(1),
            VirtualKeyCode::E => self.set_current_item(2),
            VirtualKeyCode::R => self.set_current_item(3),
            VirtualKeyCode::T => self.set_current_item(4),
            VirtualKeyCode::Y => self.set_current_item(5),
            VirtualKeyCode::U => self.set_current_item(6),
            VirtualKeyCode::I => self.set_current_item(7),
            VirtualKeyCode::O => self.set_current_item(8),
            VirtualKeyCode::P => self.set_current_item(9),

            VirtualKeyCode::A => self.color = ActivatingColor::None,
            VirtualKeyCode::S => self.color = ActivatingColor::Red,
            VirtualKeyCode::D => self.color = ActivatingColor::Blue,
            VirtualKeyCode::F => self.color = ActivatingColor::Green,
            VirtualKeyCode::G => self.color = ActivatingColor::Yellow,
            VirtualKeyCode::H => self.color = ActivatingColor::Cyan,
            VirtualKeyCode::J => self.color = ActivatingColor::Pink,

            VirtualKeyCode::N => self.wall_input_dependent = !self.wall_input_dependent,
            VirtualKeyCode::M => self.wall_opened = !self.wall_opened,

            VirtualKeyCode::B => self.waiting_for_durability_input = true,

            VirtualKeyCode::Escape => self.request_save = true,
            VirtualKeyCode::Space => self.request_load = true,

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
                let collectible = Collectible::new(self.collectible_item, self.color);

                LayerData::Collectible(collectible)
            }
        }
    }
}
