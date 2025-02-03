use bevy::{prelude::*, utils::HashMap};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PieceColor {
    White,
    Black,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PieceType {
    Pawn(PieceColor),
    Knight(PieceColor),
    Bishop(PieceColor),
    Rock(PieceColor),
    Queen(PieceColor),
    King(PieceColor),
}

#[derive(Default, Resource)]
pub struct SelectedPiece {
    pub position: Option<(usize, usize)>,
}

#[derive(Component)]
pub struct PiecePosition {
    pub x: usize,
    pub y: usize,
}

#[derive(Resource)]
pub struct Board {
    pub pieces: HashMap<(usize, usize), PieceType>,
}

impl Board {
    pub fn new() -> Self {
        let mut pieces = HashMap::new();

        // adds pawns
        for i in 0..8 {
            pieces.insert((i, 1), PieceType::Pawn(PieceColor::White));
            pieces.insert((i, 6), PieceType::Pawn(PieceColor::Black));
        }

        // adds rocks
        pieces.insert((0, 0), PieceType::Rock(PieceColor::White));
        pieces.insert((7, 0), PieceType::Rock(PieceColor::White));
        pieces.insert((0, 7), PieceType::Rock(PieceColor::Black));
        pieces.insert((7, 7), PieceType::Rock(PieceColor::Black));

        // add knights
        pieces.insert((1, 0), PieceType::Knight(PieceColor::White));
        pieces.insert((6, 0), PieceType::Knight(PieceColor::White));
        pieces.insert((1, 7), PieceType::Knight(PieceColor::Black));
        pieces.insert((6, 7), PieceType::Knight(PieceColor::Black));

        // adds bishops
        pieces.insert((2, 0), PieceType::Bishop(PieceColor::White));
        pieces.insert((5, 0), PieceType::Bishop(PieceColor::White));
        pieces.insert((2, 7), PieceType::Bishop(PieceColor::Black));
        pieces.insert((5, 7), PieceType::Bishop(PieceColor::Black));

        // adds queens
        pieces.insert((3, 0), PieceType::Queen(PieceColor::White));
        pieces.insert((3, 7), PieceType::Queen(PieceColor::Black));

        // adds kings
        pieces.insert((4, 0), PieceType::King(PieceColor::White));
        pieces.insert((4, 7), PieceType::King(PieceColor::Black));

        Self { pieces }
    }

}

pub fn spawn_pieces(mut commands: Commands, asset_server: Res<AssetServer>) {
    let tile_size = 100.0;

    let pieces = vec![
        ("a2", "wP.png"), ("b2", "wP.png"), ("c2", "wP.png"), ("d2", "wP.png"), 
        ("e2", "wP.png"), ("f2", "wP.png"), ("g2", "wP.png"), ("h2", "wP.png"), // white pawns

        ("a7", "bP.png"), ("b7", "bP.png"), ("c7", "bP.png"), ("d7", "bP.png"), 
        ("e7", "bP.png"), ("f7", "bP.png"), ("g7", "bP.png"), ("h7", "bP.png"), // black pawns

        ("a1", "wR.png"), ("h1", "wR.png"), // white rocks
        ("a8", "bR.png"), ("h8", "bR.png"), // black rocks

        ("b1", "wN.png"), ("g1", "wN.png"), // white knights
        ("b8", "bN.png"), ("g8", "bN.png"), // black knights

        ("c1", "wB.png"), ("f1", "wB.png"), // white bishops
        ("c8", "bB.png"), ("f8", "bB.png"), // black bishops

        ("d1", "wQ.png"), ("d8", "bQ.png"), // queens
        ("e1", "wK.png"), ("e8", "bK.png"), // kings
    ];

    for (pos, sprite) in pieces {
        let col = pos.chars().nth(0).unwrap() as u8 - b'a';
        let row = pos.chars().nth(1).unwrap().to_digit(10).unwrap() - 1;

        let x = col as f32 * tile_size - (4.0) * tile_size + tile_size / 2.0;
        let y = row as f32 * tile_size - (4.0) * tile_size + tile_size / 2.0;

        commands.spawn((Sprite {
            image: asset_server.load(sprite),
            custom_size: Some(Vec2::splat(tile_size * 0.8)),
            ..default()
        },
        Transform::from_xyz(x, y, 1.0),
        PiecePosition {
            x: col as usize,
            y: row as usize,
        },
        ));
    }
}

pub fn update_piece(
    buttons: Res<ButtonInput<MouseButton>>, 
    windows: Query<&Window>, 
    camera_query: Query<(&Camera, &GlobalTransform)>,
    board: Res<Board>,
    mut selected: ResMut<SelectedPiece>,
    mut query: Query<(&mut Transform, &mut PiecePosition)>,
    mut commands: Commands
    ) 
{
    if buttons.just_pressed(MouseButton::Left) {
        println!("{:?}", selected.position);
        let window = windows.single();
        let (camera, camera_transform) = camera_query.single();

        if let Some(world_pos) = window.cursor_position() {
            if let Ok(ray) = camera.viewport_to_world(camera_transform, world_pos) {
                let world_pos = ray.origin.truncate();
                let tile_width = 100.0;
                let x = ((world_pos.x + 4.0 * tile_width) / tile_width).floor() as usize;
                let y = ((world_pos.y + 4.0 * tile_width) / tile_width).floor() as usize;

                match selected.position {
                    None => {
                        if board.pieces.contains_key(&(x,y)) {
                            selected.position = Some((x,y));
                            println!("Peça selecionada em: ({}, {})", x, y);
                        }
                    }

                    Some((old_x, old_y)) => {
                        if (x,y) != (old_x, old_y) {
                            if let Some(piece) = board.pieces.get(&(old_x, old_y)) {
                                selected.position = None;
                                
                                let mut new_pieces = board.pieces.clone();
                                new_pieces.remove(&(old_x, old_y));
                                new_pieces.insert((x, y), *piece);

                                commands.insert_resource(Board {
                                    pieces: new_pieces,
                                });

                                for (mut transform, mut pos) in &mut query {
                                    if pos.x == old_x && pos.y == old_y {
                                        let new_x = x as f32 * tile_width - (4.0 * tile_width) + tile_width / 2.0;
                                        let new_y = y as f32 * tile_width - (4.0 * tile_width) + tile_width / 2.0;

                                        pos.x = x;
                                        pos.y = y;

                                        transform.translation.x = new_x;
                                        transform.translation.y = new_y;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}