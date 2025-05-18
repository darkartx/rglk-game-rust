mod renderer;
mod game;

use bevy::prelude::*;

use renderer::{RendererPlugin, window_config};
use game::GamePlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(
                WindowPlugin {
                    primary_window: Some(window_config("Rglk Game".to_string())),
                    ..Default::default()
                }
            ),
            RendererPlugin,
            GamePlugin
        ))
        .run();
}
