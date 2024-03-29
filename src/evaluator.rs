use shakmaty::{Color, Position, Role};

pub fn evaluate(position: &shakmaty::Chess) -> f32 {
    let board = position.board();

    let piece_value_mod = 0.1;
    let piece_value = piece_value(board) * piece_value_mod;

    let centre_pawn_mod = 0.1;
    let centre_pawn = centre_pawn(board) * centre_pawn_mod;

    let centre_knight_mod = 0.1;
    let centre_knight = centre_knight(board) * centre_knight_mod;

    let castling_mod = 0.1;
    let castling = castling(position) * castling_mod;

    piece_value + centre_pawn + centre_knight + castling
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
    let piece_value = (w_p_s - b_p_s)
        + (w_n_s - b_n_s) * 3
        + (w_b_s - b_b_s) * 3
        + (w_r_s - b_r_s) * 5
        + (w_q_s - b_q_s) * 9;

    piece_value as f32
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

fn centre_knight(board: &shakmaty::Board) -> f32 {
    // evaluates the centre positioning of the knights

    // bitboard representing all squares except the edge squares
    let centre_squares = shakmaty::Bitboard(0x007e_7e7e_7e7e_7e00);

    // score from white knights
    let w_n = board.by_color(Color::White) & board.by_role(Role::Knight) & centre_squares;
    let w_n_s = count_set_bits(w_n.0);

    // score from black knights
    let b_n = board.by_color(Color::Black) & board.by_role(Role::Knight) & centre_squares;
    let b_n_s = count_set_bits(b_n.0);

    (w_n_s - b_n_s) as f32
}

fn castling(position: &shakmaty::Chess) -> f32 {
    // evaluates whether each side has castled
    // simple test initially, later would like to identify whether
    // the castling is on the same side or opposite side to the opponent "attack"

    // score from white castling
    let w_c_s = position.castles().has_color(Color::White);

    // score from black castling
    let b_c_s = position.castles().has_color(Color::Black);

    (w_c_s as i8 - b_c_s as i8) as f32
}

fn count_set_bits(bits: u64) -> i8 {
    // counts the number of set bits in a u64

    let mut count: u64 = 0;
    let mut bits = bits;

    while bits != 0 {
        count += bits & 1;
        bits >>= 1;
    }

    i8::try_from(count).ok().unwrap()
}
