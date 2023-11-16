use super::Colors;

#[derive(Debug, Clone)]
pub struct WallOrientation {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}


#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct Wall {
    pub color: Colors,
    pub input_dependent: bool,
    pub orientation: WallOrientation,
    pub opened: bool,
}
