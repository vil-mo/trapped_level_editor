use self::{
    collectible::Collectible,
    floor::Floor,
    object::Object,
    wall::{Wall, WallData},
};

pub mod collectible;
pub mod floor;
pub mod object;
pub mod wall;

#[derive(Debug, Clone, Copy)]
pub enum ActivatingColor {
    None,
    Green,
    Blue,
    Red,
    Yellow,
    Cyan,
    Pink,
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
