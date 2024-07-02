#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::HashMap;

type Sq = i8;
type AlgSq = [String; 2];
type Piece = i8;
type Direction = i8;
type File = i8;
type Rank = i8;
type Color = i8;
type Ply = u32;
type Val = f64;

const COLOR_WHITE: Color = 1;
const COLOR_BLACK: Color = -1;
const COLOR_EMPTY: Color = 0;

const P_BASE: Piece = 1;
const R_BASE: Piece = 2;
const N_BASE: Piece = 3;
const B_BASE: Piece = 4;
const Q_BASE: Piece = 5;
const K_BASE: Piece = 6;

const EMPTY: Piece = 0;
const P_WHITE: Piece = P_BASE;
const R_WHITE: Piece = R_BASE;
const N_WHITE: Piece = N_BASE;
const B_WHITE: Piece = B_BASE;
const Q_WHITE: Piece = Q_BASE;
const K_WHITE: Piece = K_BASE;
const P_BLACK: Piece = -P_BASE;
const R_BLACK: Piece = -R_BASE;
const N_BLACK: Piece = -N_BASE;
const B_BLACK: Piece = -B_BASE;
const Q_BLACK: Piece = -Q_BASE;
const K_BLACK: Piece = -K_BASE;

const DIR_U: Direction = 0;
const DIR_R: Direction = 1;
const DIR_D: Direction = 2;
const DIR_L: Direction = 3;
const DIR_UR: Direction = 4;
const DIR_DR: Direction = 5;
const DIR_DL: Direction = 6;
const DIR_UL: Direction = 7;
const DIR_NUR: Direction = 8;
const DIR_NRU: Direction = 9;
const DIR_NRD: Direction = 10;
const DIR_NDR: Direction = 11;
const DIR_NDL: Direction = 12;
const DIR_NLD: Direction = 13;
const DIR_NLU: Direction = 14;
const DIR_NUL: Direction = 15;

const ALL_DIRECTIONS: [Direction; 16] = [
    DIR_U, DIR_R, DIR_D, DIR_L,
    DIR_UR, DIR_DR, DIR_DL, DIR_UL,
    DIR_NUR, DIR_NRU, DIR_NRD, DIR_NDR,
    DIR_NDL, DIR_NLD, DIR_NLU, DIR_NUL,
];

const KNIGHT_DIRECTIONS: [Direction; 8] = [
    DIR_NUR, DIR_NRU, DIR_NRD, DIR_NDR,
    DIR_NDL, DIR_NLD, DIR_NLU, DIR_NUL,
];

const ROOK_DIRECTIONS: [Direction; 4] = [
    DIR_U, DIR_R, DIR_D, DIR_L,
];

const BISHOP_DIRECTIONS: [Direction; 4] = [
    DIR_UR, DIR_DR, DIR_DL, DIR_UL,
];

const QUEEN_DIRECTIONS: [Direction; 8] = [
    DIR_U, DIR_R, DIR_D, DIR_L,
    DIR_UR, DIR_DR, DIR_DL, DIR_UL,
];

const KING_DIRECTIONS: [Direction; 8] = [
    DIR_U, DIR_R, DIR_D, DIR_L,
    DIR_UR, DIR_DR, DIR_DL, DIR_UL,
];

fn char_to_piece(c: char) -> Direction {
    match c {
        'R' => R_WHITE,
        'N' => N_WHITE,
        'B' => B_WHITE,
        'Q' => Q_WHITE,
        'K' => K_WHITE,
        'P' => P_WHITE,
        'r' => R_BLACK,
        'n' => N_BLACK,
        'b' => B_BLACK,
        'q' => Q_BLACK,
        'k' => K_BLACK,
        'p' => P_BLACK,
        _ => panic!("Unexpected!"),
    }
}

