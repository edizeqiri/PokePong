use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.984, 0.929, 0.761)))
        .add_plugins(DefaultPlugins.set(
            // This sets image filtering to nearest
            // This is done to prevent textures with low resolution (e.g. pixel art) from being blurred
            // by linear filtering.
            ImagePlugin::default_nearest(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, ball_movement)
        .add_systems(Update, ball_collision_system)
        .add_systems(Update, player_movement)
        .add_systems(Update, sprite_movement)
        .add_systems(Update, set_window_size)
        .run();
}

#[derive(Component)]
enum Direction {
    Up,
    Down,
}

#[derive(Component)]
enum Paddle {
    One,
    Two,
}

#[derive(Component)]
struct Player {
    paddle: Paddle,
    speed: f32,
}

#[derive(Component)]
struct Ball {
    speed: f32,
    direction: Vec3,
}

#[derive(Component)]
struct Score {
    score: i32,
}

#[derive(Debug, Default, Resource)]
struct PokeSize {
    width: f32,
    height: f32,
}

fn set_window_size(mut window: Query<&mut Window>, mut game_size: ResMut<PokeSize>) {
    for mut window in window.iter_mut() {
        game_size.width = window.width();
        game_size.height = window.height();
        debug!(
            "Global GameSize updated to: width {} height {}",
            game_size.width, game_size.height
        );
    }
}
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.insert_resource(PokeSize {
        width: 800.0,
        height: 800.0,
    });
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("pokepaddle.png"),
            transform: Transform::from_xyz(-500., 0., 0.),
            ..default()
        },
        Player {
            paddle: Paddle::One,
            speed: 300.,
        },
    ));
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("pokeball.png"),
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
        Ball {
            speed: 100.,
            direction: Vec3::new(10., 10., 0.).normalize(),
        },
    ));
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("pokepaddle.png"),
            transform: Transform::from_xyz(500., 0., 0.),
            ..default()
        },
        Player {
            paddle: Paddle::Two,
            speed: 300.,
        },
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
fn player_movement(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut player_query: Query<(&Player, &mut Transform)>,
) {
    for (player, mut transform) in player_query.iter_mut() {
        match player.paddle {
            Paddle::One => {
                let mut direction = 0.;

                if keys.pressed(KeyCode::W) {
                    direction = 1.;
                }
                if keys.pressed(KeyCode::S) {
                    direction = -1.;
                }
                // Fix hard coded screen size
                // This is a quick fix to prevent the paddles from going off screen
                if (transform.translation.y < 300.0 && direction == 1.)
                    || (transform.translation.y > -300.0 && direction == -1.)
                {
                    transform.translation.y += player.speed * direction * time.delta_seconds()
                }
            }
            Paddle::Two => {
                let mut direction = 0.;

                if keys.pressed(KeyCode::Up) {
                    direction = 1.;
                }
                if keys.pressed(KeyCode::Down) {
                    direction = -1.;
                }
                // Fix hard coded screen size
                // This is a quick fix to prevent the paddles from going off screen
                if (transform.translation.y < 300.0 && direction == 1.)
                    || (transform.translation.y > -300.0 && direction == -1.)
                {
                    transform.translation.y += player.speed * direction * time.delta_seconds()
                }
            }
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
            match player.paddle {
                Paddle::One => {
                    if player_transform.translation.x <= ball_transform.translation.x {
                        ball.direction.x *= -1.
                    }
                }
                Paddle::Two => {
                    if player_transform.translation.x >= ball_transform.translation.x {
                        ball.direction.x *= -1.
                    }
                }
            }
        }
    }
}
