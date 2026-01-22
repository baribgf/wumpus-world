use std::io::{stdout, Write};

use crate::{agent::Direction, env::Environment};

fn flush() {
    stdout().flush().unwrap();
}

pub fn read_command() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

pub fn print_prompt() {
    println!();
    print!("> ");
    flush();
}

pub fn parse_direction(cmd: &str) -> Option<Direction> {
    match cmd.split_whitespace().nth(1) {
        Some(d) => match d {
            "n" => Some(Direction::North),
            "s" => Some(Direction::South),
            "e" => Some(Direction::East),
            "w" => Some(Direction::West),
            _ => None,
        },
        None => None,
    }
}

pub fn invalid_input() {
    println!("Invalid input!");
}

pub fn invalid_direction() {
    println!("Invalid direction!");
}

pub fn main_menu() {
    println!();
    println!("[p] Play");
    println!("[a] Run Agent");
    println!("[h] Help");
    println!();
    println!("[q] Quit");
    print_prompt();
}

pub fn play_help() {
    println!();
    println!("[g?] View grid");
    println!("[s?] View score");
    println!("[mv] Move [n,s,e,w]");
    println!("[sh] Shoot arrow [n,s,e,w]");
    println!("[cl] Climb out");
    println!();
    println!("[?] Help");
    println!("[b] Back to Main Menu");
}

pub fn game_over(score: isize) -> bool {
    println!("Game Over! Score: {}", score);
    println!();
    loop {
        print!("Replay? [y,n]: ");
        flush();
        match read_command().as_str() {
            "y" => return false,
            "n" => return true,
            _ => invalid_input(),
        }
    }
}

pub fn confirm() -> bool {
    loop {
        print!("Are you sure? [y,n]: ");
        flush();
        match read_command().as_str() {
            "y" => return true,
            "n" => return false,
            _ => invalid_input(),
        }
    }
}

pub fn general_help() {
    println!("\n= Help");
    println!("=");
}

pub fn display_env(env: &Environment) {
    println!("{}", env);
}

pub fn display_score(score: isize) {
    println!("Score: {}", score);
}