fn piece_to_char(piece: Piece) -> char {
    match piece {
        R_WHITE => 'R',
        N_WHITE => 'N',
        B_WHITE => 'B',
        Q_WHITE => 'Q',
        K_WHITE => 'K',
        P_WHITE => 'P',
        R_BLACK => 'r',
        N_BLACK => 'n',
        B_BLACK => 'b',
        Q_BLACK => 'q',
        K_BLACK => 'k',
        P_BLACK => 'p',
        _ => panic!("Unexpected!"),
    }
}

struct MoveVal {
    mov: Option<Move>,
    val: Val,
    checkmate: bool,
    leads_to: Option<Position>,
}

struct Position {
    placement: [Piece; 64],
    active_color: Color,
    castling: [bool; 4],
    en_passant: Option<Sq>,
    halfmoves: i32,
    fullmoves: i32,
    evaluation: Option<HashMap<Ply, Vec<MoveVal>>>,
    moves: Option<Vec<Move>>,
    is_king_in_check: Option<bool>,
    is_king_in_checkmate: Option<bool>,
    is_king_in_stalemate: Option<bool>,
}

fn empty_position() -> Position {
    Position{
        placement: [EMPTY; 64],
        active_color: COLOR_WHITE,
        castling: [false; 4],
        en_passant: None,
        halfmoves: 0,
        fullmoves: 0,
        evaluation: None,
        moves: None,
        is_king_in_check: None,
        is_king_in_checkmate: None,
        is_king_in_stalemate: None,
    }
}

struct FileRank {
    f: File,
    r: Rank,
}

struct Move {
    piece: Piece,
    from: Sq,
    to: Sq,
    leads_to: Option<Position>,
}

fn sq_to_filerank(sq: Sq) -> FileRank {
    FileRank{
        f: sq % 8,
        r: sq / 8,
    }
}

fn filerank_to_sq(filerank: &FileRank) -> Sq {
    8 * filerank.r + filerank.f
}

fn fr_to_sq(f: File, r: Rank) -> Sq {
    8 * r + f
}

fn piece_at_sq(pos: &Position, sq: Sq) -> Piece {
    pos.placement[sq as usize]
}

fn set_piece_at_sq(pos: &mut Position, sq: Sq, piece: Piece) {
    pos.placement[sq as usize] = piece;
}

fn algsq_to_sq(algsq: AlgSq) -> Sq {
    filerank_to_sq(&(FileRank{
        f: match algsq[0].as_str() {
            "a" => 0,
            "b" => 1,
            "c" => 2,
            "d" => 3,
            "e" => 4,
            "f" => 5,
            "g" => 6,
            "h" => 7,
            _ => panic!("Unexpected!"),
        },
        r: match algsq[1].as_str() {
            "1" => 0,
            "2" => 1,
            "3" => 2,
            "4" => 3,
            "5" => 4,
            "6" => 5,
            "7" => 6,
            "8" => 7,
            _ => panic!("Unexpected!"),
        },
    }))
}

fn f_to_string(f: File) -> String {
    String::from(
        match f {
            0 => "a",
            1 => "b",
            2 => "c",
            3 => "d",
            4 => "e",
            5 => "f",
            6 => "g",
            7 => "h",
            _ => panic!("Unexpected!"),
        }
    )
}

fn r_to_string(r: Rank) -> String {
    String::from(
        match r {
            0 => "1",
            1 => "2",
            2 => "3",
            3 => "4",
            4 => "5",
            5 => "6",
            6 => "7",
            7 => "8",
            _ => panic!("Unexpected!"),
        }
    )
}

fn sq_to_algsq(sq: Sq) -> AlgSq {
    let filerank = sq_to_filerank(sq);
    [f_to_string(filerank.f), r_to_string(filerank.r)]
}

fn sq_to_algstring(sq: Sq) -> String {
    sq_to_algsq(sq).join("")
}

fn is_there_piece_at_sq(
    pos: &Position, sq: Sq
) -> bool {
    piece_at_sq(pos, sq) != EMPTY
}

