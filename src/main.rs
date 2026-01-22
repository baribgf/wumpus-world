mod agent;
mod agents;
mod env;
mod grid;
mod kb;
mod logic;
mod room;
mod tui;

use crate::{
    agent::{Action, Agent, Direction},
    agents::KnowledgeBasedAgent,
    env::{ActionResult, Environment},
    tui::{confirm, invalid_input},
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

    tui::play_help();

    match mode {
        GameMode::Player => loop {
            tui::print_prompt();

            let mut direction: Option<Direction>;
            match tui::read_command().as_str() {
                "b" => {
                    if confirm() {
                        break;
                    }
                }
                "?" => {
                    tui::play_help();
                }
                "" => {}
                "g?" => {
                    tui::display_env(&env);
                }
                "s?" => {
                    tui::display_score(env.score());
                }
                mov if mov.starts_with("mv ") && {
                    direction = tui::parse_direction(mov);
                    direction.is_some()
                } =>
                {
                    match env.step(&Action::Move(direction.unwrap())) {
                        ActionResult::Ok => tui::display_env(&env),
                        ActionResult::Bump => tui::invalid_direction(),
                        ActionResult::GameOver => {
                            tui::display_env(&env);
                            match tui::game_over(env.score()) {
                                true => break,
                                false => {
                                    env.initialize();
                                    tui::play_help();
                                }
                            }
                        }
                        _ => {}
                    };
                }
                shoot
                    if shoot.starts_with("sh ") && {
                        direction = tui::parse_direction(shoot);
                        direction.is_some()
                    } =>
                {
                    match env.step(&Action::Shoot(direction.unwrap())) {
                        ActionResult::Scream => {
                            tui::display_env(&env);
                            println!("You killed the Wumpus!");
                        }
                        ActionResult::Bump => tui::invalid_direction(),
                        ActionResult::Ok => {}
                        _ => {}
                    };
                }
                "cl" => {
                    match env.step(&Action::Climb) {
                        ActionResult::CannotClimb => println!("Cannot climb from here!"),
                        ActionResult::GameOver => match tui::game_over(env.score()) {
                            true => break,
                            false => {
                                env.initialize();
                                tui::play_help();
                            }
                        },
                        _ => {}
                    };
                }
                _ => invalid_input(),
            }
        },
        GameMode::Agent => {
            let mut agent = KnowledgeBasedAgent::new();

            loop {
                let action = agent.act(env.observation());
                match &action {
                    Action::Move(_) => match env.step(&action) {
                        ActionResult::Ok => tui::display_env(&env),
                        ActionResult::GameOver => {
                            tui::display_env(&env);
                            match tui::game_over(env.score()) {
                                true => break,
                                false => env.initialize(),
                            }
                        }
                        _ => {}
                    },
                    Action::Shoot(_) => match env.step(&action) {
                        ActionResult::Scream => {
                            tui::display_env(&env);
                            println!("The Wumpus is killed!");
                        }
                        _ => {}
                    },
                    Action::Climb => match env.step(&action) {
                        ActionResult::GameOver => match tui::game_over(env.score()) {
                            true => break,
                            false => env.initialize(),
                        },
                        _ => {}
                    },
                }
            }
        }
    }
}
