extern crate rand;
extern crate regex;
extern crate termion;
extern crate userio;

use ai::Ai;
use board::Board;
use human::Human;
use player::Player;
use utils;

use game::rand::prelude::*;
use self::regex::Regex;

pub fn play() {
    print!("{}", termion::clear::All);
    print!("{}", termion::cursor::Goto(1, 1));

    let mut board = Board::new();
    let players = get_players(); 
    let mut turn = 0;

    while !board.is_over() {
        println!("Tested Game over!");
        board.print();
        let current_player = &players[turn];
        let success = current_player.play_move(&mut board);
        if success {
            turn = 1 - turn;
        }
    }
    board.print();
    println!("Game Over!");    
}

fn get_players() -> Vec<Box<Player>> {
    let user_want_to_play_against_ai = userio::prompt_user(userio::PromptUserInfo{
        address_msg: "",
        prompt_msg: "Would you like to play against the AI?",
        user_response_validation_pattern: "(^[yY][eE][sS]$)|(^[nN][oO]$)",
        err_msg: "Please answer yes or no."
    });
    
    let yes_regex = Regex::new("[yY][eE][sS]").unwrap();
    if yes_regex.is_match(&user_want_to_play_against_ai) {
        let player_wish_to_goes_first = userio::prompt_user(userio::PromptUserInfo{
            address_msg: "",
            prompt_msg: "Do you want to go first?",
            user_response_validation_pattern: "(^[yY][eE][sS]$)|(^[nN][oO]$)",
            err_msg: "Please answer yes or no."
        });

        if yes_regex.is_match(&player_wish_to_goes_first) {
            let player_token = userio::prompt_user(userio::PromptUserInfo{
                address_msg: "",
                prompt_msg: "What token do you want: X or O?",
                user_response_validation_pattern: "^(X|O)$",
                err_msg: "Token must be a X or a O, please try again!"
            });
            if &player_token == "X" {
                return vec!(Box::new(Human::new("X")), Box::new(Ai::new(utils::get_other_token("X"))));
            } else {
                return vec!(Box::new(Human::new("O")), Box::new(Ai::new(utils::get_other_token("O"))));
            }
        } else {
            let mut rng = thread_rng();
            if rng.gen_range(0, 100) < 50 {
                return vec!(Box::new(Ai::new("X")), Box::new(Human::new(utils::get_other_token("X"))));
            } else {
                return vec!(Box::new(Ai::new("O")), Box::new(Human::new(utils::get_other_token("O"))));
            }
        }
    } else {
        let first_player_token = userio::prompt_user(userio::PromptUserInfo{
            address_msg: "",
            prompt_msg: "What token should the first player use?",
            user_response_validation_pattern: "^(X|O)$",
            err_msg: "Token must be a X or a O, please try again!"
        });
        if &first_player_token == "X" {
            return vec!(Box::new(Human::new("X")), Box::new(Human::new(utils::get_other_token("X"))));
        } else {
            return vec!(Box::new(Human::new("O")), Box::new(Human::new(utils::get_other_token("O"))));
        }
    }
}