fn is_move_capture(pos: &Position, mov: &Move) -> bool {
    is_there_piece_at_sq(pos, mov.to)
}

fn is_piece_white(piece: Piece) -> bool {
    piece > EMPTY
}

fn is_piece_black(piece: Piece) -> bool {
    piece < EMPTY
}

fn piece_color(piece: Piece) -> Color {
    if is_piece_white(piece) {
        COLOR_WHITE
    } else if is_piece_black(piece) {
        COLOR_BLACK
    } else {
        COLOR_EMPTY
    }
}

fn piece_base(piece: Piece) -> Piece {
    piece.abs()
}

fn toggled_color(color: Color) -> Color {
    match color {
        COLOR_WHITE => COLOR_BLACK,
        COLOR_BLACK => COLOR_WHITE,
        _ => panic!("Unexpected!"),
    }
}

fn decode_fen(fen_string: String) -> Position {
    let mut p = empty_position();
    let mut state = 0;
    let mut f = 0;
    let mut r = 7;
    let mut en_passant = [String::from(""), String::from("")];
    let mut en_passant_index = 0;
    let mut halfmoves = String::from("");
    let mut fullmoves = String::from("");
    for c in fen_string.chars() {
        if c == ' ' {
            state += 1;
        } else if state == 0 {
            match c {
                'r' => {
                    set_piece_at_sq(&mut p, fr_to_sq(f, r), R_BLACK);
                    f += 1;
                },
                'n' => {
                    set_piece_at_sq(&mut p, fr_to_sq(f, r), N_BLACK);
                    f += 1;
                },
                'b' => {
                    set_piece_at_sq(&mut p, fr_to_sq(f, r), B_BLACK);
                    f += 1;
                },
                'q' => {
                    set_piece_at_sq(&mut p, fr_to_sq(f, r), Q_BLACK);
                    f += 1;
                },
                'k' => {
                    set_piece_at_sq(&mut p, fr_to_sq(f, r), K_BLACK);
                    f += 1;
                },
                'p' => {
                    set_piece_at_sq(&mut p, fr_to_sq(f, r), P_BLACK);
                    f += 1;
                },
                'R' => {
                    set_piece_at_sq(&mut p, fr_to_sq(f, r), R_WHITE);
                    f += 1;
                },
                'N' => {
                    set_piece_at_sq(&mut p, fr_to_sq(f, r), N_WHITE);
                    f += 1;
                },
                'B' => {
                    set_piece_at_sq(&mut p, fr_to_sq(f, r), B_WHITE);
                    f += 1;
                },
                'Q' => {
                    set_piece_at_sq(&mut p, fr_to_sq(f, r), Q_WHITE);
                    f += 1;
                },
                'K' => {
                    set_piece_at_sq(&mut p, fr_to_sq(f, r), K_WHITE);
                    f += 1;
                },
                'P' => {
                    set_piece_at_sq(&mut p, fr_to_sq(f, r), P_WHITE);
                    f += 1;
                },
                '1' => { f += 1 },
                '2' => { f += 2 },
                '3' => { f += 3 },
                '4' => { f += 4 },
                '5' => { f += 5 },
                '6' => { f += 6 },
                '7' => { f += 7 },
                '8' => { f += 8 },
                '/' => { r -= 1; f = 0 },
                _ => panic!("Unexpected!"),
            }
        } else if state == 1 {
            match c {
                'w' => { p.active_color = COLOR_WHITE },
                'b' => { p.active_color = COLOR_BLACK },
                _ => panic!("Unexpected!"),
            }
        } else if state == 2 {
            match c {
                'K' => { p.castling[0] = true },
                'Q' => { p.castling[1] = true },
                'k' => { p.castling[2] = true },
                'q' => { p.castling[3] = true },
                '-' => {},
                _ => panic!("Unexpected!"),
            }
        } else if state == 3 {
            match c {
                _ => {
                    en_passant[en_passant_index] = String::from(c);
                    en_passant_index += 1;
                },
            }
        } else if state == 4 {
            if en_passant[0] != String::from("-") {
                p.en_passant = Some(algsq_to_sq(en_passant.clone()));
            }
            halfmoves.push(c);
        } else if state == 5 {
            p.halfmoves = halfmoves.parse().unwrap();
            fullmoves.push(c);
        }
    }
    p.fullmoves = fullmoves.parse().unwrap();
    p
}

