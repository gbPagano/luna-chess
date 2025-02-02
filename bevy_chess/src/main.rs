use bevy::{prelude::*, window::{EnabledButtons, PresentMode, WindowTheme}};

mod pieces;
use pieces::*;

fn main() {
    App::new()
            .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: String::from("Chess"),
                resolution: Vec2::new(800., 800.).into(),
                resizable: false,
                fit_canvas_to_parent: true,
                present_mode: PresentMode::AutoVsync,
                window_theme: Some(WindowTheme::Dark),
                enabled_buttons: EnabledButtons {
                    maximize: false,
                    ..default()
                },
                ..default()
            }),
            ..default()
        }))
        .init_resource::<SelectedPiece>()
        .insert_resource(Board::new())
        .add_systems(Startup, (setup, spawn_board, spawn_pieces))
        .add_systems(Update, update_piece)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn spawn_board(mut commands: Commands) {
    let tile_size = 100.0;
    let colors = [Color::srgb(0.92, 0.93, 0.82), Color::srgb(0.46, 0.59, 0.34)];

    for row in 0..8 {
        for col in 0..8 {
            let color = colors[(row + col) % 2];
            let x = col as f32 * tile_size - (4.0 * tile_size) + tile_size / 2.0;
            let y = row as f32 * tile_size - (4.0 * tile_size) + tile_size / 2.0;

            commands.spawn((
                Sprite {
                    color,
                    custom_size: Some(Vec2::splat(tile_size)),
                    ..default()
                },
                Transform::from_xyz(x, y, 0.0),
            ));
        }
    }
}


