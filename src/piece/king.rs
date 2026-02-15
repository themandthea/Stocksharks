use crate::board_utils::chessboard::{Board, ChessBoard};
use crate::utils::{Color, Coordinate, Piece, Square};

pub trait King {
    fn king_move(&self, square: &Square) -> Vec<Square>;
    fn is_in_check(&self, color: &Color) -> bool;
}

impl King for Board {
    fn king_move(&self, square: &Square) -> Vec<Square> {
        let mut moves = Vec::new();
        if let Some(Piece::King(color)) = square.get_piece() {
            // Kings moves logic

            let knight_moves: [(i8, i8); 8] = [
                (1, 1),
                (0, -1),
                (0, 1),
                (-1, -1),
                (1, -1),
                (1, 0),
                (-1, 1),
                (-1, 0),
            ];

            for &(dx, dy) in &knight_moves {
                let new_line = square.get_line().unwrap() as i8 + dx;
                let new_column = square.get_column().unwrap() as i8 + dy;
                if !(0..=7).contains(&new_line) || !(0..=7).contains(&new_column) {
                    continue; // Skip out of bounds
                }
                let new_coordinate = Coordinate::new(
                    (square.get_column().unwrap() as i8 + dy) as u8,
                    (square.get_line().unwrap() as i8 + dx) as u8,
                );
                let new_square = Square::new(Some(Piece::King(*color)), new_coordinate);
                if self.get_square(new_coordinate).unwrap().available()
                    || self
                        .get_square(new_coordinate)
                        .unwrap()
                        .occupied_by_oponent(color)
                {
                    moves.push(new_square);
                }
            }
            // Castling logic
            // Long castle (Queen-side) for white
            if self.long_castle.0 && color == &Color::White && !self.is_in_check(color) {
                // Vérifier que les cases entre le roi et la tour sont vides
                let e1 = Coordinate::new(4, 0); // Position du roi blanc
                let d1 = Coordinate::new(3, 0);
                let c1 = Coordinate::new(2, 0);
                let b1 = Coordinate::new(1, 0);
                let a1 = Coordinate::new(0, 0); // Position de la tour pour grand roque

                // Vérifier que les cases entre le roi et la tour sont vides
                if self.get_square(d1).unwrap().available()
                    && self.get_square(c1).unwrap().available()
                    && self.get_square(b1).unwrap().available()
                {
                    // Vérifier que la pièce en a1 est bien une tour blanche
                    if let Some(square) = self.get_square(a1) {
                        if let Some(Piece::Rook(piece_color)) = square.get_piece() {
                            if piece_color == color {
                                // Vérifier que le roi ne passe pas par une case en échec
                                // Pour cela, on simule un déplacement du roi sur les cases
                                // qu'il va traverser (d1 et c1) et on vérifie s'il est en échec

                                // Simulation pour d1
                                let mut temp_board = self.clone();
                                temp_board.set_piece(e1, None).unwrap();
                                temp_board.set_piece(d1, Some(Piece::King(*color))).unwrap();
                                if !temp_board.is_in_check(color) {
                                    // Simulation pour c1 (position finale du roi)
                                    let mut temp_board = self.clone();
                                    temp_board.set_piece(e1, None).unwrap();
                                    temp_board.set_piece(c1, Some(Piece::King(*color))).unwrap();
                                    if !temp_board.is_in_check(color) {
                                        // Le grand roque est légal, ajouter le mouvement
                                        let new_square = Square::new(Some(Piece::King(*color)), c1);
                                        moves.push(new_square);
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Long castle (Queen-side) for black
            if self.long_castle.1 && color == &Color::Black && !self.is_in_check(color) {
                // Vérifier que les cases entre le roi et la tour sont vides
                let e8 = Coordinate::new(4, 7); // Position du roi noir
                let d8 = Coordinate::new(3, 7);
                let c8 = Coordinate::new(2, 7);
                let b8 = Coordinate::new(1, 7);
                let a8 = Coordinate::new(0, 7); // Position de la tour pour grand roque

                // Vérifier que les cases entre le roi et la tour sont vides
                if self.get_square(d8).unwrap().available()
                    && self.get_square(c8).unwrap().available()
                    && self.get_square(b8).unwrap().available()
                {
                    // Vérifier que la pièce en a8 est bien une tour noire
                    if let Some(square) = self.get_square(a8) {
                        if let Some(Piece::Rook(piece_color)) = square.get_piece() {
                            if piece_color == color {
                                // Vérifier que le roi ne passe pas par une case en échec
                                // Simulation pour d8
                                let mut temp_board = self.clone();
                                temp_board.set_piece(e8, None).unwrap();
                                temp_board.set_piece(d8, Some(Piece::King(*color))).unwrap();
                                if !temp_board.is_in_check(color) {
                                    // Simulation pour c8 (position finale du roi)
                                    let mut temp_board = self.clone();
                                    temp_board.set_piece(e8, None).unwrap();
                                    temp_board.set_piece(c8, Some(Piece::King(*color))).unwrap();
                                    if !temp_board.is_in_check(color) {
                                        // Le grand roque est légal, ajouter le mouvement
                                        let new_square = Square::new(Some(Piece::King(*color)), c8);
                                        moves.push(new_square);
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Short castle (King-side) for white
            if self.short_castle.0 && color == &Color::White && !self.is_in_check(color) {
                // Vérifier que les cases entre le roi et la tour sont vides
                let e1 = Coordinate::new(4, 0); // Position du roi blanc
                let f1 = Coordinate::new(5, 0);
                let g1 = Coordinate::new(6, 0);
                let h1 = Coordinate::new(7, 0); // Position de la tour pour petit roque

                // Vérifier que les cases entre le roi et la tour sont vides
                if self.get_square(f1).unwrap().available()
                    && self.get_square(g1).unwrap().available()
                {
                    // Vérifier que la pièce en h1 est bien une tour blanche
                    if let Some(square) = self.get_square(h1) {
                        if let Some(Piece::Rook(piece_color)) = square.get_piece() {
                            if piece_color == color {
                                // Vérifier que le roi ne passe pas par une case en échec
                                // Simulation pour f1
                                let mut temp_board = self.clone();
                                temp_board.set_piece(e1, None).unwrap();
                                temp_board.set_piece(f1, Some(Piece::King(*color))).unwrap();
                                if !temp_board.is_in_check(color) {
                                    // Simulation pour g1 (position finale du roi)
                                    let mut temp_board = self.clone();
                                    temp_board.set_piece(e1, None).unwrap();
                                    temp_board.set_piece(g1, Some(Piece::King(*color))).unwrap();
                                    if !temp_board.is_in_check(color) {
                                        // Le petit roque est légal, ajouter le mouvement
                                        let new_square = Square::new(Some(Piece::King(*color)), g1);
                                        moves.push(new_square);
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Short castle (King-side) for black
            if self.short_castle.1 && color == &Color::Black && !self.is_in_check(color) {
                // Vérifier que les cases entre le roi et la tour sont vides
                let e8 = Coordinate::new(4, 7); // Position du roi noir
                let f8 = Coordinate::new(5, 7);
                let g8 = Coordinate::new(6, 7);
                let h8 = Coordinate::new(7, 7); // Position de la tour pour petit roque

                // Vérifier que les cases entre le roi et la tour sont vides
                if self.get_square(f8).unwrap().available()
                    && self.get_square(g8).unwrap().available()
                {
                    // Vérifier que la pièce en h8 est bien une tour noire
                    if let Some(square) = self.get_square(h8) {
                        if let Some(Piece::Rook(piece_color)) = square.get_piece() {
                            if piece_color == color {
                                // Vérifier que le roi ne passe pas par une case en échec
                                // Simulation pour f8
                                let mut temp_board = self.clone();
                                temp_board.set_piece(e8, None).unwrap();
                                temp_board.set_piece(f8, Some(Piece::King(*color))).unwrap();
                                if !temp_board.is_in_check(color) {
                                    // Simulation pour g8 (position finale du roi)
                                    let mut temp_board = self.clone();
                                    temp_board.set_piece(e8, None).unwrap();
                                    temp_board.set_piece(g8, Some(Piece::King(*color))).unwrap();
                                    if !temp_board.is_in_check(color) {
                                        // Le petit roque est légal, ajouter le mouvement
                                        let new_square = Square::new(Some(Piece::King(*color)), g8);
                                        moves.push(new_square);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Filtrer les mouvements qui mettent le roi en échec
        if square.get_piece().is_some() {
            let from_coord = square.coordinate;

            moves.retain(|move_square| self.is_move_safe(from_coord, move_square.coordinate));
        }
        moves
    }

    fn is_in_check(&self, color: &Color) -> bool {
        // Trouver la position du roi de la couleur donnée
        let mut king_coordinate = None;
        for square in &self.squares {
            if let Some(Piece::King(king_color)) = square.get_piece() {
                if king_color == color {
                    king_coordinate = Some(square.coordinate);
                    break;
                }
            }
        }

        if let Some(king_coord) = king_coordinate {
            if let Some(king_square) = self.get_square(king_coord) {
                let king_col = king_square.get_column().unwrap();
                let king_line = king_square.get_line().unwrap();

                // Vérifier les attaques de cavalier
                let knight_moves: [(i8, i8); 8] = [
                    (1, 2),
                    (2, 1),
                    (2, -1),
                    (1, -2),
                    (-1, -2),
                    (-2, -1),
                    (-2, 1),
                    (-1, 2),
                ];

                for &(dx, dy) in &knight_moves {
                    let new_line = king_line as i8 + dx;
                    let new_col = king_col as i8 + dy;

                    if (0..8).contains(&new_line) && (0..8).contains(&new_col) {
                        if let Some(square) =
                            self.get_square(Coordinate::new(new_col as u8, new_line as u8))
                        {
                            if let Some(Piece::Knight(piece_color)) = square.get_piece() {
                                if piece_color != color {
                                    return true; // En échec par un cavalier
                                }
                            }
                        }
                    }
                }

                // Vérifier les attaques en diagonal (fou et dame)
                let diagonal_dirs: [(i8, i8); 4] = [(1, 1), (1, -1), (-1, -1), (-1, 1)];
                for &(dx, dy) in &diagonal_dirs {
                    let mut curr_line = king_line as i8;
                    let mut curr_col = king_col as i8;

                    for _ in 0..7 {
                        // Maximum 7 cases dans une direction
                        curr_line += dx;
                        curr_col += dy;

                        if !(0..8).contains(&curr_line) || !(0..8).contains(&curr_col) {
                            break; // Hors de l'échiquier
                        }

                        if let Some(square) =
                            self.get_square(Coordinate::new(curr_col as u8, curr_line as u8))
                        {
                            if let Some(piece) = square.get_piece() {
                                if piece.color() != *color {
                                    // Vérifie si c'est un fou ou une dame (qui peuvent attaquer en diagonal)
                                    match piece {
                                        Piece::Bishop(_) | Piece::Queen(_) => return true,
                                        _ => break, // Autre pièce, bloque la ligne de vue
                                    }
                                } else {
                                    break; // Pièce amie, bloque la ligne de vue
                                }
                            }
                            // Sinon case vide, continue à chercher
                        }
                    }
                }

                // Vérifier les attaques en ligne droite (tour et dame)
                let straight_dirs: [(i8, i8); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
                for &(dx, dy) in &straight_dirs {
                    let mut curr_line = king_line as i8;
                    let mut curr_col = king_col as i8;

                    for _ in 0..7 {
                        // Maximum 7 cases dans une direction
                        curr_line += dx;
                        curr_col += dy;

                        if !(0..8).contains(&curr_line) || !(0..8).contains(&curr_col) {
                            break; // Hors de l'échiquier
                        }

                        if let Some(square) =
                            self.get_square(Coordinate::new(curr_col as u8, curr_line as u8))
                        {
                            if let Some(piece) = square.get_piece() {
                                if piece.color() != *color {
                                    // Vérifie si c'est une tour ou une dame (qui peuvent attaquer en ligne droite)
                                    match piece {
                                        Piece::Rook(_) | Piece::Queen(_) => return true,
                                        _ => break, // Autre pièce, bloque la ligne de vue
                                    }
                                } else {
                                    break; // Pièce amie, bloque la ligne de vue
                                }
                            }
                            // Sinon case vide, continue à chercher
                        }
                    }
                }

                // Vérifier les attaques de pions
                let pawn_dirs = if *color == Color::White {
                    [(1, 1), (-1, 1)] // Directions d'attaque des pions noirs contre le roi blanc
                } else {
                    [(1, -1), (-1, -1)] // Directions d'attaque des pions blancs contre le roi noir
                };

                for &(dx, dy) in &pawn_dirs {
                    let new_line = king_line as i8 + dy;
                    let new_col = king_col as i8 + dx;

                    if (0..8).contains(&new_line) && (0..8).contains(&new_col) {
                        if let Some(square) =
                            self.get_square(Coordinate::new(new_col as u8, new_line as u8))
                        {
                            if let Some(Piece::Pawn(piece_color)) = square.get_piece() {
                                if piece_color != color {
                                    return true; // En échec par un pion
                                }
                            }
                        }
                    }
                }

                // Vérifier les attaques du roi adverse (pour éviter que les rois soient adjacents)
                let king_moves: [(i8, i8); 8] = [
                    (1, 1),
                    (0, 1),
                    (1, 0),
                    (-1, -1),
                    (-1, 1),
                    (1, -1),
                    (0, -1),
                    (-1, 0),
                ];

                for &(dx, dy) in &king_moves {
                    let new_line = king_line as i8 + dy;
                    let new_col = king_col as i8 + dx;

                    if (0..8).contains(&new_line) && (0..8).contains(&new_col) {
                        if let Some(square) =
                            self.get_square(Coordinate::new(new_col as u8, new_line as u8))
                        {
                            if let Some(Piece::King(piece_color)) = square.get_piece() {
                                if piece_color != color {
                                    return true; // En échec par le roi adverse
                                }
                            }
                        }
                    }
                }
            }
        }

        false // Le roi n'est pas en échec
    }
}
