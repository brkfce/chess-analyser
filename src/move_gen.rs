use crate::evaluator::evaluate;
use shakmaty::{Move, Position};
use std::f32::INFINITY;

pub fn depth_1_best_move(position: shakmaty::Chess) -> Option<shakmaty::Move> {
    // finds the best move for the current position

    // for the time being, only going to search once (no recursion)

    let moves_list = position.legal_moves();
    if moves_list.is_empty() {
        return None;
    }

    let mut moves_score: Vec<f32> = Vec::new();

    for x in moves_list.clone() {
        let temp_pos = position.clone();
        let new_pos = temp_pos.play(&x).unwrap();
        moves_score.push(crate::evaluator::evaluate(&new_pos));
    }

    // return move with best evaluator score
    let best_index = highest_score_index(moves_score)? as usize;
    let best_move = moves_list.get(best_index).unwrap().clone();

    Some(best_move)
}

// constructing a tree of positions
pub struct PositionNode {
    position: shakmaty::Chess,
    score: f32,
    children: Vec<PositionNode>,
    source_move: Option<shakmaty::Move>,
}

impl PositionNode {
    pub fn new(position: shakmaty::Chess, score: f32, source_move: Option<Move>) -> Self {
        PositionNode {
            position,
            score,
            children: Vec::new(),
            source_move,
        }
    }
    fn new_child(&mut self, child: PositionNode) {
        self.children.push(child);
    }
}

fn minimax(
    node: &mut PositionNode,
    maximising: bool,
    depth: i8,
    alpha: &mut f32,
    beta: &mut f32,
) -> f32 {
    // implementation of the minimax algorithm
    // end if max depth is reached, or the game is over
    if depth == 0 {
        return evaluate(&node.position);
    } else {
        // generate children positions and nodes
        let child_moves = node.position.legal_moves();
        for x in child_moves {
            let new_position = node.position.clone().play(&x);
            match new_position {
                Ok(legal_move) => node.new_child(PositionNode::new(
                    legal_move.clone(),
                    evaluate(&legal_move),
                    Some(x.clone()),
                )),
                Err(_) => return node.score,
            }
        }
        // find best value for player 1/player we are hoping to "win"
        if maximising {
            let mut value = -INFINITY;
            // recurse for each sub position
            for child in &mut node.children {
                value = f32::max(value, minimax(child, false, depth - 1, alpha, beta));
                *alpha = f32::max(*alpha, value);
                if value >= *beta {
                    break;
                }
            }
            value
        // find best value for player 2/player we are hoping to "beat"
        } else {
            let mut value = INFINITY;
            // recurse for each child position
            for child in &mut node.children {
                value = f32::min(value, minimax(child, true, depth - 1, alpha, beta));
                *beta = f32::min(*beta, value);
                if value <= *alpha {
                    break;
                }
            }
            value
        }
    }
}

pub fn use_minimax(
    initial_position: shakmaty::Chess,
    maximising: bool,
    depth: i8,
) -> Option<shakmaty::Move> {
    // use the minimax algo to find the best move
    let mut initial_node = PositionNode::new(
        initial_position.clone(),
        evaluate(&initial_position.clone()),
        None,
    );
    let mut alpha = -INFINITY;
    let mut beta = INFINITY;
    let best_score = minimax(&mut initial_node, maximising, depth, &mut alpha, &mut beta);
    if initial_node.children.is_empty() {
        None
    } else {
        let best_child = initial_node
            .children
            .iter()
            .find(|child| child.score == best_score)
            .unwrap();
        best_child.source_move.clone()
    }
}
fn highest_score_index(scores: Vec<f32>) -> Option<i8> {
    // returns the index of the vec that contains the highest score

    let mut temp_max: f32;
    let mut index: i8 = 0;
    let mut top_index: i8 = 0;

    if scores.is_empty() {
        return None;
    } else {
        temp_max = scores[0];
        for x in scores {
            index += 1;
            if x > temp_max {
                temp_max = x;
                top_index = index;
            }
        }
    }
    Some(top_index)
}
