use shakmaty::{self, Position};

mod evaluator;

fn main() {
    // create initial position
    let pos = shakmaty::Chess::default();
    // let board = pos.board();

    // let score = evaluator::evaluate(board);
    let best_move = evaluator::find_best_move(pos).unwrap();

    print!("Best move: {}", best_move.to_string());

    // print!("{}", score);
}
