use super::ActivatingColor;

#[derive(Debug, Clone, Copy)]
pub enum FloorType {
    Normal,
    Button,
    Teleport,
}

#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct Floor {
    pub floor_type: FloorType,
    pub color: ActivatingColor,
    pub durability: i32,
}

impl Floor {
    pub fn new(floor_type: FloorType, color: ActivatingColor) -> Floor {
        Floor {
            floor_type,
            color,
            durability: -1,
        }
    }

    pub fn new_with_dur(floor_type: FloorType, color: ActivatingColor, durability: i32) -> Floor {
        Floor {
            floor_type,
            color,
            durability,
        }
    }
}
