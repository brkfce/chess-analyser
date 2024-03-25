use shakmaty::{self, fen, CastlingMode};

mod evaluator;
mod move_gen;

fn main() {
    // create initial position
    let pos = shakmaty::Chess::default();

    // find score of initial position
    let score = evaluator::evaluate(&pos);
    println!("Score, initial: {}", score);

    // find best move from initial position
    let best_move = move_gen::depth_1_best_move(pos.clone()).unwrap();
    println!("Best move, initial: {}", best_move);

    // create non-initial position
    let fen: fen::Fen = "rnbqkb1r/pp2pppp/2p2n2/2Pp2B1/3P4/8/PP2PPPP/RN1QKBNR b KQkq - 1 4"
        .parse()
        .unwrap();
    let pos_2: shakmaty::Chess = fen.into_position(CastlingMode::Standard).unwrap();

    // find score of non-initial position
    let score_2 = evaluator::evaluate(&pos_2);
    println!("Score, non-initial: {}", score_2);

    // find best move from non-initial position
    let best_move_2 = move_gen::depth_1_best_move(pos_2).unwrap();
    println!("Best move, non-initial: {}", best_move_2);

    // use minimax algo to find best move from initial position
    let depth = 5;
    let best_move_depth = move_gen::use_minimax(pos.clone(), true, depth);
    match best_move_depth {
        Some(b_m) => println!("Best move, initial, depth {}: {}", depth, b_m),
        None => println!("No move available."),
    }
}
