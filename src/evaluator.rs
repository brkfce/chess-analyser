use shakmaty::{Color, Position, Role};

pub fn evaluate(board: &shakmaty::Board) -> f32 {
    let piece_value_mod = 0.1;
    let piece_value = piece_value(board) * piece_value_mod;

    let centre_pawn_mod = 0.1;
    let centre_pawn = centre_pawn(board) * centre_pawn_mod;

    piece_value + centre_pawn
}

pub fn find_best_move(position: shakmaty::Chess) -> shakmaty::Move {
    // finds the best move for the current position

    let depth = 1;

    let best_move: shakmaty::Move;

    let moves_list = position.legal_moves();
    let mut moves_score: Vec<f32> = Vec::new();

    for x in moves_list {
        let temp_pos = position.clone();
        let new_pos = temp_pos.play(&x).unwrap();
        moves_score.push(evaluate(&new_pos.board()));
    }

    // replace with best move
    best_move = moves_list.first().unwrap().clone()
}

fn piece_value(board: &shakmaty::Board) -> f32 {
    // evaluates based on value of pieces

    // this could probably be done more efficiently

    // score from white pawns
    let w_p = board.by_color(Color::White) & board.by_role(Role::Pawn);
    let w_p_s = count_set_bits(w_p.0);

    // score from white knights
    let w_n = board.by_color(Color::White) & board.by_role(Role::Knight);
    let w_n_s = count_set_bits(w_n.0);

    // score from white bishops
    let w_b = board.by_color(Color::White) & board.by_role(Role::Bishop);
    let w_b_s = count_set_bits(w_b.0);

    // score from white rooks
    let w_r = board.by_color(Color::White) & board.by_role(Role::Rook);
    let w_r_s = count_set_bits(w_r.0);

    // score from white queens
    let w_q = board.by_color(Color::White) & board.by_role(Role::Queen);
    let w_q_s = count_set_bits(w_q.0);

    // score from black pawns
    let b_p = board.by_color(Color::Black) & board.by_role(Role::Pawn);
    let b_p_s = count_set_bits(b_p.0);

    // score from black knights
    let b_n = board.by_color(Color::Black) & board.by_role(Role::Knight);
    let b_n_s = count_set_bits(b_n.0);

    // score from black bishops
    let b_b = board.by_color(Color::Black) & board.by_role(Role::Bishop);
    let b_b_s = count_set_bits(b_b.0);

    // score from black rooks
    let b_r = board.by_color(Color::Black) & board.by_role(Role::Rook);
    let b_r_s = count_set_bits(b_r.0);

    // score from black queens
    let b_q = board.by_color(Color::Black) & board.by_role(Role::Queen);
    let b_q_s = count_set_bits(b_q.0);

    // sum total score
    let piece_value = (w_p_s - b_p_s) * 1
        + (w_n_s - b_n_s) * 3
        + (w_b_s - b_b_s) * 3
        + (w_r_s - b_r_s) * 5
        + (w_q_s - b_q_s) * 9;

    piece_value as f32
}

fn count_set_bits(bits: u64) -> i8 {
    // counts the number of set bits in a u64

    let mut count: u64 = 0;
    let mut bits = bits.clone();

    while bits != 0 {
        count += bits & 1;
        bits >>= 1;
    }

    i8::try_from(count).ok().unwrap()
}

fn centre_pawn(board: &shakmaty::Board) -> f32 {
    // evaluates the control of the centre with pawns

    // score from white pawns
    let w_p = board.by_color(Color::White) & board.by_role(Role::Pawn) & shakmaty::Bitboard::CENTER;
    let w_p_s = count_set_bits(w_p.0);

    // score from black pawns
    let b_p = board.by_color(Color::Black) & board.by_role(Role::Pawn) & shakmaty::Bitboard::CENTER;
    let b_p_s = count_set_bits(b_p.0);

    (w_p_s - b_p_s) as f32
}
