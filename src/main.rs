pub mod chess;
pub mod piece;
use crate::chess::ChessBoard;
use crate::chess::Coordinate;

use chess::Board;
fn main() {
    let board = Board::new();
    board.print_board();
    let piece = board.get_square(Coordinate::D(6));
    let vec = board.legal_moves(*piece.unwrap());
    println!("Possible moves for {:?} : {:?}", piece, vec);
}
