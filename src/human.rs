extern crate userio;

use board;
use player;

pub const TYPE: &str = "Human";
const MOVE_VALIDATION_PATTERN: &str = "^[1234567]$";

pub struct Human {
    token: &'static str,
    player_type: &'static str
}

impl player::Player for Human {

    fn new(token: &'static str) -> Human {
        Human {
            token: token,
            player_type: TYPE
        }
    }

    fn get_token(&self) -> &'static str {
        self.token 
    }

    fn get_type(&self) -> &'static str {
        self.player_type
    }

    fn play_move(&self, board: &mut board::Board) -> bool {
        let address_msg = &format!("To the player with token {}:", self.token);
        let col_prompt = "Which column do you want to put your token?";
        let col_err = "Not a valid column, please try again!";

        let col = userio::prompt_user(userio::PromptUserInfo{
            address_msg: address_msg,
            prompt_msg: col_prompt,
            user_response_validation_pattern: MOVE_VALIDATION_PATTERN,
            err_msg: col_err
        }).parse::<usize>().unwrap();

        board.put(col-1, self.token)
    }

}
