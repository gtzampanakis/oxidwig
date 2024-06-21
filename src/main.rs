#![allow(dead_code)]
#![allow(unused_variables)]

type Sq = i8;
type AlgSq = [String; 2];
type Piece = i8;
type Direction = i8;
type File = i8;
type Rank = i8;
type Color = i8;

const COLOR_WHITE: Color = 1;
const COLOR_BLACK: Color = -1;
const COLOR_EMPTY: Color = 0;

const P_BASE: Piece = 1;
const R_BASE: Piece = 2;
const N_BASE: Piece = 3;
const B_BASE: Piece = 4;
const Q_BASE: Piece = 5;
const K_BASE: Piece = 6;

const P_WHITE: Piece = P_BASE;
const R_WHITE: Piece = R_BASE;
const N_WHITE: Piece = N_BASE;
const B_WHITE: Piece = B_BASE;
const Q_WHITE: Piece = Q_BASE;
const K_WHITE: Piece = K_BASE;
const E: Piece = 9;
const P_BLACK: Piece = E + P_BASE;
const R_BLACK: Piece = E + R_BASE;
const N_BLACK: Piece = E + N_BASE;
const B_BLACK: Piece = E + B_BASE;
const Q_BLACK: Piece = E + Q_BASE;
const K_BLACK: Piece = E + K_BASE;

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

struct Position {
    placement: [Piece; 64],
    active_color: Color,
    castling: [bool; 4],
    en_passant: Option<Sq>,
    halfmoves: i32,
    fullmoves: i32,
}

