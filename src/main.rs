pub mod chess;
pub mod piece;
use crate::chess::ChessBoard;
use crate::chess::Color;
use crate::chess::Coordinate;
use crate::chess::Piece;
use crate::chess::Square;
use crate::piece::king::King;
use crate::piece::knight::Knight;
use crate::piece::rook::Rook;
use crate::piece::pawn::Pawn;

use chess::Board;   
fn main() {
    let mut board = Board::empty();
    let _ = board.set_piece(Coordinate::E(0), Some(Piece::King(Color::White)));
    let _ =board.set_piece(Coordinate::E(3), Some(Piece::Knight(Color::White)));
    let _ =board.set_piece(Coordinate::D(0), Some(Piece::Rook(Color::White)));
    let _ =board.set_piece(Coordinate::E(7), Some(Piece::King(Color::Black)));
    let _ = board.set_piece(Coordinate::H(7), Some(Piece::Rook(Color::Black)));
    let _ = board.set_piece(Coordinate::D(6), Some(Piece::Pawn(Color::Black)));
    let _ = board.set_piece(Coordinate::C(5), Some(Piece::Bishop(Color::White)));
    let _ = board.set_piece(Coordinate::C(2), Some(Piece::Bishop(Color::Black)));
    println!("{}", board);  // Utilise l'implémentation du trait Display

    let pawn_moves = board.pawn_move(&Square::new(Some(Piece::Pawn(Color::Black)), Coordinate::D(6)));
    let rook_moves = board.rook_move(&Square::new(Some(Piece::Rook(Color::White)), Coordinate::D(0)));
    let rook_moves2 = board.knight_move(&Square::new(Some(Piece::Knight(Color::White)), Coordinate::E(3)));
/* 
    //let white_king = board.king_move(&Square::new(Some(Piece::King(Color::White)),Coordinate::E(0)));

    //let black_king = board.king_move(&Square::new(Some(Piece::King(Color::Black)),Coordinate::E(7)));

    for square in white_king {
        println!("White King can move to: {:?}", square);
    }
    
    for square in black_king {
        println!("Black King can move to: {:?}", square);
    }
*/
    for square in pawn_moves {
        println!("Black Rook can move to: {:?}", square);
    }
    for square in rook_moves {
        println!("White Rook can move to: {:?}", square);
    }
    for square in rook_moves2 {
        println!("White knight can move to: {:?}", square);
    }
    println!("White K : {}", board.is_in_check(&Color::White));
    println!("Black K : {}", board.is_in_check(&Color::Black));

    // Créer un board à partir d'une notation FEN
    let fen_string = "rnbqkbnr/pppppppp/8/8/2B1P3/5N2/PPPP1PPP/RNBQK2R";
    let board = Board::from_fen(fen_string).unwrap();
    println!("{}", board);  // Utilise l'implémentation du trait Display
    // Obtenir la notation FEN d'un board existant
    let fen_string = board.to_fen();
    println!("FEN: {}", fen_string);    

}
