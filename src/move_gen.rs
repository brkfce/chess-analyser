use shakmaty::{Color, Position, Role};

pub fn find_best_move(position: shakmaty::Chess) -> Option<shakmaty::Move> {
    // finds the best move for the current position

    // for the time being, only going to search once (no recursion)

    let _depth = 1;

    let moves_list = position.legal_moves();
    if moves_list.len() == 0 {
        return None;
    }

    let mut moves_score: Vec<f32> = Vec::new();

    for x in moves_list.clone() {
        let temp_pos = position.clone();
        let new_pos = temp_pos.play(&x).unwrap();
        moves_score.push(crate::evaluator::evaluate(&new_pos.board()));
    }

    // return move with best evaluator score
    let best_index = highest_score_index(moves_score)? as usize;
    let best_move = moves_list.get(best_index).unwrap().clone();

    Some(best_move)
}
fn highest_score_index(scores: Vec<f32>) -> Option<i8> {
    // returns the index of the vec that contains the highest score

    let mut temp_max: f32;
    let mut index: i8 = 0;
    let mut top_index: i8 = 0;

    if scores.len() == 0 {
        return None;
    } else {
        temp_max = scores[0];
        for x in scores {
            index = index + 1;
            if x > temp_max {
                temp_max = x;
                top_index = index;
            }
        }
    }
    Some(top_index)
}
