use std::{io, u8};

mod battleship_enum;
mod battleship_struct;
mod battleship_util;

use battleship_enum::BoardPart;
use battleship_enum::ShipOrientation;

use battleship_struct::Player;

const BOARD_SIZE: usize = 8;
const SHIP_TYPES: [u8; 4] = [2, 3, 3, 4];

fn main() {
    let mut name: String = String::new();
    println!("Torpedo");

    println!("Your name:");

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    
    println!("Your name: {}", name);

    let mut player: Player = Player::new(name);
    get_ships_from_user(&mut player);

    player.print_board();

    let opponent_board = battleship_util::create_opponent_board();
    battleship_util::print_board(&opponent_board);

}

fn get_ships_from_user(player: &mut Player) {
    for i in 0..SHIP_TYPES.len() {
        let mut input_line: String;
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

            if !battleship_util::is_valid_coordinate(x) {
                println!("Not valid coordainte!");
                x = -1;
                y = -1;
                continue;
            }

            println!("Y:");
            input_line = String::from("");
            io::stdin()
                .read_line(&mut input_line)
                .expect("Failed to read line");
            y = input_line.trim().parse().unwrap_or(-1);

            if !battleship_util::is_valid_coordinate(y) {
                println!("Not valid coordainte!");
                x = -1;
                y = -1;
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
                x = -1;
                y = -1;
                continue;
            }

            if battleship_util::is_ship_overlays_board(x as u8, y as u8, SHIP_TYPES[i], &orientation) {
                println!("Invalid position!");
                x = -1;
                y = -1;
                continue;
            }

            if !player.check_coordinate_for_new_ship(x as u8, y as u8, SHIP_TYPES[i], &orientation) {
                println!("Ships cannot cross each other");
                x = -1;
                y = -1;
                continue;
            }

            player.register_ship(x as u8, y as u8, SHIP_TYPES[i], &orientation);
            player.print_board();
        }
    }
}
