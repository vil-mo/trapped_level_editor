use super::Colors;


#[derive (Debug, Clone)]
pub enum FloorType {
    Normal,
    Button,
    Teleport,
}


#[non_exhaustive]
#[derive (Debug, Clone)]
pub struct Floor {
    pub floor_type: FloorType,
    pub color: Colors,
}

impl Floor {
    pub fn new(floor_type: FloorType, color: Colors) -> Floor {
        Floor { floor_type, color }
    }
}