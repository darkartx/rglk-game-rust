use bevy_ascii_terminal::Tile;

use crate::game::Renderable;

impl From<&Renderable> for Tile {
    fn from(renderable: &Renderable) -> Self {
        Tile::new(
            renderable.glyph,
            renderable.fg_color,
            renderable.bg_color,
        )
    }
}
