use std::cmp::PartialEq;
use std::io;
use crate::Faction::{CROSS, NOUGHT, UNCLAIMED};

#[derive(Copy, Clone)]
enum Faction {
    UNCLAIMED,
    CROSS,
    NOUGHT
}

impl Faction {

    pub fn char(&self) -> &str {
        match self {
            UNCLAIMED => "-",
            CROSS => "X",
            NOUGHT => "O",
        }
    }
}

impl PartialEq for Faction {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Faction::UNCLAIMED, Faction::UNCLAIMED) => true,
            (Faction::CROSS, Faction::CROSS) => true,
            (Faction::NOUGHT, Faction::NOUGHT) => true,
            _ => false,
        }
    }
}

fn get_faction(value: bool) -> Faction {
    match value {
        true => CROSS,
        false => NOUGHT,
    }
}

fn is_winner(faction: Faction, sq: &[Faction; 9]) -> bool {
    (sq[0] == faction && sq[1] == faction && sq[2] == faction)
        || (sq[3] == faction && sq[4] == faction && sq[5] == faction)
        || (sq[6] == faction && sq[7] == faction && sq[8] == faction)
        || (sq[0] == faction && sq[3] == faction && sq[6] == faction)
        || (sq[1] == faction && sq[4] == faction && sq[7] == faction)
        || (sq[2] == faction && sq[5] == faction && sq[8] == faction)
        || (sq[0] == faction && sq[4] == faction && sq[8] == faction)
        || (sq[2] == faction && sq[4] == faction && sq[6] == faction)
}

fn main() {
    let mut chosen_square: usize;

    let mut squares: [Faction; 9] = [Faction::UNCLAIMED; 9];
    let mut turn: bool = false;

    loop {
        for i in 0..9 {
            if (i % 3 == 0) {
                println!();
            }
            print!("{}", squares[i].char());
        }
        println!();
        println!("Pick an unclaimed square: ");

        let mut input: String = String::new();

        io::stdin().read_line(&mut input).unwrap();

        chosen_square = match input.trim().parse::<usize>() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number!");
                continue;
            }
        };

        match squares[chosen_square] {
            Faction::UNCLAIMED => {
                let faction: Faction = get_faction(turn);

                squares[chosen_square] = faction;

                if is_winner(faction, &squares) {
                    println!("Game Over!");
                    break;
                }

                turn = !turn;
            }
            _ => {
                println!("Already claimed!");
                continue;
            }
        }
    }
}