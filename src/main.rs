mod agent;
mod agents;
mod env;
mod grid;
mod logic;
mod room;
mod tui;

use crate::{
    agent::{Action, Agent},
    agents::KeyboardAgent,
    env::{ActionResult, Environment},
    tui::confirm,
};

#[derive(PartialEq)]
enum GameMode {
    Player,
    Agent,
}

fn main() {
    println!("Welcome to Wumpus World!");

    loop {
        tui::main_menu();

        match tui::read_command().as_str() {
            "q" => {
                println!("Good bye!");
                break;
            }
            "p" => {
                play(GameMode::Player);
            }
            "a" => {
                // @TODO: Implement agent mode
                play(GameMode::Agent);
            }
            "h" => {
                tui::general_help();
            }
            "" => {}
            _ => {
                tui::invalid_input();
            }
        }
    }
}

fn play(mode: GameMode) {
    println!();
    println!("Initializing game..");

    let mut env = Environment::new();
    let mut agent = KeyboardAgent::new();

    println!();
    tui::play_help();
    println!();

    loop {
        if mode == GameMode::Player {
            tui::print_prompt();

            let mut agent_cmd = false;
            let cmd = tui::read_command();
            match cmd.as_str() {
                "b" => {
                    if confirm() {
                        break;
                    }
                }
                "?" => {
                    tui::play_help();
                    println!();
                }
                "" => {}
                "g?" => {
                    tui::display_env(&env);
                }
                "s?" => {
                    tui::display_score(env.score());
                }
                _ => {
                    agent_cmd = true;
                    agent.set_cmd(cmd);
                }
            }

            if !agent_cmd {
                continue;
            }
        }

        let action = agent.act(env.observation());
        match &action {
            Action::Move(_) => match env.step(&action) {
                ActionResult::Ok => tui::display_env(&env),
                ActionResult::Bump => tui::invalid_direction(),
                ActionResult::GameOver => {
                    tui::display_env(&env);
                    match tui::game_over(env.score()) {
                        true => break,
                        false => {
                            env.initialize();
                            tui::play_help();
                            println!();
                        }
                    }
                }
                _ => {}
            },
            Action::Shoot(_) => match env.step(&action) {
                ActionResult::Scream => {
                    tui::display_env(&env);
                    println!("You killed the Wumpus!");
                }
                ActionResult::Bump => tui::invalid_direction(),
                _ => {}
            },
            Action::Climb => match env.step(&action) {
                ActionResult::CannotClimb => println!("Cannot climb from here!"),
                ActionResult::GameOver => match tui::game_over(env.score()) {
                    true => break,
                    false => {
                        env.initialize();
                        tui::play_help();
                        println!();
                    }
                },
                _ => {}
            },
            Action::None => {
                panic!("Agent cannot act!");
            }
        }
    }
}
