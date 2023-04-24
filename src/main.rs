use bevy::{prelude::*, sprite::MaterialMesh2dBundle, window::PrimaryWindow};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_startup_system(spawn_player)
        .add_startup_system(spawn_camera)
        // .add_startup_system(draw_grid)
        // .add_system(player_movement)
        .run();
}

const TILE_SIZE: f32 = 64.0;

#[derive(Component)]
pub struct Player {}

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
    let init_pos = Vec3::new(window.height() / 2.0, window.width() / 2.0, 0.0);

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Box::new(TILE_SIZE, TILE_SIZE, 1.0).into())
                .into(),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            transform: Transform::from_translation(init_pos),
            ..default()
        },
        Grid {},
    ));
}

// Our snake :D
pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let window = window_query.get_single().unwrap();
    let init_pos = Vec3::new(window.height() / 2.0, window.width() / 2.0, 0.0);

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Quad::new(Vec2::new(TILE_SIZE, TILE_SIZE)).into())
                .into(),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            transform: Transform::from_translation(init_pos),
            ..default()
        },
        Player {},
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

// Grid movement
pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let Ok(mut transform) = player_query.get_single_mut() else {
        return;
    };

    let mut direction = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::W) {
        direction += Vec3::new(0.0, 1.0, 0.0);
    } else if keyboard_input.pressed(KeyCode::A) {
        direction += Vec3::new(-1.0, 0.0, 0.0);
    } else if keyboard_input.pressed(KeyCode::S) {
        direction += Vec3::new(0.0, -1.0, 0.0);
    } else if keyboard_input.pressed(KeyCode::D) {
        direction += Vec3::new(1.0, 0.0, 0.0);
    }

    if direction.length() > 0.0 {
        direction = direction.normalize();
    }

    transform.translation += direction * TILE_SIZE * time.delta_seconds();
}
