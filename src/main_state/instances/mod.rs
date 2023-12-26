use self::{
    collectible::Collectible,
    floor::Floor,
    object::Object,
    wall::{Wall, WallData, WallOrientation},
};

pub mod collectible;
pub mod floor;
pub mod object;
pub mod wall;

#[derive(Debug, Clone, Copy)]
pub enum ActivatingColor {
    None,
    Red,
    Blue,
    Green,
    Yellow,
    Cyan,
    Pink,
}

impl ToString for ActivatingColor {
    fn to_string(&self) -> String {
        match self {
            ActivatingColor::None => String::from("n"),
            ActivatingColor::Red => String::from("r"),
            ActivatingColor::Blue => String::from("b"),
            ActivatingColor::Green => String::from("g"),
            ActivatingColor::Yellow => String::from("y"),
            ActivatingColor::Cyan => String::from("c"),
            ActivatingColor::Pink => String::from("p"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Layer<O = (), F = (), W = (), C = ()> {
    Object(O),
    Floor(F),
    Wall(W),
    Collectible(C),
}

pub type LayerData = Layer<Object, Floor, WallData, Collectible>;
pub type LayerContent = Layer<Object, Floor, Wall, Collectible>;

impl LayerContent {
    pub fn new(data: LayerData, orientation: WallOrientation) -> LayerContent {
        match data {
            LayerData::Floor(d) => LayerContent::Floor(d),
            LayerData::Object(d) => LayerContent::Object(d),
            LayerData::Collectible(d) => LayerContent::Collectible(d),

            LayerData::Wall(d) => {
                let mut wall = Wall::new();
                wall.merge_data(d, orientation);
                LayerContent::Wall(wall)
            }
        }
    }
}
