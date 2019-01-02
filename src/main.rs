extern crate ncurses;

use ncurses::*;

const ASCII_N:i32 = 110;
const ASCII_Q:i32 = 113;
const ASCII_SPACE:i32 = 32;

const TILE_WIDTH:i32 = 3;
const N:char = '-';
const X:char = 'X';
const O:char = 'O';

const H0:&[i32; 3] = &[0, 1, 2];
const H1:&[i32; 3] = &[3, 4, 5];
const H2:&[i32; 3] = &[6, 7, 8];
const V0:&[i32; 3] = &[0, 3, 6];
const V1:&[i32; 3] = &[1, 4, 7];
const V2:&[i32; 3] = &[2, 5, 8];
const D0:&[i32; 3] = &[0, 4, 8];
const D1:&[i32; 3] = &[2, 4, 6];


fn main() {
    initscr();
    noecho();
    keypad(stdscr(), true);

    loop {
        match menu() {
            ASCII_N => new(&mut [N, N, N, N, N, N, N, N, N]),
            ASCII_Q => break,
            _ => (),
        }
    }

    endwin();
}

fn menu() -> i32 {
    clear();
    mvprintw(1, 1, "~ Tit Tac Toe ~ A rust implementation ~");
    mvprintw(3, 1, "`n` to play");
    mvprintw(4, 1, "`q` to quit");
    mv(6, 0);
    getch()
}

fn new(board:&mut [char; 9]) {
    let mut current_position = (1, 1);
    let mut current_turn:char = X;

    clear();
    loop {
        display(board, current_position);

        if check_for_win(board, current_position) == true {
            print_message("WIN! :)");
            break;
        }

        if check_for_draw(board) == true {
            print_message("Draw Game :|");
            break;
        }

        let ch = getch();
        match ch {
            KEY_LEFT => current_position = move_left(current_position),
            KEY_RIGHT => current_position = move_right(current_position),
            KEY_UP => current_position = move_up(current_position),
            KEY_DOWN => current_position = move_down(current_position),
            ASCII_SPACE => current_turn = next_turn(board, current_position, current_turn),
            ASCII_Q => break,
            _ => (),
        }
    }
}

fn move_left(position:(i32, i32)) -> (i32, i32) {
    return if position.1 > 0 {
        (position.0, position.1 - 1)
    } else { position }
}

fn move_right(position:(i32, i32)) -> (i32, i32) {
    return if position.1 < 2 {
        (position.0, position.1 + 1)
    } else { position }
}

fn move_up(position:(i32, i32)) -> (i32, i32) {
    return if position.0 > 0 {
        (position.0 - 1, position.1)
    } else { position }
}

fn move_down(position:(i32, i32)) -> (i32, i32) {
    return if position.0 < 2 {
        (position.0 + 1, position.1)
    } else { position }
}

fn check_for_win(board:&mut[char; 9], position:(i32, i32)) -> bool {
    match position {
        (0, 0) => return check_triplets(board, &[H0, V0, D0]),
        (0, 1) => return check_triplets(board, &[H0, V1]),
        (0, 2) => return check_triplets(board, &[H0, V2, D1]),
        (1, 0) => return check_triplets(board, &[H1, V0]),
        (1, 1) => return check_triplets(board, &[H1, V1, D0, D1]),
        (1, 2) => return check_triplets(board, &[H1, V2]),
        (2, 0) => return check_triplets(board, &[H2, V0, D1]),
        (2, 1) => return check_triplets(board, &[H2, V1]),
        (2, 2) => return check_triplets(board, &[H2, V2, D0]),
        _ => return false,
    }
}

fn check_triplets(board:&mut[char; 9], triplets:&[&[i32; 3]]) -> bool {
    for triplet in triplets {
        if check_triplet(board, **triplet) == true {
            return true;
        }
    }

    false
}

fn check_triplet(board:&mut[char; 9], triplet:[i32; 3]) -> bool {
    let tiles = [
        board[triplet[0] as usize],
        board[triplet[1] as usize],
        board[triplet[2] as usize],
    ];

    for tile in tiles.iter() {
        if *tile == N {
            return false;
        }
    }

    if tiles[0] != tiles[1] {
        return false;
    }

    if tiles[1] != tiles[2] {
        return false;
    }

    true
}

fn check_for_draw(board:&mut[char; 9]) -> bool {
    for item in board.iter() {
        if *item == N {
            return false;
        }
    }

    true
}

fn print_message(message:&str) {
    mvprintw(1, 10, message);
    mv(4, 1);
    getch();
}

fn next_turn(board:&mut[char; 9], position:(i32, i32), turn:char) -> char {
    let index = get_index_from_position(position) as usize;
    let mut next_turn = turn;

    if board[index] == N {
        board[index] = turn;

        if turn == X {
            next_turn = O;
        } else {
            next_turn = X;
        }
    }

    next_turn
}

fn get_index_from_position(position:(i32, i32)) -> i32 {
    position.0 * TILE_WIDTH + position.1
}

fn display(board:&[char; 9], current:(i32, i32)) {
    let mut i = 0;

    for c in board.iter() {
        let position = (i / 3, i % 3);

        let mut tile_string = format!(" {} ", c);

        if position == current {
            tile_string = format!("[{}]", c);
        }

        mvprintw(position.0, position.1 * TILE_WIDTH, &tile_string);
        i = i + 1;
    }
}
