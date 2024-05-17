use bevy::prelude::*;

use crate::create_objects::{BallState, BrickState, PlayerRectangleState, Floor};
use bevy_xpbd_2d::prelude::*;

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
                on_collision_with_brick,
                on_collision_with_floor,
                move_inactive_ball,
                move_rectangle,
                (
                    on_collision_with_player_rectangle,
                    remove_brick
                ).chain()
            ),
        )
        .insert_resource(GameState { lives: 3 });
    }
}

fn activate_ball(
    mut ball_query: Query<(&mut BallState, &mut LinearVelocity)>, 
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let (mut ball_state, mut linear_velocity) = ball_query.single_mut();
    if keyboard_input.just_pressed(KeyCode::Space) {
        ball_state.active = true;
        linear_velocity.x = 0.0;
        linear_velocity.y = 1.0 * ball_state.speed;
    }
}

// read more about event readers
fn on_collision_with_player_rectangle(
    mut linear_velocity_query: Query<&mut LinearVelocity, With<BallState>>,
    ball_query: Query<(&GlobalTransform, &BallState)>,
    rectangle_query: Query<(&GlobalTransform, &PlayerRectangleState)>,
    colliding_entities_query: Query<&CollidingEntities, With<PlayerRectangleState>>,
) {
    let mut linear_velocity = linear_velocity_query.single_mut();

    if let Ok(colliding_entities) = colliding_entities_query.get_single() {
        if !(colliding_entities.0.is_empty()) {
            let (ball_global_transform, ball_state) = ball_query.single();
            let (rectangle_global_transform, rectangle_state) = rectangle_query.single();
            let ball_x = ball_global_transform.translation().x;
            let rectangle_x = rectangle_global_transform.translation().x;
            let rectangle_width = rectangle_state.width;
            // value between -1 and 1
            let x_direction = (ball_x - rectangle_x) / rectangle_width;
            // normalize y
            *linear_velocity = LinearVelocity(linear_velocity.normalize());
            linear_velocity.x = x_direction;
            // normalize again to get back to original speed
            *linear_velocity = LinearVelocity(linear_velocity.normalize());
            linear_velocity.x = linear_velocity.x * ball_state.speed;
            linear_velocity.y = linear_velocity.y * ball_state.speed;
        }
    }
}

fn on_collision_with_floor(
    mut floor_query: Query<(&CollidingEntities, &mut Floor)>,
    mut game_state: ResMut<GameState>,
    mut ball_query: Query<(&mut LinearVelocity, &mut Transform, &mut BallState)>,
    player_rectangle_query: Query<&GlobalTransform, With<PlayerRectangleState>>,
    time: Res<Time>,
) {
    if let Ok((colliding_entities, mut floor_state)) = floor_query.get_single_mut() {
        floor_state.hit_timer.tick(time.delta());
        // only react to first collision
        if !(colliding_entities.0.is_empty()) && floor_state.hit_timer.elapsed_secs() > 0.05 {
            floor_state.hit_timer.reset();
            game_state.lives -= 1;
            let (mut linear_velocity, mut transform, mut ball_state) = ball_query.single_mut();
            let rectangle_global_transform = player_rectangle_query.single().translation();
            ball_state.active = false;
            linear_velocity.x = 0.0;
            linear_velocity.y = 0.0;
            transform.translation.x = ball_state.initial_position.x + rectangle_global_transform.x;
            transform.translation.y = ball_state.initial_position.y;
        }
    }
}
fn move_inactive_ball(
    mut ball_query: Query<(&mut Transform, &GlobalTransform, &BallState)>,
    player_rectangle_query: Query<&PlayerRectangleState>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    window: Query<&Window>,
) {
    let (mut transform, global_transform, ball_state) = ball_query.single_mut();
    let window = window.single();
    let rectangle_width = player_rectangle_query.single().width;
    if ball_state.active == false {
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
}

fn move_rectangle(
    mut rectangle: Query<(&GlobalTransform, &mut Transform, &PlayerRectangleState)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    window: Query<&Window>,
) {
    let (global_transform, mut transform, state) = rectangle.single_mut();
    let window = window.single();
    if keyboard_input.pressed(KeyCode::ArrowLeft)
        && global_transform.translation().x - state.width / 2.0 >= -window.width() / 2.0
    {
        transform.translation += Vec3 {
            x: -15.0,
            y: 0.0,
            z: 0.0,
        }; } else if keyboard_input.pressed(KeyCode::ArrowRight)
        && global_transform.translation().x + state.width / 2.0 <= window.width() / 2.0
    {
        transform.translation += Vec3 {
            x: 15.0,
            y: 0.0,
            z: 0.0,
        };
    }
}

fn on_collision_with_brick(
    mut brick_query: Query<(&CollidingEntities, &mut BrickState)>,
    time: Res<Time>,
) {

    for (brick_entities, mut brick_state) in &mut brick_query {
        brick_state.hit_timer.tick(time.delta());
        if !(brick_entities.0.is_empty()) {
            // only react to first collision
            println!("{:?}", brick_state.hit_timer.elapsed_secs());
            if brick_state.hit_timer.elapsed_secs() > 0.05 {
                brick_state.hit_bar -= 1;
                brick_state.hit_timer.reset();
            }
        }

    }
}

fn remove_brick(
    mut commands: Commands,
    mut brick_query: Query<(Entity, &mut BrickState)>,
    time: Res<Time>,
) {
    for (entity, brick_state) in &mut brick_query {
        // ensure that one loop has passed before despawning so physics will be applied
        if brick_state.hit_bar == 0 && brick_state.hit_timer.elapsed_secs() > time.delta_seconds() as f32 {
            commands.entity(entity).despawn();
        }
    }
}
