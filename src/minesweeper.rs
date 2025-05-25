use bevy::{
  app::{App, Plugin, Startup},
  asset::Assets,
  color::Color,
  ecs::{
    component::Component,
    system::{Commands, Res, ResMut},
  },
  hierarchy::{BuildChildren, ChildBuild},
  math::primitives::Rectangle,
  render::mesh::{Mesh, Mesh2d},
  sprite::{ColorMaterial, MeshMaterial2d},
  transform::components::Transform,
};
use bevy_world_space::{
  position::Position,
  world_unit::{AspectRatio, WorldUnit, WorldVec2},
};
use bitvec::{bitvec, order::Lsb0, vec::BitVec};

#[derive(Component)]
#[require(Transform)]
struct Minesweeper {
  bombs: BitVec<u32>,
  flags: BitVec<u32>,
  revealed: BitVec<u32>,
  width: u32,
  height: u32,
}

impl Minesweeper {
  fn empty(width: u32, height: u32) -> Self {
    let size = (width * height) as usize;

    Self {
      bombs: bitvec![u32, Lsb0; 0; size],
      flags: bitvec![u32, Lsb0; 0; size],
      revealed: bitvec![u32, Lsb0; 0; size],
      width,
      height,
    }
  }
}

#[derive(Component)]
#[require(Transform)]
struct Tile;

pub struct MinesweeperPlugin;

impl MinesweeperPlugin {
  fn initialize(
    mut commands: Commands,
    aspect_ratio: Res<AspectRatio>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
  ) {
    const WIDTH: u32 = 10;
    const HEIGHT: u32 = 10;

    let circle = meshes.add(Rectangle::new(1.0, 1.0));
    let gray = materials.add(Color::srgb(0.7, 0.7, 0.7));

    commands
      .spawn(Minesweeper::empty(WIDTH, HEIGHT))
      .with_children(|parent| {
        for r in 0..WIDTH {
          for c in 0..HEIGHT {
            parent.spawn((
              Tile,
              Mesh2d(circle.clone()),
              MeshMaterial2d(gray.clone()),
              Position::new(
                WorldVec2::new_normalized(
                  2. * ((c as f32 + 0.5) / WIDTH as f32) - 1.,
                  2. * ((r as f32 + 0.5) / HEIGHT as f32) - 1.,
                  &aspect_ratio,
                ),
                0.85 * WorldUnit::screen_width(&aspect_ratio) / WIDTH as f32,
                1,
                10.,
              ),
            ));
          }
        }
      });
  }
}

impl Plugin for MinesweeperPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Startup, Self::initialize);
  }
}
