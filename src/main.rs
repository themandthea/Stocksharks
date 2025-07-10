pub mod chess;
pub mod piece;

use chess::Board;
fn main() {
    let board = Board::new();
    board.print_board();
}
