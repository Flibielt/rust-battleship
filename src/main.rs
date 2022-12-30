use core::fmt;

const BOARD_SIZE: usize = 8;
const SHIP_TYPES: [u8; 4] = [2, 3, 3, 4];

#[derive(Copy, Clone)]
enum BoardPart {
    EMPTY,
    UNKNOWN,
    SHIP,
    HIT
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
    ships: [Ship; SHIP_TYPES.len()]
}

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
    println!("Torpedo");
    println!("{}", create_empty_board()[0][0])
}

fn create_empty_board() -> [[BoardPart; BOARD_SIZE]; BOARD_SIZE] {
    return [[BoardPart::EMPTY;BOARD_SIZE];BOARD_SIZE];
}
