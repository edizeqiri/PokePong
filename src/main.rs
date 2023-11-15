use bevy::a11y::accesskit::Role::Math;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(
            // This sets image filtering to nearest
            // This is done to prevent textures with low resolution (e.g. pixel art) from being blurred
            // by linear filtering.
            ImagePlugin::default_nearest(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, sprite_movement)
        .run();
}

#[derive(Component)]
enum Direction {
    Up,
    Down
}

#[derive(Component)]
enum Player {
    One,
    Two
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("karpador.png"),
            transform: Transform::from_xyz(500., 0., 0.).with_scale(Vec3::new(0.25,0.25,0.25)),
            ..default()
        },
        Player::One,
        Direction::Up
    ));
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("karpador.png"),
            transform: Transform::from_xyz(0., 0., 0.).with_scale(Vec3::new(0.25,0.25,0.25)),
            ..default()
        },

    ));
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("karpador.png"),
            transform: Transform::from_xyz(-500., 0., 0.).with_scale(Vec3::new(0.25,0.25,0.25)),
            ..default()
        },
        Player::Two,
        Direction::Down
    ));
}

fn sprite_movement(time: Res<Time>, mut sprite_position: Query<(&mut Direction, &mut Transform)>) {
    for (mut logo, mut transform) in &mut sprite_position {
       match *logo {
           Direction::Up => transform.translation.y += 300. * time.delta_seconds(),
           Direction::Down => transform.translation.y -= 300. * time.delta_seconds(),
       }

        if transform.translation.y > 100. {
            *logo = Direction::Down;
        } else if transform.translation.y < -100. {
            *logo = Direction::Up;
        }


    }
}