use shakmaty::{Color, Role};

pub fn piece_value(board: &shakmaty::Board) -> Option<f32> {
    // evaluates based on value of pieces

    //  piece value scaling factor
    // i.e. weighting for contribution to board score
    let piece_value_scaling = 0.1;

    // this could probably be done more efficiently

    // score from white pawns
    let w_p = board.by_color(Color::White) & board.by_role(Role::Pawn);
    let w_p_s = count_set_bits(w_p.0)?;

    // score from white knights
    let w_n = board.by_color(Color::White) & board.by_role(Role::Knight);
    let w_n_s = count_set_bits(w_n.0)?;

    // score from white bishops
    let w_b = board.by_color(Color::White) & board.by_role(Role::Bishop);
    let w_b_s = count_set_bits(w_b.0)?;

    // score from white rooks
    let w_r = board.by_color(Color::White) & board.by_role(Role::Rook);
    let w_r_s = count_set_bits(w_r.0)?;

    // score from white queens
    let w_q = board.by_color(Color::White) & board.by_role(Role::Queen);
    let w_q_s = count_set_bits(w_q.0)?;

    // score from black pawns
    let b_p = board.by_color(Color::Black) & board.by_role(Role::Pawn);
    let b_p_s = count_set_bits(b_p.0)?;

    // score from black knights
    let b_n = board.by_color(Color::Black) & board.by_role(Role::Knight);
    let b_n_s = count_set_bits(b_n.0)?;

    // score from black bishops
    let b_b = board.by_color(Color::Black) & board.by_role(Role::Bishop);
    let b_b_s = count_set_bits(b_b.0)?;

    // score from black rooks
    let b_r = board.by_color(Color::Black) & board.by_role(Role::Rook);
    let b_r_s = count_set_bits(b_r.0)?;

    // score from black queens
    let b_q = board.by_color(Color::Black) & board.by_role(Role::Queen);
    let b_q_s = count_set_bits(b_q.0)?;

    // sum total score
    let piece_value = (w_p_s - b_p_s) * 1
        + (w_n_s - b_n_s) * 3
        + (w_b_s - b_b_s) * 3
        + (w_r_s - b_r_s) * 5
        + (w_q_s - b_q_s) * 9;
    let final_score = Some(piece_value_scaling * piece_value as f32);

    final_score
}

fn count_set_bits(bits: u64) -> Option<i8> {
    // counts the number of set bits in a u64

    let mut count: u64 = 0;
    let mut bits = bits.clone();

    while bits != 0 {
        count += bits & 1;
        bits >>= 1;
    }

    i8::try_from(count).ok()
}
