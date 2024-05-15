use bevy::prelude::*;

use crate::create_objects::{BallState, PlayerRectangle, RectangleState};

pub struct GameLogicPlugin;

#[derive(Resource)]
struct GameState {
    lives: i32,
}

impl Plugin for GameLogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                activate_ball,
                move_inactive_ball,
                (
                    check_if_ball_hits_rectangle,
                    check_if_ball_hits_window_side,
                    move_ball,
                    move_rectangle,
                ).chain(),
            ),
        )
        .insert_resource(GameState { lives: 3 });
    }
}

fn move_ball(mut ball: Query<(&mut Transform, &BallState)>, timer: Res<Time>) {
    let (mut transform, ball) = ball.single_mut();
    if !ball.active {
        return;
    }
    transform.translation += ball.speed * timer.delta_seconds() * ball.direction;
}

fn activate_ball(
    mut ball: Query<&mut BallState>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let mut ball_state = ball.single_mut();
    if keyboard_input.just_pressed(KeyCode::Space) {
        ball_state.active = true;
    }
}

fn move_inactive_ball(
    mut ball: Query<(&mut Transform, &BallState, &GlobalTransform), With<BallState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    rectangle_state: Query<&RectangleState, With<PlayerRectangle>>,
    window: Query<&Window>,
) {
    let (mut transform, ball_state, global_transform) = ball.single_mut();
    let rectangle_width = rectangle_state.single().width;
    let window = window.single();
    if ball_state.active {
        return;
    }
    if keyboard_input.pressed(KeyCode::ArrowLeft) && global_transform.translation().x - rectangle_width / 2.0 >= -window.width() / 2.0 {
        transform.translation += Vec3 {
            x: -15.0,
            y: 0.0,
            z: 0.0,
        };
    } else if keyboard_input.pressed(KeyCode::ArrowRight) && global_transform.translation().x + rectangle_width / 2.0 <= window.width() / 2.0{
        transform.translation += Vec3 {
            x: 15.0,
            y: 0.0,
            z: 0.0,
        };
    }
}

fn move_rectangle(
    mut rectangle: Query<(&GlobalTransform, &mut Transform, &RectangleState), With<PlayerRectangle>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    window: Query<&Window>,
) {
    let (global_transform, mut transform, state) = rectangle.single_mut();
    let window = window.single();
    if keyboard_input.pressed(KeyCode::ArrowLeft) && global_transform.translation().x - state.width / 2.0 >= -window.width() / 2.0 {
        transform.translation += Vec3 {
            x: -15.0,
            y: 0.0,
            z: 0.0,
        };

    } else if keyboard_input.pressed(KeyCode::ArrowRight) && global_transform.translation().x + state.width / 2.0 <= window.width() / 2.0 {
        transform.translation += Vec3 {
            x: 15.0,
            y: 0.0,
            z: 0.0,
        };
    }
}

