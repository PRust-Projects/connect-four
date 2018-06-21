#[macro_use]
extern crate clap;

mod ai;
mod board;
mod game;
mod human;
mod player;
mod utils;

fn main() {
    let matches = clap_app!(connectfour => 
                            (version: "1.0")
                            (author: "PChan")
                            (about: "Time to challenge the Connect Four master!")
                            (@subcommand play => 
                             (version: "1.0")
                             (author: "PChan")
                             (about: "Play Connect Four!"))).get_matches();

    if let Some(_matches) = matches.subcommand_matches("play") {
        game::play();
    } else {
        println!("This app allows you to play Connect Four against another person or the computer!\n");
        println!("Run `connect-four help` for more information!")
    }
}    
