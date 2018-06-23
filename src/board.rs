extern crate termion;

use std::fmt;

pub const HEIGHT: usize = 6;
pub const WIDTH: usize = 7;

pub struct Board {
    board: [[&'static str; WIDTH]; HEIGHT],
    bottom_most_empty_spots: [i32; WIDTH],
    last_move: [i32; 2],
    empty_spots_count: i32
}

impl Board {

    pub fn new() -> Board {
        Board {
            board: [[" ", " ", " ", " ", " ", " ", " "],
            [" ", " ", " ", " ", " ", " ", " "],
            [" ", " ", " ", " ", " ", " ", " "],
            [" ", " ", " ", " ", " ", " ", " "],
            [" ", " ", " ", " ", " ", " ", " "],
            [" ", " ", " ", " ", " ", " ", " "]],
            bottom_most_empty_spots: [5, 5, 5, 5, 5, 5, 5],
            last_move: [-1, -1],
            empty_spots_count: 42,
        }
    }

    pub fn get(&self, row: usize, col: usize) -> &'static str {
        if row < HEIGHT && col < WIDTH {
            return self.board[row][col];
        }
        return "";
    }

    pub fn put(&mut self, col: usize, token: &'static str) -> bool {
        let row = self.bottom_most_empty_spots[col];
        if row == -1 {
            return false;
        }
        self.board[row as usize][col] = token;
        self.last_move = [row, col as i32];
        self.bottom_most_empty_spots[col] -= 1;
        self.empty_spots_count -= 1;
        true
    }

    pub fn remove(&mut self, col: usize) -> bool {
        let row = (self.bottom_most_empty_spots[col] + 1) as usize;
        if row >= HEIGHT  || self.board[row][col] == " " {
            return false;
        }
        self.board[row][col] = " ";
        self.bottom_most_empty_spots[col] += 1;
        self.empty_spots_count += 1;
        true
    }

    pub fn available_moves(&self) -> Vec<usize> {
        let mut available_moves = Vec::new();
        let bottom_most_empty_spots_len = self.bottom_most_empty_spots.len();
        let start = bottom_most_empty_spots_len / 2;
        if self.bottom_most_empty_spots[start] > -1 {
            available_moves.push(start);
        }
        for offset in 1..(bottom_most_empty_spots_len / 2)+1 {
            if start - offset < bottom_most_empty_spots_len && self.bottom_most_empty_spots[start - offset] > -1 {
                available_moves.push(start - offset);
            }
            if start + offset < bottom_most_empty_spots_len && self.bottom_most_empty_spots[start + offset] > -1 {
                available_moves.push(start + offset);
            }
        }
        available_moves
    }

    pub fn num_of_empty_spots(&self) -> i32 {
        self.empty_spots_count
    }

    pub fn is_over(&self) -> bool {
        self.is_filled() || self.someone_won()
    }

    pub fn is_filled(&self) -> bool {
        self.empty_spots_count == 0
    }
    
    pub fn someone_won(&self) -> bool {
        if self.last_move[0] == -1 || self.last_move[1] == -1 {
            return false;
        }
        let last_move_row = self.last_move[0] as usize;
        let last_move_col = self.last_move[1] as usize;
        let token = self.board[last_move_row][last_move_col];

        self.won_by_row(last_move_row, last_move_col, token) ||
            self.won_by_col(last_move_row, last_move_col, token) ||
            self.won_by_major_diagonal(last_move_row, last_move_col, token) ||
            self.won_by_minor_diagonal(last_move_row, last_move_col, token)
    }

    fn won_by_row(&self, row: usize, col: usize, token: &str) -> bool {
        self.count_by_direction(row, col, 0, -1, token) + self.count_by_direction(row, col, 0, 1, token) >= 3
    }

    fn won_by_col(&self, row: usize, col: usize, token: &str) -> bool {
        self.count_by_direction(row, col, -1, 0, token) + self.count_by_direction(row, col, 1, 0, token) >= 3
    }

    fn won_by_major_diagonal(&self, row: usize, col: usize, token: &str) -> bool {
        self.count_by_direction(row, col, -1, -1, token) + self.count_by_direction(row, col, 1, 1, token) >= 3
    }
    
    fn won_by_minor_diagonal(&self, row: usize, col: usize, token: &str) -> bool {
        self.count_by_direction(row, col, -1, 1, token) + self.count_by_direction(row, col, 1, -1, token) >= 3
    }

    fn count_by_direction(&self, row: usize, col: usize, row_direction: i32, col_direction: i32, token: &str) -> i32 {
        let row_i32 = row as i32;
        let col_i32 = col as i32;
        
        let mut is_same_count = 0;
        for i in 1..5 {
            let modified_row = row_i32 + (row_direction * i);
            let modified_col = col_i32 + (col_direction * i);
            if modified_row < 0 || modified_row >= HEIGHT as i32 ||
                modified_col < 0 || modified_col >= WIDTH as i32 {
                break;
            }
            if self.board[modified_row as usize][modified_col as usize] == token {
                is_same_count += 1;
            } else {
                break;
            }
        }
        is_same_count
    }


    pub fn print(&self) {
        print!("{}", termion::clear::All);
        print!("{}", termion::cursor::Goto(1, 1));
        println!("{}", self);
    }

}

impl fmt::Display for Board {
    
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut str_board = String::from("");
        for row in 0..self.board.len() {
            for col in 0..self.board[0].len() {
                str_board.push_str(" ");
                if row == self.last_move[0] as usize && col == self.last_move[1] as usize {
                    str_board.push_str(&format!("{}{}", termion::color::Fg(termion::color::Green), self.board[row][col]));
                } else if self.board[row][col] == "X" {
                    str_board.push_str(&format!("{}{}", termion::color::Fg(termion::color::Blue), self.board[row][col]));
                } else if self.board[row][col] == "O" {
                    str_board.push_str(&format!("{}{}", termion::color::Fg(termion::color::Red), self.board[row][col]));
                } else {
                    str_board.push_str(self.board[row][col]);
                }
                str_board.push_str(&format!("{}", termion::style::Reset));
                str_board.push_str(" ");
                if col < self.board[0].len() - 1 {
                    str_board.push_str("|");
                }
            }
            str_board.push_str("\n");
            if row < self.board.len() - 1 {
                str_board.push_str("---------------------------\n");
            }
        }

        writeln!(f, "{}", str_board)
    }    

}