fn check_if_ball_hits_rectangle(
    mut commands: Commands,
    mut rectangles: Query<(&GlobalTransform, Entity, &mut RectangleState)>,
    mut ball: Query<(&GlobalTransform, &mut BallState, &mut Transform)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let (ball_global_transform, mut ball_state, mut ball_transform) = ball.single_mut();
    for (rectangle_transform, rectangle_entity, mut rectangle_state) in &mut rectangles {
        let rectangle_width = rectangle_state.width;
        let rectangle_height = rectangle_state.height;
        let rectangle_x = rectangle_transform.translation().x;
        let rectangle_y = rectangle_transform.translation().y;
        let ball_x = ball_global_transform.translation().x;
        let ball_y = ball_global_transform.translation().y;
        let bottom_left_corner = Vec2 {
            x: rectangle_x - rectangle_width / 2.0,
            y: rectangle_y - rectangle_height / 2.0,
        };
        let bottom_right_corner = Vec2 {
            x: rectangle_x + rectangle_width / 2.0,
            y: rectangle_y - rectangle_height / 2.0,
        };
        let top_left_corner = Vec2 {
            x: rectangle_x - rectangle_width / 2.0,
            y: rectangle_y + rectangle_height / 2.0,
        };
        let top_right_corner = Vec2 {
            x: rectangle_x + rectangle_width / 2.0,
            y: rectangle_y + rectangle_height / 2.0,
        };
        let distance_to_bottom_left_corner = Vec2 {
            x: ball_x - bottom_left_corner.x,
            y: ball_y - bottom_left_corner.y,
        }
        .length();
        let distance_to_bottom_right_corner = Vec2 {
            x: ball_x - bottom_right_corner.x,
            y: ball_y - bottom_right_corner.y,
        }
        .length();
        let distance_to_top_left_corner = Vec2 {
            x: ball_x - top_left_corner.x,
            y: ball_y - top_left_corner.y,
        }
        .length();
        let distance_to_top_right_corner = Vec2 {
            x: ball_x - top_right_corner.x,
            y: ball_y - top_right_corner.y,
        }
        .length();
        if (ball_x - rectangle_x).abs() <= rectangle_width / 2.0
            && (ball_y - rectangle_y).abs() <= rectangle_height / 2.0 + ball_state.radius
        {
            if !rectangle_state.player_controlled {
                rectangle_state.hit_bar -= 1;
                commands.entity(rectangle_entity).insert(materials.add(Color::rgb(0.0, 1.0 / rectangle_state.hit_bar as f32, 0.0)));
                reverse_ball_y_direction(&mut ball_state);
            } else {
                change_ball_direction(&mut ball_state, (ball_x - rectangle_x) / rectangle_width);
            }
            // prevent ball from getting stuck
            ball_transform.translation += ball_state.direction * 5.0;
        } else if (ball_y - rectangle_y).abs() <= rectangle_height / 2.0
            && (ball_x - rectangle_x).abs() <= rectangle_width / 2.0 + ball_state.radius
            || distance_to_bottom_left_corner <= ball_state.radius
            || distance_to_top_right_corner <= ball_state.radius
            || distance_to_top_left_corner <= ball_state.radius
            || distance_to_bottom_right_corner <= ball_state.radius
        {
            if !rectangle_state.player_controlled {
                // prevents ball from going through all rectangles in a line if the ball goes straight up
                if ball_state.direction.x.abs() / ball_state.direction.length() > 0.1 {
                    reverse_ball_x_direction(&mut ball_state);
                } else {
                    reverse_ball_y_direction(&mut ball_state);
                }
                rectangle_state.hit_bar -= 1;
                ball_transform.translation += ball_state.direction * 5.0;
                commands.entity(rectangle_entity).insert(materials.add(Color::rgb(0.0, 1.0 / rectangle_state.hit_bar as f32, 0.0)));
            }
        }
        if rectangle_state.hit_bar == 0 {
            commands.entity(rectangle_entity).despawn();
        }
    }
}

fn check_if_ball_hits_window_side(
    windows: Query<&Window>,
    mut ball: Query<(&GlobalTransform, &mut BallState, &mut Transform)>,
    rectangle_global_transform: Query<&GlobalTransform, With<PlayerRectangle>>,
    mut game_state: ResMut<GameState>,
) {
    let (ball_global_transform, mut ball_state, mut ball_transform) = ball.single_mut();
    let window = windows.single();
    let rectangle_global_transform = rectangle_global_transform.single().translation();
    let ball_x = ball_global_transform.translation().x;
    let ball_y = ball_global_transform.translation().y;
    let window_width = window.width();
    let window_height = window.height();
    if ball_x - ball_state.radius <= -window_width / 2.0 || ball_x + ball_state.radius >= window_width / 2.0 {
        reverse_ball_x_direction(&mut ball_state);
        ball_transform.translation += ball_state.direction * 5.0;
    } else if ball_y + ball_state.radius >= window_height / 2.0 {
        reverse_ball_y_direction(&mut ball_state);
        ball_transform.translation += ball_state.direction * 5.0;
    } else if ball_y - ball_state.radius <= -window_height / 2.0 {
        game_state.lives -= 1;
        ball_transform.translation += ball_state.direction * 5.0;
        handle_ball_hits_floor(
            &mut ball_state,
            &mut ball_transform.translation,
            rectangle_global_transform,
            window_height,
        );
    }
}

fn handle_ball_hits_floor(
    ball_state: &mut BallState,
    ball_position: &mut Vec3,
    rectangle_position: Vec3,
    window_height: f32,
) {
    *ball_position = Vec3 {
        x: rectangle_position.x,
        // add offset to prevent ball from getting stuck
        y: -window_height / 2.0 + ball_state.radius * 2.0 + window_height / 60.0,
        z: 0.0,
    };
    ball_state.direction = Vec3::new(0.0, 1.0, 0.0);
    ball_state.active = false;
}

fn change_ball_direction(ball_state: &mut BallState, x_direction: f32) {
    ball_state.direction = Vec3 {
        x: x_direction,
        y: -ball_state.direction.y,
        z: ball_state.direction.z,
    };
    // normalize vector back to original speed
    ball_state.direction = ball_state.direction / ball_state.direction.length();
}

fn reverse_ball_x_direction(ball_state: &mut BallState) {
    ball_state.direction = Vec3 {
        x: -ball_state.direction.x,
        y: ball_state.direction.y,
        z: ball_state.direction.z,
    }
}

fn reverse_ball_y_direction(ball_state: &mut BallState) {
    ball_state.direction = Vec3 {
        x: ball_state.direction.x,
        y: -ball_state.direction.y,
        z: ball_state.direction.z,
    }
}
