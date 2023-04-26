use bevy::{prelude::*, window::PrimaryWindow};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_snake_head)
        // .add_startup_system(draw_grid)
        .add_system(movement)
        .run();
}

const TILE_SIZE: f32 = 32.0;

#[derive(Component)]
pub struct SnakeHead {}

#[derive(Component)]
pub struct Food {}

#[derive(Component)]
pub struct Grid {}

pub fn spawn_snake_head(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::GREEN,
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(TILE_SIZE, TILE_SIZE, TILE_SIZE),
                ..default()
            },
            ..default()
        },
        SnakeHead {},
    ));
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

// Movement
pub fn movement(
    mut snake_head_query: Query<&mut Transform, With<SnakeHead>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    for mut transform in snake_head_query.iter_mut() {
        transform.translation.y += 2.0;
    }

    // let window = window_query.get_single().unwrap();

    // let Ok(mut transform) = snake_head_query.get_single_mut() else {
    //     return;
    // };
}