fn next_sq_in_dir(sq: Sq, dir:Direction) -> Option<Sq> {
    let filerank = sq_to_filerank(sq);
    let mut f = filerank.f;
    let mut r = filerank.r;
    let mut n_dir = false;
    match dir {
        DIR_U => { r += 1; },
        DIR_R => { f += 1 },
        DIR_D => { r -= 1 },
        DIR_L => { f -= 1 },
        DIR_UR => { r += 1; f += 1 },
        DIR_DR => { r -= 1; f += 1 },
        DIR_DL => { r -= 1; f -= 1 },
        DIR_UL => { r += 1; f -= 1 },
        DIR_NUR => { n_dir = true; r += 2; f += 1 },
        DIR_NRU => { n_dir = true; f += 2; r += 1 },
        DIR_NRD => { n_dir = true; f += 2; r -= 1 },
        DIR_NDR => { n_dir = true; r -= 2; f += 1 },
        DIR_NDL => { n_dir = true; r -= 2; f -= 1 },
        DIR_NLD => { n_dir = true; f -= 2; r -= 1 },
        DIR_NLU => { n_dir = true; f -= 2; r += 1 },
        DIR_NUL => { n_dir = true; r += 2; f -= 1 },
        _ => panic!("Unexpected!"),
    };
    if n_dir || f < 0 || f > 7 || r < 0 || r > 7 {
        None
    } else {
        Some(fr_to_sq(f, r))
    }
}

fn apply_dir_u(fr: &mut FileRank) -> bool { fr.r += 1; fr.r > 7 }
fn apply_dir_r(fr: &mut FileRank) -> bool { fr.f += 1; fr.f > 7 }
fn apply_dir_d(fr: &mut FileRank) -> bool { fr.r -= 1; fr.r < 0 }
fn apply_dir_l(fr: &mut FileRank) -> bool { fr.f -= 1; fr.f < 0 }
fn apply_dir_ur(fr: &mut FileRank) -> bool {
    apply_dir_u(fr) || apply_dir_r(fr)
}
fn apply_dir_dr(fr: &mut FileRank) -> bool {
    apply_dir_d(fr) || apply_dir_r(fr)
}
fn apply_dir_dl(fr: &mut FileRank) -> bool {
    apply_dir_d(fr) || apply_dir_l(fr)
}
fn apply_dir_ul(fr: &mut FileRank) -> bool {
    apply_dir_u(fr) || apply_dir_l(fr)
}
fn apply_dir_nur(fr: &mut FileRank) -> bool {
    apply_dir_u(fr) || apply_dir_u(fr) || apply_dir_r(fr)
}
fn apply_dir_nru(fr: &mut FileRank) -> bool {
    apply_dir_r(fr) || apply_dir_r(fr) || apply_dir_u(fr)
}
fn apply_dir_nrd(fr: &mut FileRank) -> bool {
    apply_dir_r(fr) || apply_dir_r(fr) || apply_dir_d(fr)
}
fn apply_dir_ndr(fr: &mut FileRank) -> bool {
    apply_dir_d(fr) || apply_dir_d(fr) || apply_dir_r(fr)
}
fn apply_dir_ndl(fr: &mut FileRank) -> bool {
    apply_dir_d(fr) || apply_dir_d(fr) || apply_dir_l(fr)
}
fn apply_dir_nld(fr: &mut FileRank) -> bool {
    apply_dir_l(fr) || apply_dir_l(fr) || apply_dir_d(fr)
}
fn apply_dir_nlu(fr: &mut FileRank) -> bool {
    apply_dir_l(fr) || apply_dir_l(fr) || apply_dir_u(fr)
}
fn apply_dir_nul(fr: &mut FileRank) -> bool {
    apply_dir_u(fr) || apply_dir_u(fr) || apply_dir_l(fr)
}

