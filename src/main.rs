use bevy::prelude::*;
use bevy::window::WindowResolution;
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};

// Global state for our GUI (similar to the original implementation)
static CLICK_COUNT: AtomicI32 = AtomicI32::new(0);
static BUTTON_STATE: AtomicBool = AtomicBool::new(false);

// Resources for Bevy ECS
#[derive(Resource)]
struct GuiState {
    click_count: i32,
    button_state: bool,
    message: String,
}

impl Default for GuiState {
    fn default() -> Self {
        Self {
            click_count: 0,
            button_state: false,
            message: "Welcome to Rust GUI!".to_string(),
        }
    }
}

// Components for UI elements
#[derive(Component)]
struct ClickButton;

#[derive(Component)]
struct CounterLabel;

#[derive(Component)]
struct StatusLabel;

#[derive(Component)]
struct MessageLabel;

#[derive(Component)]
struct ResetButton;

#[derive(Component)]
struct InfoButton;

#[derive(Component)]
struct DoubleClickButton;

#[derive(Component)]
struct SpecialEffectButton;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Rust GUI on iOS".to_string(),
                resolution: WindowResolution::new(375, 667), // iPhone resolution
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(GuiState::default())
        .add_systems(Startup, setup_ui)
        .add_systems(
            Update,
            (
                handle_click_button,
                handle_reset_button,
                handle_info_button,
                handle_double_click_button,
                handle_special_effect_button,
                update_ui_text,
            ),
        )
        .run();
}

fn setup_ui(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2d);

    // Root UI container
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(20.0),
                padding: UiRect::all(Val::Px(20.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("Rust GUI on iOS"),
                TextColor(Color::WHITE),
                TextFont {
                    font_size: 32.0,
                    ..default()
                },
            ));

            // Status label
            parent.spawn((
                Text::new("Button State: OFF"),
                TextColor(Color::WHITE),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                StatusLabel,
            ));

            // Counter label
            parent.spawn((
                Text::new("Clicks: 0"),
                TextColor(Color::WHITE),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                CounterLabel,
            ));

            // Message label
            parent.spawn((
                Text::new("Welcome to Rust GUI!"),
                TextColor(Color::WHITE),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                MessageLabel,
            ));

            // Main click button
            parent.spawn((
                Button,
                Node {
                    width: Val::Px(300.0),
                    height: Val::Px(60.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    margin: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.2, 0.4, 1.0)),
                ClickButton,
                children![(
                    Text::new("Click Me! (Rust GUI)"),
                    TextColor(Color::WHITE),
                    TextFont {
                        font_size: 20.0,
                        ..default()
                    },
                )],
            ));

            // Button row container
            parent.spawn((
                Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(10.0),
                    ..default()
                },
                children![
                    (
                        Button,
                        Node {
                            width: Val::Px(140.0),
                            height: Val::Px(40.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.3, 0.3, 0.3)),
                        InfoButton,
                        children![(
                            Text::new("Get Info"),
                            TextColor(Color::WHITE),
                            TextFont {
                                font_size: 16.0,
                                ..default()
                            },
                        )],
                    ),
                    (
                        Button,
                        Node {
                            width: Val::Px(140.0),
                            height: Val::Px(40.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.3, 0.3, 0.3)),
                        DoubleClickButton,
                        children![(
                            Text::new("Double Click"),
                            TextColor(Color::WHITE),
                            TextFont {
                                font_size: 16.0,
                                ..default()
                            },
                        )],
                    ),
                ],
            ));

            // Special effect button
            parent.spawn((
                Button,
                Node {
                    width: Val::Px(200.0),
                    height: Val::Px(40.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    margin: UiRect::all(Val::Px(5.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.3, 0.3, 0.3)),
                SpecialEffectButton,
                children![(
                    Text::new("Special Effect"),
                    TextColor(Color::WHITE),
                    TextFont {
                        font_size: 16.0,
                        ..default()
                    },
                )],
            ));

            // Reset button
            parent.spawn((
                Button,
                Node {
                    width: Val::Px(160.0),
                    height: Val::Px(40.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    margin: UiRect::vertical(Val::Px(20.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.8, 0.2, 0.2)),
                ResetButton,
                children![(
                    Text::new("Reset GUI"),
                    TextColor(Color::WHITE),
                    TextFont {
                        font_size: 16.0,
                        ..default()
                    },
                )],
            ));
        });
}

fn handle_click_button(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<ClickButton>),
    >,
    mut gui_state: ResMut<GuiState>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = Color::srgb(0.1, 0.3, 0.8).into();

                // Update state
                gui_state.click_count += 1;
                gui_state.button_state = !gui_state.button_state;

                // Update global atomics for compatibility
                CLICK_COUNT.store(gui_state.click_count, Ordering::Relaxed);
                BUTTON_STATE.store(gui_state.button_state, Ordering::Relaxed);

                // Update message
                if gui_state.click_count == 1 {
                    gui_state.message = "Button clicked for the first time!".to_string();
                } else {
                    gui_state.message = format!(
                        "Button clicked {} times! Current state: {}",
                        gui_state.click_count,
                        if gui_state.button_state { "ON" } else { "OFF" }
                    );
                }

                println!("Button clicked, count: {}", gui_state.click_count);
            }
            Interaction::Hovered => {
                *color = if gui_state.button_state {
                    Color::srgb(0.3, 0.7, 0.3).into()
                } else {
                    Color::srgb(0.3, 0.5, 1.2).into()
                };
            }
            Interaction::None => {
                *color = if gui_state.button_state {
                    Color::srgb(0.2, 0.6, 0.2).into()
                } else {
                    Color::srgb(0.2, 0.4, 1.0).into()
                };
            }
        }
    }
}

