use super::ActivatingColor;

#[derive(Debug, Clone, Copy)]
pub enum WallType {
    Normal,
}

#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct WallData {
    pub wall_type: WallType,
    pub color: ActivatingColor,
    pub input_dependent: bool,
    pub opened: bool,
}

impl WallData {
    pub fn new(
        wall_type: WallType,
        color: ActivatingColor,
        input_dependent: bool,
        opened: bool,
    ) -> WallData {
        WallData {
            wall_type,
            color,
            input_dependent,
            opened,
        }
    }

    pub fn default(wall_type: WallType) -> WallData {
        WallData {
            wall_type,
            color: ActivatingColor::None,
            input_dependent: false,
            opened: false,
        }
    }
}

pub enum WallOrientation {
    Right,
    Down,
}

#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct Wall {
    pub down: Option<WallData>,
    pub right: Option<WallData>,
}

impl Wall {
    pub fn new() -> Wall {
        Wall {
            down: None,
            right: None,
        }
    }

    pub fn merge(&mut self, other: Wall) {
        if let Some(data) = other.right {
            self.right = Some(data);
        }
        if let Some(data) = other.down {
            self.down = Some(data);
        }
    }

    pub fn merge_data(&mut self, other: WallData, orientation: WallOrientation) {
        match orientation {
            WallOrientation::Right => self.right = Some(other),
            WallOrientation::Down => self.down = Some(other),
        }
    }
}