fn for_each_legal_sq_for_pawn(
    pos: &Position, sq: Sq, fr0: &FileRank,
    color: Color, mut func_for_sqs: impl FnMut(Sq),
    mut func_for_captures: impl FnMut(Sq, Piece) -> bool
) {
    let mut fr = FileRank{f: fr0.f, r: fr0.r};
    if color == COLOR_WHITE {
        // One forward.
        apply_dir_u(&mut fr);
        {
            let piece_found = piece_at_sq(pos, filerank_to_sq(&fr));
            if piece_found == EMPTY {
                func_for_sqs(filerank_to_sq(&fr));
            }
        }
        if fr0.r == 1 {
            // Two forward.
            apply_dir_u(&mut fr);
            let piece_found = piece_at_sq(pos, filerank_to_sq(&fr));
            if piece_found == EMPTY {
                func_for_sqs(filerank_to_sq(&fr));
            }
        }
        // Captures
        {
            fr = FileRank{f: fr0.f, r: fr0.r};
            apply_dir_ur(&mut fr);
            let piece_found = piece_at_sq(pos, filerank_to_sq(&fr));
            if piece_color(piece_found) == -color {
                if func_for_captures(filerank_to_sq(&fr), piece_found) {
                    return;
                }
                func_for_sqs(filerank_to_sq(&fr));
            }
        }
        {
            fr = FileRank{f: fr0.f, r: fr0.r};
            apply_dir_ul(&mut fr);
            let piece_found = piece_at_sq(pos, filerank_to_sq(&fr));
            if piece_color(piece_found) == -color {
                if func_for_captures(filerank_to_sq(&fr), piece_found) {
                    return;
                }
                func_for_sqs(filerank_to_sq(&fr));
            }
        }
    } else {
        // One forward.
        apply_dir_d(&mut fr);
        {
            let piece_found = piece_at_sq(pos, filerank_to_sq(&fr));
            if piece_found == EMPTY {
                func_for_sqs(filerank_to_sq(&fr));
            }
        }
        if fr0.r == 6 {
            // Two forward.
            apply_dir_d(&mut fr);
            let piece_found = piece_at_sq(pos, filerank_to_sq(&fr));
            if piece_found == EMPTY {
                func_for_sqs(filerank_to_sq(&fr));
            }
        }
        // Captures
        {
            fr = FileRank{f: fr0.f, r: fr0.r};
            apply_dir_dr(&mut fr);
            let piece_found = piece_at_sq(pos, filerank_to_sq(&fr));
            if piece_color(piece_found) == -color {
                if func_for_captures(filerank_to_sq(&fr), piece_found) {
                    return;
                }
                func_for_sqs(filerank_to_sq(&fr));
            }
        }
        {
            fr = FileRank{f: fr0.f, r: fr0.r};
            apply_dir_dl(&mut fr);
            let piece_found = piece_at_sq(pos, filerank_to_sq(&fr));
            if piece_color(piece_found) == -color {
                if func_for_captures(filerank_to_sq(&fr), piece_found) {
                    return;
                }
                func_for_sqs(filerank_to_sq(&fr));
            }
        }
    }
}

