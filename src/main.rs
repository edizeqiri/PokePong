use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.984, 0.929, 0.761)))
        .add_plugins(DefaultPlugins.set(
            // This sets image filtering to nearest
            // This is done to prevent textures with low resolution (e.g. pixel art) from being blurred
            // by linear filtering.
            ImagePlugin::default_nearest(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, ball_movement)
        .add_systems(FixedUpdate, ball_player_bounce)
        .add_systems(FixedUpdate, ball_wall_bounce)
        .add_systems(FixedUpdate, ball_out)
        .add_systems(FixedUpdate, player_movement)
        .add_systems(Update, set_window_size)
        .add_systems(Update, update_score_board)
        .add_systems(Update, draw_collider_gizmo)
        .run();
}

#[derive(Component)]
struct Player {
    paddle: Paddle,
    speed: f32,
    score: u32,
}

enum Paddle {
    One,
    Two,
}

#[derive(Component)]
struct Ball {
    speed: f32,
    direction: Vec3,
}

#[derive(Component)]
struct BoxCollider {
    width: f32,
    height: f32,
}

#[derive(Component)]
struct ScoreBoard;

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
        ScoreBoard,
        Text2dBundle {
            text: Text::from_section(
                "0:0",
                TextStyle {
                    color: Color::BLACK,
                    font_size: 24.0,
                    ..default()
                },
            ),
            transform: Transform::from_xyz(0., 350., 0.),
            ..default()
        },
    ));
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("pokepaddle.png"),
            transform: Transform::from_xyz(-500., 0., 0.),
            ..default()
        },
        Player {
            paddle: Paddle::One,
            speed: 300.,
            score: 0,
        },
        BoxCollider {
            width: 55.,
            height: 150.,
        },
    ));
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("pokeball.png"),
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
        Ball {
            speed: 200.,
            direction: Vec3::new(10., 10., 0.).normalize(),
        },
        BoxCollider {
            width: 45.,
            height: 45.,
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
            score: 0,
        },
        BoxCollider {
            width: 55.,
            height: 150.,
        },
    ));
}

fn player_movement(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&Player, &mut Transform)>,
    game_size: Res<PokeSize>,
) {
    for (player, mut transform) in player_query.iter_mut() {
        match player.paddle {
            Paddle::One => {
                let mut direction = 0.;

                if keys.pressed(KeyCode::KeyW) {
                    direction = 1.;
                }
                if keys.pressed(KeyCode::KeyS) {
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

                if keys.pressed(KeyCode::ArrowUp) {
                    direction = 1.;
                }
                if keys.pressed(KeyCode::ArrowDown) {
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
        transform.translation += ball.direction * ball.speed * time.delta_seconds();
    }
}

fn ball_wall_bounce(
    mut ball_query: Query<(&mut Ball, &mut Transform, &mut BoxCollider)>,
    game_size: Res<PokeSize>,
) {
    for (mut ball, ball_transform, collider) in ball_query.iter_mut() {
        // Top wall
        if game_size.height / 2.0 < ball_transform.translation.y + collider.height / 2. {
            ball.direction.y *= -1.;
        }
        // Bottom wall
        if -(game_size.height / 2.0) > ball_transform.translation.y - collider.height / 2. {
            ball.direction.y *= -1.;
        }
    }
}

fn ball_player_bounce(
    mut ball_query: Query<(&mut Ball, &mut Transform)>,
    player_query: Query<(&Player, &BoxCollider, &Transform), Without<Ball>>,
) {
    for (mut ball, mut ball_transform) in ball_query.iter_mut() {
        for (player, box_collider, player_transform) in player_query.iter() {
            match player.paddle {
                Paddle::One => {
                    if player_transform.translation.x + box_collider.width
                        >= ball_transform.translation.x
                        && ball_transform.translation.y
                            <= (player_transform.translation.y + box_collider.height / 2.)
                        && ball_transform.translation.y
                            >= (player_transform.translation.y - box_collider.height / 2.)
                    {
                        ball.direction.x *= -1.;

                        // speed up ball
                        speed_up_ball(
                            &mut ball_transform,
                            box_collider,
                            player_transform,
                            &mut ball,
                        );
                    }
                }
                Paddle::Two => {
                    if player_transform.translation.x - box_collider.width
                        <= ball_transform.translation.x
                        && ball_transform.translation.y
                            <= (player_transform.translation.y + box_collider.height / 2.)
                        && ball_transform.translation.y
                            >= (player_transform.translation.y - box_collider.height / 2.)
                    {
                        ball.direction.x *= -1.;
                        speed_up_ball(
                            &mut ball_transform,
                            box_collider,
                            player_transform,
                            &mut ball,
                        );
                    }
                }
            }
        }
    }
}

fn speed_up_ball(
    ball_transform: &mut Mut<'_, Transform>,
    box_collider: &BoxCollider,
    player_transform: &Transform,
    ball: &mut Mut<'_, Ball>,
) {
    if ball_transform.translation.y
        <= (player_transform.translation.y + ((box_collider.height / 2.) * 2. / 3.))
    {
        ball.speed += 0.05
    } else if ball_transform.translation.y
        <= (player_transform.translation.y + ((box_collider.height / 2.) * 1. / 3.))
    {
        ball.speed += 0.1
    }
}

fn ball_out(
    mut ball_query: Query<(&mut Ball, &mut Transform)>,
    mut player_query: Query<&mut Player>,
    game_size: Res<PokeSize>,
) {
    for (mut ball, mut ball_transform) in ball_query.iter_mut() {
        // Right
        if game_size.width / 2.0 < ball_transform.translation.x {
            ball_transform.translation = Vec3::new(0., 0., 0.);
            ball.speed = 200.;

            for mut player in player_query.iter_mut() {
                if let Paddle::One = player.paddle {
                    player.score += 1
                }
            }
        }
        // Left
        if -(game_size.width / 2.0) > ball_transform.translation.x {
            ball_transform.translation = Vec3::new(0., 0., 0.);
            ball.speed = 200.;

            for mut player in player_query.iter_mut() {
                if let Paddle::Two = player.paddle {
                    player.score += 1
                }
            }
        }
    }
}

fn update_score_board(
    mut score_board_query: Query<&mut Text, With<ScoreBoard>>,
    player_query: Query<&Player>,
) {
    for mut score_board in &mut score_board_query {
        let mut player_one_score = 0;
        let mut player_two_score = 0;

        for player in player_query.iter() {
            match player.paddle {
                Paddle::One => player_one_score = player.score,
                Paddle::Two => player_two_score = player.score,
            }
        }
        score_board.sections[0].value = format!("{player_one_score}:{player_two_score}");
    }
}

fn draw_collider_gizmo(collider_query: Query<(&BoxCollider, &Transform)>, mut gizmos: Gizmos) {
    for (collider, transform) in collider_query.iter() {
        gizmos.rect_2d(
            transform.translation.xy(),
            Rot2::default(),
            Vec2::new(collider.width, collider.height),
            Color::hsl(123.0, 123.0, 123.0),
        )
    }
}

fn collider(Query: BoxCollider) {}