fn empty_position() -> Position {
    Position{
        placement: [E; 64],
        active_color: COLOR_WHITE,
        castling: [false; 4],
        en_passant: None,
        halfmoves: 0,
        fullmoves: 0,
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
    piece_at_sq(pos, sq) != E
}

fn is_move_capture(pos: &Position, mov: &Move) -> bool {
    is_there_piece_at_sq(pos, mov.to)
}

//fn move_to_alg(pos: &Position, mov: &Move) -> String {
//    let piece_moving = mov.piece;
//    let sq_from = mov.from; 
//    let sq_to = mov.to;
//    let algsq = sq_to_algsq(sq_to);
//    let is_capture = is_move_capture(pos, mov);
//    let capture_str = String::from(
//        match is_capture {
//            true => "x",
//            false => "",
//        }
//    );
//    let piece_moving_str = match piece_moving % E {
//        P_BASE => match is_capture {
//            true => f_to_string(
//                        sq_to_filerank(sq_from).f),
//            false => String::from(""),
//        },
//        N_BASE => String::from("N"),
//        B_BASE => String::from("B"),
//        Q_BASE => String::from("Q"),
//        R_BASE => String::from("R"),
//        K_BASE => String::from("K"),
//        _ => panic!("Unexpected!"),
//    };
//
//    for sq in 0..64 {
//        let piece = piece_at_sq(pos, sq);
//        if piece % E != P_BASE {
//            if piece != piece_moving {
//                if (
//                    sq_to_filerank(sq).f
//                        ==
//                    sq_to_filerank(sq_from).f
//                ) {
//                    if sq != sq_from {
//                    }
//                }
//            }
//        }
//    }
//
//    String::from("foo")
//}
//

fn is_piece_white(piece: Piece) -> bool {
    piece < E
}

fn is_piece_black(piece: Piece) -> bool {
    piece > E
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
    piece % E
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
                'r' => set_piece_at_sq(
                                &mut p, fr_to_sq(f, r), R_BLACK),
                'n' => set_piece_at_sq(
                                &mut p, fr_to_sq(f, r), N_BLACK),
                'b' => set_piece_at_sq(
                                &mut p, fr_to_sq(f, r), B_BLACK),
                'q' => set_piece_at_sq(
                                &mut p, fr_to_sq(f, r), Q_BLACK),
                'k' => set_piece_at_sq(
                                &mut p, fr_to_sq(f, r), K_BLACK),
                'p' => set_piece_at_sq(
                                &mut p, fr_to_sq(f, r), P_BLACK),
                'R' => set_piece_at_sq(
                                &mut p, fr_to_sq(f, r), R_WHITE),
                'N' => set_piece_at_sq(
                                &mut p, fr_to_sq(f, r), N_WHITE),
                'B' => set_piece_at_sq(
                                &mut p, fr_to_sq(f, r), B_WHITE),
                'Q' => set_piece_at_sq(
                                &mut p, fr_to_sq(f, r), Q_WHITE),
                'K' => set_piece_at_sq(
                                &mut p, fr_to_sq(f, r), K_WHITE),
                'P' => set_piece_at_sq(
                                &mut p, fr_to_sq(f, r), P_WHITE),
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

fn legal_moves_from_sq(pos: &Position, sq: Sq) -> Vec<Move> {
    let piece = piece_at_sq(pos, sq);
    let color = piece_color(piece);
    let base = piece_base(piece);
    match base {
        //P_BASE => legal_moves_for_pawn(pos, sq, color),
        //R_BASE => legal_moves_for_rook(pos, sq, color),
        //N_BASE => legal_moves_for_knight(pos, sq, color),
        //B_BASE => legal_moves_for_bishop(pos, sq, color),
        //Q_BASE => legal_moves_for_queen(pos, sq, color),
        //K_BASE => legal_moves_for_king(pos, sq, color),
        _ => panic!("Unexpected!"),
    }
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

//fn for_each_sq_in_dir(
//        sq: Sq,
//        dir:Direction,
//        func: fn(Sq)
//) {
//    let filerank = sq_to_filerank(sq);
//    let mut f = filerank.f;
//    let mut r = filerank.r;
//    let mut n_dir = false;
//    loop {
//        match dir {
//            DIR_U => { r += 1; },
//            DIR_R => { f += 1 },
//            DIR_D => { r -= 1 },
//            DIR_L => { f -= 1 },
//            DIR_UR => { r += 1; f += 1 },
//            DIR_DR => { r -= 1; f += 1 },
//            DIR_DL => { r -= 1; f -= 1 },
//            DIR_UL => { r += 1; f -= 1 },
//            DIR_NUR => { n_dir = true; r += 2; f += 1 },
//            DIR_NRU => { n_dir = true; f += 2; r += 1 },
//            DIR_NRD => { n_dir = true; f += 2; r -= 1 },
//            DIR_NDR => { n_dir = true; r -= 2; f += 1 },
//            DIR_NDL => { n_dir = true; r -= 2; f -= 1 },
//            DIR_NLD => { n_dir = true; f -= 2; r -= 1 },
//            DIR_NLU => { n_dir = true; f -= 2; r += 1 },
//            DIR_NUL => { n_dir = true; r += 2; f -= 1 },
//            _ => panic!("Unexpected!"),
//        };
//        if n_dir || f < 0 || f > 7 || r < 0 || r > 7 {
//            break;
//        } else {
//            if func(fr_to_sq(f, r)) {
//                break;
//            };
//        }
//    }
//}

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
    pos: &Position, sq: Sq, fr0: &FileRank, color: Color, func: fn(Sq)
) {
    let mut fr = FileRank{f: fr0.f, r: fr0.r};
    if color == COLOR_WHITE {
        // One forward.
        apply_dir_u(&mut fr);
        {
            let piece_found = piece_at_sq(pos, filerank_to_sq(&fr));
            if piece_found == E {
                func(filerank_to_sq(&fr));
            }
        }
        if fr0.r == 1 {
            // Two forward.
            apply_dir_u(&mut fr);
            let piece_found = piece_at_sq(pos, filerank_to_sq(&fr));
            if piece_found == E {
                func(filerank_to_sq(&fr));
            }
        }
        // Captures
        {
            fr = FileRank{f: fr0.f, r: fr0.r};
            apply_dir_ur(&mut fr);
            let piece_found = piece_at_sq(pos, filerank_to_sq(&fr));
            if piece_color(piece_found) == -color {
                func(filerank_to_sq(&fr));
            }
        }
        {
            fr = FileRank{f: fr0.f, r: fr0.r};
            apply_dir_ul(&mut fr);
            let piece_found = piece_at_sq(pos, filerank_to_sq(&fr));
            if piece_color(piece_found) == -color {
                func(filerank_to_sq(&fr));
            }
        }
    } else {
        // One forward.
        apply_dir_d(&mut fr);
        {
            let piece_found = piece_at_sq(pos, filerank_to_sq(&fr));
            if piece_found == E {
                func(filerank_to_sq(&fr));
            }
        }
        if fr0.r == 6 {
            // Two forward.
            apply_dir_d(&mut fr);
            let piece_found = piece_at_sq(pos, filerank_to_sq(&fr));
            if piece_found == E {
                func(filerank_to_sq(&fr));
            }
        }
        // Captures
        {
            fr = FileRank{f: fr0.f, r: fr0.r};
            apply_dir_dr(&mut fr);
            let piece_found = piece_at_sq(pos, filerank_to_sq(&fr));
            if piece_color(piece_found) == -color {
                func(filerank_to_sq(&fr));
            }
        }
        {
            fr = FileRank{f: fr0.f, r: fr0.r};
            apply_dir_dl(&mut fr);
            let piece_found = piece_at_sq(pos, filerank_to_sq(&fr));
            if piece_color(piece_found) == -color {
                func(filerank_to_sq(&fr));
            }
        }
    }
}

fn for_each_legal_sq_from_sq(
    pos: &Position, sq: Sq, func: fn(Sq)
) {
    let fr0 = sq_to_filerank(sq);
    let piece = piece_at_sq(pos, sq);
    let color = piece_color(piece);
    let piece_base = piece % E;
    if piece_base == P_BASE {
        for_each_legal_sq_for_pawn(pos, sq, &fr0, color, func);
        return;
    }
    let apply_funcs = match piece_base {
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
    for apply_func in apply_funcs {
        let mut fr = FileRank{f: fr0.f, r: fr0.r};
        loop {
            if apply_func(&mut fr) {
                break;
            }
            let piece_found = piece_at_sq(pos, filerank_to_sq(&fr));
            let piece_found_color = piece_color(piece_found);
            if piece_found_color == COLOR_EMPTY {
                func(filerank_to_sq(&fr));
            } else if piece_found_color == color {
                break;
            } else {
                func(filerank_to_sq(&fr));
                break;
            }
            match piece_base {
                N_BASE => { break; },
                K_BASE => { break; },
                _ => {},
            }
        }
    }
}


fn main() {
    let pos = decode_fen(String::from(
        "8/8/8/8/8/8/1r6/P7 w - - 0 1"));
    let sq = fr_to_sq(0, 0);
    fn print_sq(sq: Sq) {
        println!("{}", sq_to_algstring(sq));
    }
    for_each_legal_sq_from_sq(&pos, sq, print_sq);
    println!("Hello world!");
}
