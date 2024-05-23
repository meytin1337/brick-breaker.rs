use crate::game_logic::RecreateBricksEvent;
use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_xpbd_2d::prelude::*;
use std::f32::consts::PI;
use std::time::Duration;

pub struct CreateObjectsPlugin;

#[derive(Component)]
pub struct BallState {
    pub radius: f32,
    pub speed: f32,
    pub direction: Vec3,
    pub active: bool,
    pub initial_position: Vec3,
}

#[derive(Component)]
pub struct Walls;

#[derive(Component)]
pub struct Floor {
    pub hit_timer: Timer,
}

#[derive(Component)]
pub struct BrickState {
    pub width: f32,
    pub height: f32,
    pub hit_bar: i32,
    pub hit_timer: Timer,
}

#[derive(Component)]
pub struct PlayerRectangleState {
    pub width: f32,
    pub height: f32,
}

#[derive(Bundle)]
pub struct BrickBundle {
    friction: Friction,
    restitution: Restitution,
    body: RigidBody,
    collider: Collider,
    material_mesh: MaterialMesh2dBundle<ColorMaterial>,
    state: BrickState,
}

#[derive(Bundle)]
pub struct PlayerRectangleBundle {
    friction: Friction,
    restitution: Restitution,
    body: RigidBody,
    collider: Collider,
    material_mesh: MaterialMesh2dBundle<ColorMaterial>,
    state: PlayerRectangleState,
}

#[derive(Component)]
pub struct PlayerRectangle;

impl Plugin for CreateObjectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, recreate_bricks);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Query<&Window>,
) {
    commands.spawn(Camera2dBundle::default());
    let window = window.single();
    let radius = window.height() / 38.0;
    let ball_state = BallState {
        radius,
        speed: 800.0,
        direction: Vec3::new(0.0, 1.0, 0.0),
        active: false,
        initial_position: Vec3::new(0.0, -window.height() / 2.0 + radius * 2.0, 0.0),
    };
    let ball = Mesh2dHandle(meshes.add(Circle { radius }));
    let mut impulse = ExternalImpulse::new(Vec2::new(0.0, 0.0));
    commands.spawn((
        RigidBody::Dynamic,
        Friction::new(0.0),
        Restitution::new(1.0),
        impulse,
        GravityScale(0.0),
        Rotation::from_degrees(0.0),
        Collider::circle(radius),
        LinearVelocity(Vec2::new(0.0, 0.0)),
        MaterialMesh2dBundle {
            mesh: ball,
            material: materials.add(Color::rgb(1.0, 0.0, 0.0)),
            transform: Transform {
                rotation: Quat::from_rotation_z(PI / 2.0),
                translation: Vec3 {
                    x: 0.0,
                    // add offset to prevent ball from getting stuck
                    y: -window.height() / 2.0 + radius * 2.0, // + window.height() / 60.0,
                    z: 0.0,
                },
                scale: Vec3 {
                    x: 1.0,
                    y: 1.0,
                    z: 1.0,
                },
            },
            ..default()
        },
        ball_state,
    ));
    impulse.apply_impulse(Vec2::X);
    let rectangle_width = window.width() / 7.0;
    let rectangle_height = window.height() / 40.0;
    let rectangle = Mesh2dHandle(meshes.add(Rectangle::new(rectangle_height, rectangle_width)));
    let state = PlayerRectangleState {
        width: rectangle_width,
        height: rectangle_height,
    };
    commands.spawn((
        PlayerRectangleBundle {
            body: RigidBody::Static,
            friction: Friction::new(0.0),
            restitution: Restitution::new(1.0),
            collider: Collider::rectangle(rectangle_height, rectangle_width),
            material_mesh: MaterialMesh2dBundle {
                mesh: rectangle,
                material: materials.add(Color::rgb(0.0, 0.0, 1.0)),
                transform: Transform {
                    rotation: Quat::from_rotation_z(PI / 2.0),
                    translation: Vec3 {
                        x: 0.0,
                        y: -window.height() / 2.0 + window.height() / 100.0,
                        z: 0.0,
                    },
                    scale: Vec3 {
                        x: 1.0,
                        y: 1.0,
                        z: 1.0,
                    },
                },
                ..default()
            },
            state,
        },
        PlayerRectangle,
    ));
    commands.spawn((
        RigidBody::Static,
        Collider::polyline(
            vec![
                Vec2::new(-window.width() / 2.0, -window.height() / 2.0),
                Vec2::new(-window.width() / 2.0, window.height() / 2.0),
                Vec2::new(window.width() / 2.0, window.height() / 2.0),
                Vec2::new(window.width() / 2.0, -window.height() / 2.0),
            ],
            None,
        ),
        Restitution::new(1.0),
        Friction::new(0.0),
        Walls,
    ));
    commands.spawn((
        RigidBody::Static,
        Collider::polyline(
            vec![
                Vec2::new(-window.width() / 2.0, -window.height() / 2.0),
                Vec2::new(window.width() / 2.0, -window.height() / 2.0),
            ],
            None,
        ),
        Restitution::new(1.0),
        Friction::new(0.0),
        Floor {
            hit_timer: Timer::new(Duration::from_secs_f32(10000.0), TimerMode::Repeating),
        },
    ));
}

pub fn recreate_bricks(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Query<&Window>,
    mut recreate_bricks_event: EventReader<RecreateBricksEvent>,
    brick_query: Query<Entity, With<BrickState>>,
) {
   for _ in recreate_bricks_event.read() {
        for entity in &brick_query {
            commands.entity(entity).despawn();
        }
        for i in -2..3 {
            for j in -2..3 {
                let window = window.single();
                let rectangle_width = window.width() / 8.;
                let rectangle_height = window.height() / 20.;
                let rectangle =
                    Mesh2dHandle(meshes.add(Rectangle::new(rectangle_height, rectangle_width)));
                let state = BrickState {
                    width: rectangle_width,
                    height: rectangle_height,
                    hit_timer: Timer::new(Duration::from_secs_f32(1000.0), TimerMode::Repeating),
                    hit_bar: if (j % 2 == 0 && (i == 1 || i == -1)) || (j == 0 && i == 0) {
                        3
                    } else {
                        1
                    },
                };
                commands.spawn(BrickBundle {
                    friction: Friction::new(0.0),
                    restitution: Restitution::new(1.0),
                    body: RigidBody::Static,
                    collider: Collider::rectangle(rectangle_height, rectangle_width),
                    material_mesh: MaterialMesh2dBundle {
                        mesh: rectangle,
                        material: materials.add(Color::rgb(0.0, 1.0 / state.hit_bar as f32, 0.0)),
                        transform: Transform {
                            rotation: Quat::from_rotation_z(PI / 2.0),
                            translation: Vec3 {
                                x: j as f32 * (rectangle_width + window.width() / 40.),
                                y: i as f32 * (rectangle_height + window.height() / 30.)
                                    + window.height() / 4.0,
                                z: 0.0,
                            },
                            scale: Vec3 {
                                x: 1.0,
                                y: 1.0,
                                z: 1.0,
                            },
                        },
                        ..default()
                    },
                    state,
                });
            }
        }
    }
}
