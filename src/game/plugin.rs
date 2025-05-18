use bevy::prelude::*;
use bevy_ascii_terminal::{color::{BLACK, RED, YELLOW}, Glyph};

use super::component::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup)
            .add_systems(Update, left_walking)
        ;
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(EntityBundle {
        renderable: Renderable { glyph: '@', fg_color: YELLOW, bg_color: BLACK },
        position: [40, 25].into(),
    });

    for i in 0..10 {
        commands.spawn((
            EntityBundle {
                renderable: Renderable { glyph: Glyph::SmilingFace.into(), fg_color: RED, bg_color: BLACK },
                position: [i * 7, 20].into()
            },
            LeftMover
        ));
    }
}

fn left_walking(entities: Query<&mut Position, With<LeftMover>>) {
    for mut position in entities {
        position.0.x -= 1;
        if position.0.x < 0 {
            position.0.x = 79;
        }
    }
}