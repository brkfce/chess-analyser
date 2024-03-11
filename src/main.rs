use shakmaty::{self, Position};

mod evaluator;

fn main() {
    // create initial position
    let pos = shakmaty::Chess::default();
    let board = pos.board();

    let score = evaluator::evaluate(board);

    print!("{}", score);
}
