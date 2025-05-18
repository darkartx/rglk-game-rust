use bevy::prelude::*;
use bevy_ascii_terminal::{color::BLACK, *};

use crate::game::{Position, Renderable};

const TERMINAL_SIZE: [u32; 2] = [80, 40];

static CLEAR_TILE: Tile = Tile::new(' ', BLACK, BLACK);

pub struct RendererPlugin;

impl Plugin for RendererPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(TerminalPlugins)
            .add_systems(Startup, setup)
            .add_systems(Update, render)
        ;
    }
}

pub fn window_config(title: String) -> Window {
    Window {
        title,
        resolution: (TERMINAL_SIZE[0] as f32 * 12.0, TERMINAL_SIZE[1] as f32 * 12.0).into(),
        resizable: false,
        ..Default::default()
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(
        Terminal::new(TERMINAL_SIZE)
            .with_clear_tile(CLEAR_TILE)
    );
    commands.spawn(TerminalCamera::new());
}

fn render(entities: Query<(&Renderable, &Position)>, mut terminal: Query<&mut Terminal>) {
    let mut terminal = terminal.single_mut().unwrap();

    terminal.clear();

    for (renderable, position) in entities {
        terminal.put_tile(position.0, renderable.into());
    }

}