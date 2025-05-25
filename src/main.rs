use bevy::{
  DefaultPlugins,
  app::App,
  prelude::PluginGroup,
  utils::default,
  window::{PresentMode, Window, WindowPlugin},
};

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
    .run();
}
