use board;

pub trait Player {

    fn new(token: &'static str) -> Self where Self: Sized;

    fn get_token(&self) -> &'static str;
    fn get_type(&self) -> &'static str;

    fn play_move(&self, board: &mut board::Board) -> bool ;

}
