use super::ActivatingColor;

#[derive(Debug, Clone, Copy)]
pub enum ObjectType {
    Player,
    Box,
    TeleBox,
}

#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct Object {
    pub object_type: ObjectType,
    pub color: ActivatingColor,
}

impl Object {
    pub fn new(object_type: ObjectType, color: ActivatingColor) -> Self {
        Object { object_type, color }
    }

    pub fn default(object_type: ObjectType) -> Self {
        Object {object_type, color: ActivatingColor::None}
    }
}