fn for_each_legal_sq_from_sq(
    pos: &Position, sq: Sq,
    mut func_for_sqs: impl FnMut(Sq),
    mut func_for_captures: impl FnMut(Sq, Piece) -> bool,
    piece_override: Option<Piece>,
) {
    let fr0 = sq_to_filerank(sq);
    let piece;
    match piece_override {
        Some(p) => { piece = p; },
        None => { piece = piece_at_sq(pos, sq); },
    }
    let color = piece_color(piece);
    let pb = piece_base(piece);
    if pb == P_BASE {
        for_each_legal_sq_for_pawn(
            pos, sq, &fr0, color, func_for_sqs, func_for_captures);
        return;
    }
    let apply_dir_funcs = match pb {
        R_BASE => vec![apply_dir_u, apply_dir_r, apply_dir_d, apply_dir_l],
        B_BASE => vec![apply_dir_ur, apply_dir_dr, apply_dir_dl, apply_dir_ul],
        Q_BASE => vec![
            apply_dir_u, apply_dir_r, apply_dir_d, apply_dir_l,
            apply_dir_ur, apply_dir_dr, apply_dir_dl, apply_dir_ul
        ],
        K_BASE => vec![
            apply_dir_u, apply_dir_r, apply_dir_d, apply_dir_l,
            apply_dir_ur, apply_dir_dr, apply_dir_dl, apply_dir_ul
        ],
        N_BASE => vec![
            apply_dir_nur, apply_dir_nru, apply_dir_nrd, apply_dir_ndr,
            apply_dir_ndl, apply_dir_nld, apply_dir_nlu, apply_dir_nul
        ],
        _ => panic!("Unexpected!"),
    };
    for apply_dir_func in apply_dir_funcs {
        let mut fr = FileRank{f: fr0.f, r: fr0.r};
        loop {
            if apply_dir_func(&mut fr) {
                break;
            }
            let sq = filerank_to_sq(&fr);
            let piece_found = piece_at_sq(pos, sq);
            let piece_found_color = piece_color(piece_found);
            if piece_found_color == COLOR_EMPTY {
                func_for_sqs(sq);
            } else if piece_found_color == color {
                break;
            } else {
                if func_for_captures(sq, piece_found) {
                    return;
                }
                func_for_sqs(sq);
                break;
            }
            match pb {
                N_BASE => { break; },
                K_BASE => { break; },
                _ => {},
            }
        }
    }
}

fn does_move_lead_to_own_king_in_check(pos: &Position, mov: &Move) -> bool {
    //println!("Checking move {} -> {}",
    //            sq_to_algstring(mov.from), sq_to_algstring(mov.to));
    is_king_in_check(&position_after_move(pos, mov), true)
}

fn for_each_legal_move_from_position(pos: &Position, mut func: impl FnMut(Move)) {
    for sq in 0 .. 64 {
        let piece_found = piece_at_sq(pos, sq);
        let piece_found_color = piece_color(piece_found);
        if piece_found_color == pos.active_color {
            for_each_legal_sq_from_sq(
                &pos,
                sq,
                |sq_to: Sq| {
                    func(Move{
                        piece: piece_found,
                        from: sq,
                        to: sq_to,
                        leads_to: None,
                    })
                },
                |cap_sq, cap_piece| {
                    return false;
                },
                None
            );
        }
    }
}

fn set_moves_to_position(pos: &mut Position) {
    match pos.moves {
        Some(_) => panic!("Unexpected"),
        None => {
            let mut v = Vec::new();
            for_each_legal_move_from_position(
                pos,
                |mov| {
                    //println!("{}", does_move_lead_to_own_king_in_check(pos, &mov));
                    if !does_move_lead_to_own_king_in_check(pos, &mov) {
                        v.push(mov);
                    }
                }
            );
            pos.moves = Some(v);
        }
    }
}

fn expand_position(pos: &mut Position) {
    set_is_king_in_check(pos);
    set_moves_to_position(pos);
    set_is_king_in_checkmate(pos);
    set_is_king_in_stalemate(pos);
}

