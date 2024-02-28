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
    size: f32,
}

#[derive(Component)]
struct Ball {
    speed: f32,
    direction: Vec3,
    velocity: f32,
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
    window.iter_mut().for_each(|window| {
        game_size.width = window.width();
        game_size.height = window.height();
        debug!(
            "Global GameSize updated to: width {} height {}",
            game_size.width, game_size.height
        );
    });
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
            speed: 500.,
            size: 200.,
        },
    ));
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("pokeball.png"),
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
        Ball {
            speed: 500.,
            direction: Vec3::new(10., 10., 0.).normalize(),
            velocity: 1.,
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
            size: 200.,
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
    game_size: Res<PokeSize>,
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
                if (transform.translation.y < game_size.height / 2.0 && direction == 1.)
                    || (transform.translation.y > -(game_size.height / 2.0) && direction == -1.)
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
                if (transform.translation.y < game_size.height / 2.0 && direction == 1.)
                    || (transform.translation.y > -(game_size.height / 2.0) && direction == -1.)
                {
                    transform.translation.y += player.speed * direction * time.delta_seconds()
                }
            }
        }
    }
}

fn ball_movement(time: Res<Time>, mut ball_query: Query<(&Ball, &mut Transform)>) {
    for (ball, mut transform) in ball_query.iter_mut() {
        transform.translation += ball.direction * ball.speed * ball.velocity * time.delta_seconds();
    }
}

fn ball_collision_system(
    mut ball_query: Query<(&mut Ball, &Transform)>,
    player_query: Query<(&Player, &Transform)>,
    game_size: Res<PokeSize>,
) {
    for (mut ball, ball_transform) in ball_query.iter_mut() {
        // check wall collision
        wall_collision_ball(&game_size, ball_transform, &mut ball);

        // check player collision
        player_collision_ball(&player_query, ball_transform, ball);
    }
}

fn wall_collision_ball(
    game_size: &Res<'_, PokeSize>,
    ball_transform: &Transform,
    ball: &mut Mut<'_, Ball>,
) {
    if game_size.height / 2.0 < ball_transform.translation.y {
        ball.direction.y *= -1.;
    }
    if -(game_size.height / 2.0) > ball_transform.translation.y {
        ball.direction.y *= -1.;
    }
    if game_size.width / 2.0 < ball_transform.translation.x {
        ball.direction.x *= -1.;
    }
    if -(game_size.width / 2.0) > ball_transform.translation.x {
        ball.direction.x *= -1.;
    }
}

fn player_collision_ball(
    player_query: &Query<'_, '_, (&Player, &Transform)>,
    ball_transform: &Transform,
    mut ball: Mut<'_, Ball>,
) {
    for (player, player_transform) in player_query {
        match player.paddle {
            Paddle::One => {
                if player_transform.translation.x + 30. >= ball_transform.translation.x
                    && ball_transform.translation.y
                        <= (player_transform.translation.y + player.size / 2.)
                    && ball_transform.translation.y
                        >= (player_transform.translation.y - player.size / 2.)
                {
                    ball.direction.x *= -1.;

                    // speed up ball
                    speed_up_ball(ball_transform, player_transform, player, &mut ball);
                }
            }
            Paddle::Two => {
                if player_transform.translation.x - 30. <= ball_transform.translation.x
                    && ball_transform.translation.y
                        <= (player_transform.translation.y + player.size / 2.)
                    && ball_transform.translation.y
                        >= (player_transform.translation.y - player.size / 2.)
                {
                    ball.direction.x *= -1.;
                    speed_up_ball(ball_transform, player_transform, player, &mut ball);
                }
            }
        }
    }
}

fn speed_up_ball(
    ball_transform: &Transform,
    player_transform: &Transform,
    player: &Player,
    ball: &mut Mut<'_, Ball>,
) {
    if ball_transform.translation.y
        <= (player_transform.translation.y + ((player.size / 2.) * 2. / 3.))
    {
        ball.velocity += 0.05
    } else if ball_transform.translation.y
        <= (player_transform.translation.y + ((player.size / 2.) * 1. / 3.))
    {
        ball.velocity += 0.1
    }
}
