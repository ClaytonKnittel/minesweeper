use bevy::{
  app::{App, Plugin, Startup, Update},
  asset::{Assets, Handle},
  color::Color,
  ecs::{
    component::Component,
    event::{Event, EventReader, EventWriter},
    system::{Commands, Query, Res, ResMut, Resource, Single},
  },
  hierarchy::{BuildChildren, ChildBuild, Children},
  log::info,
  math::primitives::Rectangle,
  render::{
    mesh::{Mesh, Mesh2d},
    view::InheritedVisibility,
  },
  sprite::{ColorMaterial, MeshMaterial2d},
  transform::components::Transform,
};
use bevy_world_space::{
  mouse::MouseEvent,
  position::Position,
  world_unit::{AspectRatio, WorldUnit, WorldVec2},
};
use bitvec::{bitvec, order::Lsb0, vec::BitVec};
use strum::EnumTable;

#[derive(Component)]
#[require(InheritedVisibility, Transform)]
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

#[derive(Event)]
enum ClickTile {
  Uncover { pos: (u32, u32) },
  PlaceFlag { pos: (u32, u32) },
}

#[derive(Component)]
#[require(Transform)]
struct Tile {
  pos: (u32, u32),
}

#[derive(EnumTable)]
enum TileState {
  Covered,
  Empty,
  Flag,
  Bomb,
}

#[derive(Resource)]
struct Resources {
  colors: TileStateTable<Handle<ColorMaterial>>,
}

impl Resources {
  fn initialize(mut materials: ResMut<Assets<ColorMaterial>>) -> Self {
    Self {
      colors: TileStateTable::new(
        materials.add(Color::srgb(0.7, 0.7, 0.7)),
        materials.add(Color::srgb(0.2, 0.2, 0.2)),
        materials.add(Color::srgb(0.4, 0.8, 0.3)),
        materials.add(Color::srgb(0.8, 0.2, 0.1)),
      ),
    }
  }
}

pub struct MinesweeperPlugin;

impl MinesweeperPlugin {
  fn initialize(
    mut commands: Commands,
    aspect_ratio: Res<AspectRatio>,
    mut meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
  ) {
    const WIDTH: u32 = 10;
    const HEIGHT: u32 = 10;

    let resources = Resources::initialize(materials);

    let square = meshes.add(Rectangle::new(1.0, 1.0));

    commands
      .spawn(Minesweeper::empty(WIDTH, HEIGHT))
      .with_children(|parent| {
        for r in 0..WIDTH {
          for c in 0..HEIGHT {
            parent.spawn((
              Tile { pos: (c, r) },
              Mesh2d(square.clone()),
              MeshMaterial2d(resources.colors[TileState::Covered].clone_weak()),
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

    commands.insert_resource(resources);
  }

  fn handle_click(
    aspect_ratio: Res<AspectRatio>,
    resources: Res<Resources>,
    mut clicks: EventReader<MouseEvent>,
    mut click_tiles: EventWriter<ClickTile>,
    minesweeper: Single<&Minesweeper>,
    mut q_tiles: Query<(&Tile, &mut MeshMaterial2d<ColorMaterial>)>,
  ) {
    for click in clicks.read() {
      if let MouseEvent::LeftClick(pos) = click {
        let pos = pos.screen_normalized(&aspect_ratio);
        let x = ((pos.x + 1.) / 2. * minesweeper.width as f32) as u32;
        let y = ((pos.y + 1.) / 2. * minesweeper.height as f32) as u32;
        if (0..minesweeper.width).contains(&x) && (0..minesweeper.height).contains(&y) {
          let pos = (x, y);
          info!("Clicked {:?}", pos);
          click_tiles.send(ClickTile::Uncover { pos });

          for (tile, mut color) in &mut q_tiles {
            if tile.pos == pos {
              *color = MeshMaterial2d(resources.colors[TileState::Empty].clone_weak());
            }
          }
        }
      }
    }
  }
}

impl Plugin for MinesweeperPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_event::<ClickTile>()
      .add_systems(Startup, Self::initialize)
      .add_systems(Update, Self::handle_click);
  }
}
