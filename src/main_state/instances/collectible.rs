#[derive(Debug, Clone, Copy)]
pub enum CollectibleType {
    Win,
}

#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct Collectible {
    pub collectible_type: CollectibleType,
}

impl Collectible {
    pub fn new(collectible_type: CollectibleType) -> Collectible {
        Collectible { collectible_type }
    }
}
