use bevy::prelude::*;

use crate::game_logic::{GameOverEvent, GameState, GameWonEvent, HideContainersEvent};

#[derive(Event)]
pub struct NewGameEvent;


const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

#[derive(Component)]
struct Container;

#[derive(Component)]
pub struct GameOverContainer;

#[derive(Component)]
pub struct GameWonContainer;

#[derive(Component)]
pub struct LivesCounter;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, (button_system, lives_counter, hide_containers, display_game_over, display_game_won))
            .add_event::<NewGameEvent>();
            
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        // new game
        .spawn((NodeBundle {
            background_color: BackgroundColor(Color::rgba(0.1, 0.1, 0.1, 0.99)),
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            ..default()
        }, Container))
        .with_children(|parent| {
            // left vertical fill (border)
            parent.spawn((
                TextBundle::from_section(
                    "Welcome to Brick Breaker!",
                    TextStyle {
                        font: asset_server.load("embedded://fonts/AgaveNerdFont-Regular.ttf"),
                        font_size: 60.0,
                        ..default()
                    },
                )
                .with_style(Style {
                    margin: UiRect::all(Val::Px(5.0)),
                    ..default()
                }),
                // Because this is a distinct label widget and
                // not button/list item text, this is necessary
                // for accessibility to treat the text accordingly.
                Label,
            ));
        })
        .with_children(|parent| {
            // left vertical fill (border)
            parent.spawn((
                TextBundle::from_section(
                    "Press the button to start the game!",
                    TextStyle {
                        font: asset_server.load("embedded://fonts/AgaveNerdFont-Regular.ttf"),
                        font_size: 40.0,
                        ..default()
                    },
                )
                .with_style(Style {
                    margin: UiRect::all(Val::Px(5.)),
                    ..default()
                }),
                // Because this is a distinct label widget and
                // not button/list item text, this is necessary
                // for accessibility to treat the text accordingly.
                Label,
            ));
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    background_color: BackgroundColor(NORMAL_BUTTON),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Start",
                        TextStyle {
                            font: asset_server.load("embedded://fonts/AgaveNerdFont-Regular.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });
        // game over
        commands.spawn((NodeBundle {
            background_color: BackgroundColor(Color::rgba(0.1, 0.1, 0.1, 0.99)),
            visibility: Visibility::Hidden,
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            ..default()
        }, Container, GameOverContainer))
        .with_children(|parent| {
            // left vertical fill (border)
            parent.spawn((
                TextBundle::from_section(
                    "Game Over!",
                    TextStyle {
                        font: asset_server.load("embedded://fonts/AgaveNerdFont-Regular.ttf"),
                        font_size: 60.0,
                        ..default()
                    },
                )
                .with_style(Style {
                    margin: UiRect::all(Val::Px(5.)),
                    ..default()
                }),
                // Because this is a distinct label widget and
                // not button/list item text, this is necessary
                // for accessibility to treat the text accordingly.
                Label,
            ));
        })
        .with_children(|parent| {
            // left vertical fill (border)
            parent.spawn((
                TextBundle::from_section(
                    "Press the button to start a new game!",
                    TextStyle {
                        font: asset_server.load("embedded://fonts/AgaveNerdFont-Regular.ttf"),
                        font_size: 40.0,
                        ..default()
                    },
                )
                .with_style(Style {
                    margin: UiRect::all(Val::Px(5.)),
                    ..default()
                }),
                // Because this is a distinct label widget and
                // not button/list item text, this is necessary
                // for accessibility to treat the text accordingly.
                Label,
            ));
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    background_color: BackgroundColor(NORMAL_BUTTON),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Start",
                        TextStyle {
                            font: asset_server.load("embedded://fonts/AgaveNerdFont-Regular.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });
        // game won
        commands.spawn((NodeBundle {
            background_color: BackgroundColor(Color::rgba(0.1, 0.1, 0.1, 0.99)),
            visibility: Visibility::Hidden,
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            ..default()
        }, Container, GameWonContainer))
        .with_children(|parent| {
            // left vertical fill (border)
            parent.spawn((
                TextBundle::from_section(
                    "Game won!",
                    TextStyle {
                        font: asset_server.load("embedded://fonts/AgaveNerdFont-Regular.ttf"),
                        font_size: 60.0,
                        ..default()
                    },
                )
                .with_style(Style {
                    margin: UiRect::all(Val::Px(5.)),
                    ..default()
                }),
                // Because this is a distinct label widget and
                // not button/list item text, this is necessary
                // for accessibility to treat the text accordingly.
                Label,
            ));
        })
        .with_children(|parent| {
            // left vertical fill (border)
            parent.spawn((
                TextBundle::from_section(
                    "Press the button to start a new game!",
                    TextStyle {
                        font: asset_server.load("embedded://fonts/AgaveNerdFont-Regular.ttf"),
                        font_size: 40.0,
                        ..default()
                    },
                )
                .with_style(Style {
                    margin: UiRect::all(Val::Px(5.)),
                    ..default()
                }),
                // Because this is a distinct label widget and
                // not button/list item text, this is necessary
                // for accessibility to treat the text accordingly.
                Label,
            ));
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    background_color: BackgroundColor(NORMAL_BUTTON),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Start",
                        TextStyle {
                            font: asset_server.load("embedded://fonts/AgaveNerdFont-Regular.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });
        // lives counter
       commands.spawn((
            TextBundle::from_section(
                "",
                TextStyle {
                    font: asset_server.load("embedded://fonts/AgaveNerdFont-Regular.ttf"),
                    font_size: 25.0,
                    ..default()
                },
            )
            .with_style(Style {
                margin: UiRect::all(Val::Px(5.)),
                ..default()
            }),
            // Because this is a distinct label widget and
            // not button/list item text, this is necessary
            // for accessibility to treat the text accordingly.
            Label, LivesCounter
        ));
}

fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut new_game_event: EventWriter<NewGameEvent>,
) {
    for (interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::RED;
                new_game_event.send(NewGameEvent);
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

fn hide_containers(
    mut hide_containers_event: EventReader<HideContainersEvent>,
    mut visibility_query: Query<&mut Visibility, With<Container>>,
) {
    for _ in hide_containers_event.read() {
        for mut visibility in visibility_query.iter_mut() {
            *visibility = Visibility::Hidden;
        }
    }
}

fn lives_counter(
    game_state: Res<GameState>,
    mut text_query: Query<&mut Text, With<LivesCounter>>,
) {
    // update lives counter
    if game_state.in_game {
        let mut text = text_query.single_mut();
        text.sections[0].value = format!("Lives: {}", game_state.lives);
    }
}

fn display_game_over(
    mut game_over_event: EventReader<GameOverEvent>,
    mut visibility_query: Query<&mut Visibility, With<GameOverContainer>>,
) {
    for _ in game_over_event.read() {
        for mut visibility in visibility_query.iter_mut() {
            *visibility = Visibility::Visible;
        }
    }
}

fn display_game_won(
    mut game_won_event: EventReader<GameWonEvent>,
    mut visibility_query: Query<&mut Visibility, With<GameWonContainer>>,
) {
    for _ in game_won_event.read() {
        for mut visibility in visibility_query.iter_mut() {
            *visibility = Visibility::Visible;
        }
    }
}
