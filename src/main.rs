use shakmaty::{self, Position};

mod evaluator;

fn main() {
    // create initial position
    let pos = shakmaty::Chess::default();
    let board = pos.board();

    let score = evaluator::piece_value(board).unwrap();

    print!("{}", score);
}
