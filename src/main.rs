pub mod chess;
pub mod piece;
use crate::chess::ChessBoard;
use crate::chess::Color;
use crate::chess::Coordinate;
use crate::chess::Piece;
use crate::chess::Square;

use chess::Board;   
fn main() {
    let mut board = Board::new();
    let _ = board.set_piece(Coordinate::D(3), Some(Piece::Knight(Color::White)));
    let _ =board.set_piece(Coordinate::E(5), Some(Piece::Pawn(Color::Black)));
    let _ =board.set_piece(Coordinate::B(2), Some(Piece::Pawn(Color::Black)));
    board.print_board();

    let piece = board.get_square(Coordinate::D(3)).unwrap();
    let vec = board.legal_moves(piece).unwrap();
    for square in vec.iter() {
        println!("{}", square);
    }
}
