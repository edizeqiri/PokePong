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
        .add_systems(Update, ball_movement)
        .add_systems(Update, sprite_movement)
        .run();
}

#[derive(Component)]
enum Direction {
    Up,
    Down,
}

#[derive(Component)]
enum Player {
    One,
    Two
}

#[derive(Component)]
struct Ball;

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
        Ball
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
           Direction::Up => transform.translation.y += 150. * time.delta_seconds(),
           Direction::Down => transform.translation.y -= 150. * time.delta_seconds(),
       }

        if transform.translation.y > 100. {
            *logo = Direction::Down;
        } else if transform.translation.y < -100. {
            *logo = Direction::Up;
        }
    }
}

fn ball_movement(time: Res<Time>, mut balls: Query<(&mut Ball, &mut Transform)>, mut players: Query<(&mut Player, &mut Transform)>) {

    for (mut ball, mut ball_transform) in &mut balls {
        for (mut player, mut player_transform) in &mut players {
            if ball_transform.translation.x >= player_transform.translation.x {

            }
        }


    }
}