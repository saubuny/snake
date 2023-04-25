use bevy::{prelude::*, sprite::MaterialMesh2dBundle, window::PrimaryWindow};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_snake_head)
        // .add_startup_system(draw_grid)
        .run();
}

const TILE_SIZE: f32 = 64.0;

#[derive(Component)]
pub struct SnakeHead {}

#[derive(Component)]
pub struct Food {}

#[derive(Component)]
pub struct Grid {}

// Grid for easy viewing of tiles
pub fn draw_grid(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let window = window_query.get_single().unwrap();

    // Painful amount of casting
    for x in (0..window.width() as i32).step_by(TILE_SIZE as usize) {
        for y in (0..window.height() as i32).step_by(TILE_SIZE as usize) {
            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: meshes
                        .add(shape::Quad::new(Vec2::new(TILE_SIZE, TILE_SIZE)).into())
                        .into(),
                    material: materials.add(ColorMaterial::from(Color::WHITE)),
                    transform: Transform::from_xyz(x as f32, y as f32, 1.0),
                    ..default()
                },
                Grid {},
            ));
        }
    }
}

pub fn spawn_snake_head(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Quad::new(Vec2::new(TILE_SIZE, TILE_SIZE)).into())
                .into(),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            transform: Transform::from_translation(Vec3::new(
                window.height() / 2.0,
                window.width() / 2.0,
                1.0,
            )),
            ..default()
        },
        SnakeHead {},
    ));
}

// Basic 2D Camera
pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}
