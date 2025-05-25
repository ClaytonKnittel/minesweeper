use bevy::{
  DefaultPlugins,
  app::App,
  prelude::PluginGroup,
  utils::default,
  window::{PresentMode, Window, WindowPlugin},
};
use bevy_world_space::{WorldSpacePlugins, world_init::WorldInitPlugin};
use minesweeper::minesweeper::MinesweeperPlugin;

fn main() {
  App::new()
    .add_plugins(DefaultPlugins.set(WindowPlugin {
      primary_window: Some(Window {
        title: "Minesweeper".into(),
        name: Some("minesweeper.app".into()),
        present_mode: PresentMode::AutoVsync,
        prevent_default_event_handling: false,
        ..default()
      }),
      ..default()
    }))
    .add_plugins(WorldSpacePlugins.set(WorldInitPlugin { screen_width: 800., screen_height: 800. }))
    .add_plugins(MinesweeperPlugin)
    .run();
}
