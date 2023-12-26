use super::ActivatingColor;

#[derive(Debug, Clone, Copy)]
pub enum CollectibleType {
    Win,
}

#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct Collectible {
    pub collectible_type: CollectibleType,
    pub color: ActivatingColor,
}

impl Collectible {
    pub fn new(collectible_type: CollectibleType, color: ActivatingColor) -> Collectible {
        Collectible {
            collectible_type,
            color,
        }
    }

    pub fn default(collectible_type: CollectibleType) -> Collectible {
        Collectible {
            collectible_type,
            color: ActivatingColor::None,
        }
    }
}
