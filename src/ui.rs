pub mod ui_mod {
    use bevy::prelude::*;

    pub struct UiPlugin;

    impl Plugin for UiPlugin {
        fn build(&self, app: &mut AppBuilder) {
            app.add_systems(Startup, setup);
        }
    }

    fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut materials: ResMut<Assets<ColorMaterial>) {
        commands.spawn_bundle(TextBundle {
            text: Text {
                sections: vec![
                    TextSection {
                        value: "Hello, Bevy!".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 60.0,
                            color: Color::WHITE,
                        },
                    },
                ],
                alignment: TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                },
            },
            ..Default::default()
        });
    }
}
