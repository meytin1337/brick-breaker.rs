use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use std::f32::consts::PI;

pub struct CreateObjectsPlugin;

#[derive(Component, Debug)]
pub struct BallState {
    pub radius: f32,
    pub speed: f32,
    pub direction: Vec3,
    pub active: bool,
}

#[derive(Component, Debug)]
pub struct RectangleState {
    pub width: f32,
    pub height: f32,
    pub player_controlled: bool,
    pub hit_bar: i32,
}

#[derive(Bundle)]
pub struct RectangleBundle {
    material_mesh: MaterialMesh2dBundle<ColorMaterial>,
    rectangle_state: RectangleState,
}

#[derive(Component, Debug)]
pub struct PlayerRectangle;

impl Plugin for CreateObjectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
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
    let ball = Mesh2dHandle(meshes.add(Circle {
        radius
    }));
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: ball,
            material: materials.add(Color::rgb(1.0, 0.0, 0.0)),
            transform: Transform {
                rotation: Quat::from_rotation_z(PI / 2.0),
                translation: Vec3 {
                    x: 0.0,
                    // add offset to prevent ball from getting stuck
                    y: -window.height() / 2.0 + radius * 2.0 + window.height() / 60.0,
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
        BallState {
            radius,
            speed: 400.0,
            direction: Vec3::new(0.0, 1.0, 0.0),
            active: false,
        },
    ));
    let rectangle =
        Mesh2dHandle(meshes.add(Rectangle::new(window.height() / 40.0, window.width() / 7.0)));
    commands.spawn((
        RectangleBundle {
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
            rectangle_state: RectangleState {
                width: window.width() / 6.0,
                height: window.height() / 20.0,
                player_controlled: true,
                hit_bar: 1,
            },
        },
        PlayerRectangle,
    ));
    for i in -2..3 {
        for j in -2..3 {
            let rectangle_width = window.width() / 8.;
            let rectangle_height = window.height() / 20.;
            let rectangle =
                Mesh2dHandle(meshes.add(Rectangle::new(rectangle_height, rectangle_width)));
            commands.spawn(RectangleBundle {
                material_mesh: MaterialMesh2dBundle {
                    mesh: rectangle,
                    material: materials.add(Color::rgb(0.0, 1.0, 0.0)),
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
                rectangle_state: RectangleState {
                    width: rectangle_width,
                    height: rectangle_height,
                    player_controlled: false,
                    hit_bar: if j % 2 == 0 { 3 } else { 1 },
                },
            });
        }
    }
}
