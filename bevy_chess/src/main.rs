use bevy::{prelude::*, window::{EnabledButtons, PresentMode, WindowTheme}};

mod pieces;
mod board;
use {pieces::*, board::*};

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
