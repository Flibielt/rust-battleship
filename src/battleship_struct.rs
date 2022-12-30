use crate::BOARD_SIZE;

use crate::battleship_enum::BoardPart;
use crate::battleship_enum::ShipOrientation;

pub struct Player {
    name: String,
    ships: Vec<Ship>,
    board: [[BoardPart; BOARD_SIZE]; BOARD_SIZE]
}

impl Player {
    pub fn new(name: String) -> Self {
        Self { name: name, ships: Vec::new(), board: crate::battleship_util::create_empty_board() }
    }

    pub fn check_coordinate_for_new_ship(&self, x: u8, y: u8, ship_length: u8, orientation: &ShipOrientation) -> bool {
        let mut local_x: usize = x.into();
        let mut local_y: usize = y.into();

        for _i in 0..ship_length {
            if !matches!(self.board[local_x][local_y], BoardPart::EMPTY) {
                return false;
            }

            if matches!(orientation, ShipOrientation::HORIZONTAL) {
                local_x = local_x + 1;
            } else {
                local_y = local_y + 1;
            }
        }

        return true;
    }

    pub fn register_ship(&mut self, x: u8, y: u8, ship_length: u8, orientation: &ShipOrientation) {
        let mut local_x: usize = x.into();
        let mut local_y: usize = y.into();
        for _i in 0..ship_length {
            self.board[local_x][local_y] = BoardPart::SHIP;
            
            if matches!(orientation, ShipOrientation::HORIZONTAL) {
                local_x = local_x + 1;
            } else {
                local_y = local_y + 1;
            }
        }

        self.ships.push(Ship{start_coordinate: [x, y], end_coordinate: [local_x as u8, local_y as u8], hit_count: 0})
    }

    pub fn print_board(&self) {
        println!("Name: {}", self.name);

        print!("  ");
        for x in 0..BOARD_SIZE {
            print!("{} ", x);
        }
        print!("|X|");
        println!("");

        for y in 0..BOARD_SIZE {
            print!("{} ", y);
            for x in 0..BOARD_SIZE {
                print!("{} ", self.board[x][y]);
            }
            println!("");
        }
        println!("|Y|")
    }
}

#[derive(Copy, Clone)]
pub struct Ship {
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
