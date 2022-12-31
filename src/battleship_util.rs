use crate::BOARD_SIZE;
use crate::BoardPart;
use crate::ShipOrientation;

pub fn create_empty_board() -> [[BoardPart; BOARD_SIZE]; BOARD_SIZE] {
    return [[BoardPart::EMPTY;BOARD_SIZE];BOARD_SIZE];
}

pub fn is_valid_coordinate(x: i8) -> bool {
    return x >= 0 && x < BOARD_SIZE as i8;
}

pub fn is_ship_overlays_board(x: u8, y: u8, length: u8, orientation: &ShipOrientation) -> bool {
    if matches!(orientation, ShipOrientation::HORIZONTAL) {
        return x + length > BOARD_SIZE as u8;
    }
    return y + length > BOARD_SIZE as u8;
}

pub fn create_opponent_board() -> [[BoardPart; BOARD_SIZE]; BOARD_SIZE] {
    return [[BoardPart::UNKNOWN;BOARD_SIZE];BOARD_SIZE];
}

pub fn print_board(board: &[[BoardPart; BOARD_SIZE]; BOARD_SIZE]) {
    print!("  ");
    for x in 0..BOARD_SIZE {
        print!("{} ", x);
    }
    print!("|X|");
    println!("");

    for y in 0..BOARD_SIZE {
        print!("{} ", y);
        for x in 0..BOARD_SIZE {
            print!("{} ", board[x][y]);
        }
        println!("");
    }
    println!("|Y|");
}