fn is_king_in_checkmate(pos: &Position) -> bool {
    match &pos.moves {
        None => panic!("Unexpected"),
        Some(moves) => {
            if moves.is_empty() {
                return is_king_in_check(pos, false)
            }
            return false
        },
    }
}

fn is_king_in_stalemate(pos: &Position) -> bool {
    match &pos.moves {
        None => panic!("Unexpected"),
        Some(moves) => {
            if moves.is_empty() {
                return !is_king_in_check(pos, false)
            }
            return false
        },
    }
}

fn print_sq(sq: Sq) {
    println!("{}", sq_to_algstring(sq));
}

fn print_move(mov: Move) {
    println!("Move from {} to {}",
                sq_to_algstring(mov.from),
                sq_to_algstring(mov.to));
}

fn position_after_move(pos: &Position, mov: &Move) -> Position {
    let mut new_placement: [Piece; 64] = [0; 64];
    new_placement.copy_from_slice(&pos.placement);
    let mut pos = Position{
        placement: new_placement,
        active_color: -pos.active_color,
        castling: pos.castling,
        en_passant: pos.en_passant,
        halfmoves: pos.halfmoves, //TODO
        fullmoves: pos.fullmoves, // TODO
        evaluation: None,
        moves: None,
        is_king_in_check: None,
        is_king_in_checkmate: None,
        is_king_in_stalemate: None,
    };
    set_piece_at_sq(&mut pos, mov.from, EMPTY);
    set_piece_at_sq(&mut pos, mov.to, mov.piece);
    pos
}

fn position_static_value(pos: &Position) -> Val {
    let mut result = 0.0;
    for f in 0 .. 8 {
        for r in 0 .. 8 {
            let sq = fr_to_sq(f, r);
            let piece_found = piece_at_sq(pos, sq);
            result += match piece_found {
                R_WHITE =>  5.0,
                R_BLACK => -5.0,
                N_WHITE =>  3.0,
                N_BLACK => -3.0,
                B_WHITE =>  3.0,
                B_BLACK => -3.0,
                Q_WHITE =>  9.0,
                Q_BLACK => -9.0,
                P_WHITE =>  1.0,
                P_BLACK => -1.0,
                K_WHITE =>  9999.0,
                K_BLACK => -9999.0,
                EMPTY => 0.0,
                _ => panic!("Unexpected"),
            };
        }
    }
    result
}

fn is_king_in_check(pos: &Position, w_toggled_active_color: bool) -> bool {
    let sign;
    if w_toggled_active_color { sign = -1; }
    else { sign = 1; }

    let king;
    if pos.active_color == COLOR_WHITE { king = K_WHITE * sign; }
    else { king = K_BLACK * sign; }

    for f in 0 .. 8 {
        for r in 0 .. 8 {
            let sq = fr_to_sq(f, r);
            let piece_found = piece_at_sq(pos, sq);
            if piece_found == king {
                //println!("King found at {}", sq_to_algstring(sq));
                return is_king_in_specific_sq_in_check(
                                pos, sq, w_toggled_active_color);
            }
        }
    }
    false // For positions without king.
}

fn set_is_king_in_check(pos: &mut Position) {
    match &pos.is_king_in_check {
        None => { pos.is_king_in_check = Some(is_king_in_check(pos, false)); },
        Some(_) => panic!("Unexpected"),
    };
}

fn set_is_king_in_checkmate(pos: &mut Position) {
    match &pos.is_king_in_checkmate {
        None => { pos.is_king_in_checkmate = Some(is_king_in_checkmate(pos)); },
        Some(_) => panic!("Unexpected"),
    };
}

fn set_is_king_in_stalemate(pos: &mut Position) {
    match &pos.is_king_in_stalemate {
        None => { pos.is_king_in_stalemate = Some(is_king_in_stalemate(pos)); },
        Some(_) => panic!("Unexpected"),
    };
}

