use core::fmt;
use std::{io, u8};

const BOARD_SIZE: usize = 8;
const SHIP_TYPES: [u8; 4] = [2, 3, 3, 4];

#[derive(Copy, Clone)]
enum BoardPart {
    EMPTY,
    UNKNOWN,
    SHIP,
    HIT
}

enum ShipOrientation {
    VERTICAL,
    HORIZONTAL
}

impl fmt::Display for BoardPart {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BoardPart::EMPTY => write!(f, " "),
            BoardPart::UNKNOWN => write!(f, "?"),
            BoardPart::SHIP => write!(f, "S"),
            BoardPart::HIT => write!(f, "X"),
        }
    }
}

struct Player {
    name: String,
    ships: [Ship; SHIP_TYPES.len()],
    board: [[BoardPart; BOARD_SIZE]; BOARD_SIZE]
}

impl Player {
    pub fn check_coordinate_for_new_ship(&self, x: u8, y: u8, ship_length: u8, orientation: &ShipOrientation) -> bool {
        let mut local_x: usize = x.into();
        let mut local_y: usize = y.into();

        for i in 0..ship_length {
            if !matches!(self.board[local_x][local_y], BoardPart::EMPTY) {
                return false;
            }

            if matches!(orientation, ShipOrientation::HORIZONTAL) {
                local_y = local_y + 1;
            } else {
                local_x = local_x + 1;
            }
        }

        return true;
    }
}

#[derive(Copy, Clone)]
struct Ship {
    start_coordinate: [u8; 2],
    end_coordinate: [u8; 2],
    hit_count: u8
}

impl Ship {
    pub fn is_in_ship(&self, coordinate: [u8; 2]) -> bool {
        let number_between: bool;

        if self.start_coordinate[0] >= self.end_coordinate[0] {
            number_between = self.start_coordinate[0] >= coordinate[0] && self.end_coordinate[0] <= coordinate[0];
        } else {
            number_between = self.end_coordinate[0] >= coordinate[0] && self.start_coordinate[0] <= coordinate[0];
        }

        if !number_between {
            return false;
        }

        if self.start_coordinate[1] >= self.end_coordinate[1] {
            return self.start_coordinate[1] >= coordinate[1] && self.end_coordinate[1] <= coordinate[1];
        } else {
            return self.end_coordinate[1] >= coordinate[1] && self.start_coordinate[1] <= coordinate[1];
        }
    }

    pub fn register_hit(&mut self) {
        self.hit_count = self.hit_count + 1;
    }

    pub fn is_ship_destroyed(&self) -> bool {
        return self.hit_count >= self.get_length();
    }

    fn get_length(&self) -> u8 {
        let length: u8;
        if self.start_coordinate[0] - self.end_coordinate[0] == 0 {
            return self.start_coordinate[0].abs_diff(self.end_coordinate[0]);
        }
        return self.start_coordinate[1].abs_diff(self.end_coordinate[1]);
    }
}

fn main() {
    let mut name: String = String::new();
    println!("Torpedo");

    println!("Your name:");

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    
    println!("Your name: {}", name);

    let mut player: Player = Player { name: name, ships: create_empty_ships(), board: create_empty_board() };

    for i in 0..SHIP_TYPES.len() {
        let mut input_line = String::new();
        let mut x: i8 = -1; 
        let mut y: i8 = -1; 

        println!("Ship length: {}", SHIP_TYPES[i]);

        while x < 0 || y < 0 {
            println!("X:");
            input_line = String::from("");
            io::stdin()
                .read_line(&mut input_line)
                .expect("Failed to read line");
            x = input_line.trim().parse().unwrap_or(-1);

            if !is_valid_coordinate(x) {
                println!("Not valid coordainte!");
                continue;
            }

            println!("Y:");
            input_line = String::from("");
            io::stdin()
                .read_line(&mut input_line)
                .expect("Failed to read line");
            y = input_line.trim().parse().unwrap_or(-1);

            if !is_valid_coordinate(y) {
                println!("Not valid coordainte!");
                continue;
            }

            println!("Ship orientation:");
            println!("1: Horizontal");
            println!("2: Vertical");
            input_line = String::from("");
            io::stdin()
                .read_line(&mut input_line)
                .expect("Failed to read line");
            let orientation: ShipOrientation;
            if input_line.trim() == "1" {
                orientation = ShipOrientation::HORIZONTAL;
            } else if input_line.trim() == "2" {
                orientation = ShipOrientation::VERTICAL;
            } else {
                println!("Not valid input!");
                continue;
            }

            if is_ship_overlays_board(x as u8, y as u8, SHIP_TYPES[i], &orientation) {
                println!("Invalid position!");
                continue;
            }

            if !player.check_coordinate_for_new_ship(x as u8, y as u8, SHIP_TYPES[i], &orientation) {
                println!("Ships cannot cross each other");
                x = -1;
                y = -1;
            }
        }
    }

}

fn create_empty_board() -> [[BoardPart; BOARD_SIZE]; BOARD_SIZE] {
    return [[BoardPart::EMPTY;BOARD_SIZE];BOARD_SIZE];
}

fn create_empty_ships() -> [Ship; SHIP_TYPES.len()] {
    return [Ship{start_coordinate: [0, 0], end_coordinate: [0, 0], hit_count: 0}; SHIP_TYPES.len()];
}

fn is_valid_coordinate(x: i8) -> bool {
    return x >= 0 && x < BOARD_SIZE as i8;
}

fn is_ship_overlays_board(x: u8, y: u8, length: u8, orientation: &ShipOrientation) -> bool {
    if matches!(orientation, ShipOrientation::HORIZONTAL) {
        return x + length > BOARD_SIZE as u8;
    }
    return y + length > BOARD_SIZE as u8;
}
