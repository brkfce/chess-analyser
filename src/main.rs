use shakmaty::{self, Position};

mod evaluator;
mod move_gen;

fn main() {
    // create initial position
    let pos = shakmaty::Chess::default();
    let board = pos.board();

    let score = evaluator::evaluate(board);
    let best_move = move_gen::depth_1_best_move(pos).unwrap();

    println!("Best move: {}", best_move);

    print!("{}", score);
}
