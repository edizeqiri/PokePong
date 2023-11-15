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
        .add_systems(Update, ball_collision_system)
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
    Two,
}

#[derive(Component)]
struct Ball {
    speed: f32,
    direction: Vec3,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("karpador.png"),
            transform: Transform::from_xyz(500., 0., 0.).with_scale(Vec3::new(0.25, 0.25, 0.25)),
            ..default()
        },
        Player::One,
        Direction::Up,
    ));
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("karpador.png"),
            transform: Transform::from_xyz(0., 0., 0.).with_scale(Vec3::new(0.25, 0.25, 0.25)),
            ..default()
        },
        Ball {
            speed: 100.,
            direction: Vec3::new(1., 0., 0.).normalize(),
        },
    ));
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("karpador.png"),
            transform: Transform::from_xyz(-500., 0., 0.).with_scale(Vec3::new(0.25, 0.25, 0.25)),
            ..default()
        },
        Player::Two,
        Direction::Down,
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

fn ball_movement(time: Res<Time>, mut ball_query: Query<(&Ball, &mut Transform)>) {
    for (ball, mut transform) in ball_query.iter_mut() {
        transform.translation += ball.direction * ball.speed * time.delta_seconds();
    }
}

fn ball_collision_system(
    mut ball_query: Query<(&mut Ball, &Transform)>,
    player_query: Query<(&Player, &Transform)>,
) {
    for (mut ball, ball_transform) in ball_query.iter_mut() {
        for (player, player_transform) in &player_query {
            match player {
                Player::One => {
                    if player_transform.translation.x <= ball_transform.translation.x {
                        ball.direction.x *= -1.
                    }
                }
                Player::Two => {
                    if player_transform.translation.x >= ball_transform.translation.x {
                        ball.direction.x *= -1.
                    }
                }
            }
        }
    }
}
