use super::instances::{
    collectible::{Collectible, CollectibleType},
    floor::{Floor, FloorType},
    object::{Object, ObjectType},
    wall::{WallData, WallType},
    ActivatingColor, Item, Layer, LayerContent,
};

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
        }
    }

    pub fn get_content(&self) -> LayerContent {
        match self.layer {
            Layer::Object(()) => {
                let object = Object::new(self.object_item, self.color);

                LayerContent::Object(object)
            }

            Layer::Floor(()) => {
                let floor = Floor::new(self.floor_item, self.color);

                LayerContent::Floor(floor)
            }

            Layer::Wall(()) => {
                let wall = WallData::new(
                    self.wall_item,
                    self.color,
                    self.wall_input_dependent,
                    self.wall_opened,
                );

                LayerContent::Wall(wall)
            }

            Layer::Collectible(()) => {
                let collectible = Collectible::new(self.collectible_item);

                LayerContent::Collectible(collectible)
            }
        }
    }
}