fn is_king_in_specific_sq_in_check(
        pos: &Position, sq: Sq, w_toggled_active_color: bool,
) -> bool {
    //println!("Checking if king on {} is in check...", sq_to_algstring(sq));
    let sign;
    if w_toggled_active_color { sign = -1; }
    else { sign = 1; }
    let mut result = false;
    for p in [R_WHITE, N_WHITE, B_WHITE, Q_WHITE, P_WHITE] {
        for_each_legal_sq_from_sq(
            pos, sq,
            |sq| { },
            |cap_sq, cap_piece| {
                if cap_piece == (-p * pos.active_color * sign) {
                    result = true;
                    // Return true to instruct
                    // for_each_legal_sq_from_sq to stop.
                    return true;
                }
                return false;
            },
            Some(p * pos.active_color * sign),
        );
        if result {
            return result;
        }
    }
    result
}

fn position_val_at_ply(pos: &Position, ply: Ply) -> Vec<MoveVal> {
    let mut v = Vec::<MoveVal>::new();
    if ply == 0 {
        v.push(MoveVal{
            mov: None,
            val: position_static_value(pos),
            checkmate: false,
            leads_to: None,
        });
    } else {
        for_each_legal_move_from_position(
            pos, 
            |mov| {
                let new_pos = position_after_move(pos, &mov);
                v.push(MoveVal{
                    mov: Some(mov),
                    val: 0.0,
                    checkmate: false,
                    leads_to: Some(new_pos),
                });
            }
        );
    }
    v
}

// TODO: print moves
// TODO: sort moves in evaluation

fn move_to_string(mov: &Move, pos: &Position) -> String {
    let piece_moving = piece_at_sq(pos, mov.from);
    let piece_being_captured = piece_at_sq(pos, mov.to);
    let piece_string = match piece_moving {
        P_WHITE => "",
        R_WHITE => "R",
        N_WHITE => "N",
        B_WHITE => "B",
        Q_WHITE => "Q",
        K_WHITE => "K",
        P_BLACK => "",
        R_BLACK => "R",
        N_BLACK => "N",
        B_BLACK => "B",
        Q_BLACK => "Q",
        K_BLACK => "K",
        _ => panic!("Unexpected"),
    };
    let capturing_pawn_string;
    let capture_string;
    if piece_being_captured != EMPTY {
        if piece_moving == P_WHITE || piece_moving == P_BLACK {
            capturing_pawn_string =
                f_to_string(sq_to_filerank(mov.from).f);
        } else {
            capturing_pawn_string = String::from("");
        }
        capture_string = "x";
    } else {
        capturing_pawn_string = String::from("");
        capture_string = "";
    }
    let check_or_checkmate_string;
    let mut new_pos = position_after_move(pos, mov);
    expand_position(&mut new_pos);
    if is_king_in_checkmate(&new_pos) {
        check_or_checkmate_string = "#";
    } else if is_king_in_check(&new_pos, false) {
        check_or_checkmate_string = "+";
    } else {
        check_or_checkmate_string = "";
    }
    let sq_string = sq_to_algstring(mov.to);
    let result = vec![
        piece_string,
        &capturing_pawn_string,
        capture_string,
        &sq_string,
        check_or_checkmate_string
    ];
    String::from(result.join(""))
}

fn main() {
    let starting_fen =
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let empty_fen = "8/8/8/8/8/8/8/8 w - - 0 1";
    let fen_problematic_shows_no_valid_moves =
            "8/4k3/3N1N2/4Q3/1B6/8/1K6/8 b - - 0 1";
    let fen = 
            "8/4k3/3P1P2/4Q3/1B6/8/1K6/8 w - - 0 1";
    let mut pos = decode_fen(
                    String::from(fen_problematic_shows_no_valid_moves));
    expand_position(&mut pos);
    match pos.moves {
        Some(ref moves) => {
            for mov in moves.iter() {
                println!("{}", move_to_string(mov, &pos));
            }
        },
        None => {},
    }
    println!("Done");
}
