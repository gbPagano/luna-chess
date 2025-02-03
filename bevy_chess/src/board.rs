use bevy::prelude::*;

const COLOR_WHITE: Color = Color::srgb(0.92, 0.93, 0.82);
const COLOR_BLACK: Color = Color::srgb(0.46, 0.59, 0.34);
const COLOR_HIGHLIGHT: Color = Color::srgb(0.56, 0.59, 0.34);

#[derive(Debug, Component)]
pub struct TilePosition {
    pub x: u8,
    pub y: u8,
}

pub fn spawn_board(mut commands: Commands) {
    let tile_size = 100.;
    let colors = [COLOR_WHITE, COLOR_BLACK];

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
                TilePosition {
                    x: row as u8,
                    y: col as u8,
                }
            ));
        }
    }
}