fn handle_reset_button(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<ResetButton>),
    >,
    mut gui_state: ResMut<GuiState>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = Color::srgb(0.6, 0.1, 0.1).into();

                // Reset state
                *gui_state = GuiState::default();
                CLICK_COUNT.store(0, Ordering::Relaxed);
                BUTTON_STATE.store(false, Ordering::Relaxed);

                println!("GUI Reset!");
            }
            Interaction::Hovered => {
                *color = Color::srgb(1.0, 0.3, 0.3).into();
            }
            Interaction::None => {
                *color = Color::srgb(0.8, 0.2, 0.2).into();
            }
        }
    }
}

fn handle_info_button(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<InfoButton>),
    >,
    gui_state: Res<GuiState>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = Color::srgb(0.2, 0.2, 0.2).into();

                let info = format!(
                    "GUI Info: {} clicks, state: {}",
                    gui_state.click_count,
                    if gui_state.button_state { "ON" } else { "OFF" }
                );
                println!("{}", info);
            }
            Interaction::Hovered => {
                *color = Color::srgb(0.4, 0.4, 0.4).into();
            }
            Interaction::None => {
                *color = Color::srgb(0.3, 0.3, 0.3).into();
            }
        }
    }
}

fn handle_double_click_button(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<DoubleClickButton>),
    >,
    mut gui_state: ResMut<GuiState>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = Color::srgb(0.2, 0.2, 0.2).into();

                // Double click simulation
                gui_state.click_count += 2;
                gui_state.button_state = !gui_state.button_state;

                CLICK_COUNT.store(gui_state.click_count, Ordering::Relaxed);
                BUTTON_STATE.store(gui_state.button_state, Ordering::Relaxed);

                let message = format!("Double click! Total: {} clicks", gui_state.click_count);
                println!("{}", message);
                gui_state.message = message;
            }
            Interaction::Hovered => {
                *color = Color::srgb(0.4, 0.4, 0.4).into();
            }
            Interaction::None => {
                *color = Color::srgb(0.3, 0.3, 0.3).into();
            }
        }
    }
}

fn handle_special_effect_button(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<SpecialEffectButton>),
    >,
    mut gui_state: ResMut<GuiState>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = Color::srgb(0.2, 0.2, 0.2).into();

                let message = if gui_state.click_count > 10 {
                    "Amazing! You're a clicking master!"
                } else if gui_state.click_count > 5 {
                    "Good job! Keep clicking!"
                } else {
                    "Just getting started!"
                };

                println!("{}", message);
                gui_state.message = message.to_string();
            }
            Interaction::Hovered => {
                *color = Color::srgb(0.4, 0.4, 0.4).into();
            }
            Interaction::None => {
                *color = Color::srgb(0.3, 0.3, 0.3).into();
            }
        }
    }
}

fn update_ui_text(
    gui_state: Res<GuiState>,
    mut counter_query: Query<
        &mut Text,
        (
            With<CounterLabel>,
            Without<StatusLabel>,
            Without<MessageLabel>,
        ),
    >,
    mut status_query: Query<
        &mut Text,
        (
            With<StatusLabel>,
            Without<CounterLabel>,
            Without<MessageLabel>,
        ),
    >,
    mut message_query: Query<
        &mut Text,
        (
            With<MessageLabel>,
            Without<CounterLabel>,
            Without<StatusLabel>,
        ),
    >,
) {
    if gui_state.is_changed() {
        // Update counter
        for mut text in &mut counter_query {
            **text = format!("Clicks: {}", gui_state.click_count);
        }

        // Update status
        for mut text in &mut status_query {
            **text = format!(
                "Button State: {}",
                if gui_state.button_state { "ON" } else { "OFF" }
            );
        }

        // Update message
        for mut text in &mut message_query {
            **text = gui_state.message.clone();
        }
    }
}
