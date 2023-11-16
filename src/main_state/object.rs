use super::Colors;

#[derive(Debug, Clone)]
pub enum ObjectType {
    Player,
    Box,
}

#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct Object {
    pub object_type: ObjectType,
    pub color: Colors,
}

impl Object {
    pub fn new(object_type: ObjectType, color: Colors) -> Self {
        Object { object_type, color }
    }
}
