use bevy::{
    color::LinearRgba,
    ecs::{
        bundle::Bundle,
        component::Component
    }, math::IVec2
};

#[derive(Component, Debug)]
pub struct Position(pub IVec2);

impl From<IVec2> for Position {
    fn from(value: IVec2) -> Self {
        Self(value)
    }
}

impl From<Position> for IVec2 {
    fn from(value: Position) -> Self {
        value.0
    }
}

impl From<(i32, i32)> for Position {
    fn from(value: (i32, i32)) -> Self {
        Self(value.into())
    }
}

impl From<[i32; 2]> for Position {
    fn from(value: [i32; 2]) -> Self {
        Self(value.into())
    }
}

#[derive(Debug, Component)]
pub struct Renderable {
    pub glyph: char,
    pub fg_color: LinearRgba,
    pub bg_color: LinearRgba,
}

#[derive(Debug, Bundle)]
pub struct EntityBundle {
    pub renderable: Renderable,
    pub position: Position
}

#[derive(Debug, Component)]
pub struct LeftMover;
