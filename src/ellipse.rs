pub mod shape {
    use bevy::{
        prelude::*,
        sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    };

    pub struct ShapePlugin;

    impl Plugin for ShapePlugin {
        fn build(&self, app: &mut App) {
            app.add_systems(Startup, setup);
            app.add_system(Update, move_cube);
        }
    }
    fn setup(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
        commands.spawn(Camera2dBundle::default());
        let shape = Mesh2dHandle(meshes.add(Circle { radius: 50.0 }));
        commands.spawn(MaterialMesh2dBundle {
            mesh: shape,
            material: materials.add(Color::rgb(1.0, 0.0, 0.0)),
            ..default()
        });
    }
    fn move_cube(mut cubes: Query<(&mut Transform, &mut CubeState)>, timer: Res<Time>) {
        for (mut transform, cube) in &mut cubes {
            // Move the cube forward smoothly at a given move_speed.
            let forward = transform.forward();
            transform.translation += forward * cube.move_speed * timer.delta_seconds();
        }
    }
}
