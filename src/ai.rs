extern crate termion;

use board;
use board::Board;
use player;
use utils;

use std;

pub const TYPE: &str = "Ai";

pub struct Ai {
    token: &'static str,
    player_type: &'static str,
}

impl Ai {

    fn minimax(&self, board: &mut Board, turn: i32, alpha: i32, beta: i32, depth: usize, search_limit: usize) -> (i32, i32) {
        if board.is_over() || depth > search_limit {
            return self.evaluate(board, depth, turn);
        }
            
        let available_moves = board.available_moves();
        let mut moves = Vec::new();
        let mut scores = Vec::new();

        let mut alpha = alpha;
        let mut beta = beta;
        let mut best_move = -1;
        for col in &available_moves {
            if turn == 0 {
                board.put(*col, self.token);
            } else {
                board.put(*col, utils::get_other_token(self.token));
            }
            moves.push(col);
            let (_best_move, score) = self.minimax(board, 1-turn, alpha, beta, depth+1, search_limit);
            scores.push(score);
            if turn == 0 {
                let (best_score, index) = self.get_best_move(&scores, 0);
                best_move = *moves[index] as i32;
                if best_score > alpha {
                    alpha = best_score;
                }
            } else {
                let (best_score, _index) = self.get_best_move(&scores, 1);
                if best_score < beta {
                    beta = best_score;
                }
            }
            board.remove(*col);
            if alpha >= beta {
                break;
            }

            // if score > alpha_copy && turn == 0 {
            //     alpha_copy = score;
            // } else if score < beta_copy && turn == 1 {
            //     beta_copy = score;
            // }
            // scores.push(score);
            // board.remove(*col);
            // if alpha_copy >= beta_copy {
            //     break;
            // }
        }
        // let (best_score, index) = self.get_best_move(scores, 0);
        if turn == 0 {
            return (best_move, alpha);
        } else {
            return (best_move, beta);
        }
    }

    fn evaluate(&self, board: &mut board::Board, depth: usize, turn: i32) -> (i32, i32) {
        if board.someone_won() {
            if turn == 0 {
                return (-1, depth as i32 - std::i32::MAX);
            } else {
                return (-1, std::i32::MAX - depth as i32);
            }
        }
        if board.is_filled() {
            return (-1, 0);
        }
        (-1, self.linked_row_strength(board, turn)) 
    }

    fn get_length(&self, position: [i32; 2], dir_offset: [i32; 2], token: &str, visited: &mut [[bool; 7]; 6], count: i32, board: &board::Board) -> i32 {
        let mut count = count;
        let mut row = (position[0] + count * dir_offset[0]) as usize;
        let mut col = (position[1] + count * dir_offset[1]) as usize;
        while board.get(row, col) == token {
            visited[row][col] = true;
            count += 1;
            row = ((row as i32) + dir_offset[0]) as usize;
            col = ((col as i32) + dir_offset[1]) as usize;
        }
        count
    }

    fn get_length_pair(&self, position: [i32; 2], dir_offset: [i32; 2], token: &str, visited: &mut [[bool; 7]; 6], board: &board::Board) -> [i32; 2] {
        let position = [position[0] + dir_offset[0], position[1] + dir_offset[1]];
        let player_length = self.get_length(position, dir_offset, token, visited, 0, board);
        let possible_length = self.get_length(position, dir_offset, " ", visited, player_length, board);
        [player_length, possible_length]
    }

    fn get_rating(&self, position: [i32; 2], token: &str, visited: &mut [[bool; 7]; 6], directions: &[[i32; 4]; 2], board: &board::Board, turn: i32) -> i32 {
        let mut rating = 0;
        let line_weights = [0, 10, 100];
        for i in 0..4 {
            let pos_direction = [directions[0][i], directions[1][i]];
            let pos_dir_score = self.get_length_pair(position, pos_direction, token, visited, board);
            let neg_direction = [-directions[0][i], -directions[1][i]];
            let neg_dir_score = self.get_length_pair(position, neg_direction, token, visited, board);
            if pos_dir_score[1] + neg_dir_score[1] >= 3 {
                rating += -turn * line_weights[(pos_dir_score[0] + neg_dir_score[0]) as usize];
            }
        }
        rating
    }

    fn linked_row_strength(&self, board: &board::Board, turn: i32) -> i32 {
        let mut total = 0;
        let directions = [[1, 1, 0, -1], [0, 1, 1, 1]];
        let mut visited = [[false, false, false, false, false, false, false],
        [false, false, false, false, false, false, false],
        [false, false, false, false, false, false, false],
        [false, false, false, false, false, false, false],
        [false, false, false, false, false, false, false],
        [false, false, false, false, false, false, false]];
        for row in 0..board::HEIGHT {
            for col in 0..board::WIDTH {
                if visited[row][col] {
                    continue;
                }
                let token = board.get(row, col);
                if token != " " {
                    total += self.get_rating([row as i32, col as i32], token, &mut visited, &directions, board, turn);
                }
                visited[row][col] = true;
            }
        }
        total
    }

    fn get_best_move(&self, scores: &Vec<i32>, turn: i32) -> (i32, usize) {
        if turn == 0 {
            let max_index = self.get_max_index(scores);
            return (scores[max_index], max_index);
        } else {
            let min_index = self.get_min_index(scores);
            return (scores[min_index], min_index);
        }
    }

    fn get_max_index(&self, vec: &Vec<i32>) -> usize {
        let mut max_index = 0;
        for index in 1..vec.len() {
            if vec[index] > vec[max_index] {
                max_index = index;
            }
        }
        max_index
    }

    fn get_min_index(&self, vec: &Vec<i32>) -> usize {
        let mut min_index = 0;
        for index in 1..vec.len() {
            if vec[index] < vec[min_index] {
                min_index = index;
            }
        }
        min_index
    }

}

impl player::Player for Ai {

    fn new(token: &'static str) -> Ai {
        Ai {
            token: token,
            player_type: TYPE,
        }
    }

    fn get_token(&self) -> &'static str {
        self.token
    }

    fn get_type(&self) -> &'static str {
        self.player_type
    }

    fn play_move(&self, board: &mut board::Board) -> bool {
        println!("{}Please wait a moment while I think about my next move...", termion::color::Fg(termion::color::Green));
        if board.num_of_empty_spots() > 27 {
            let (best_move, _score) = self.minimax(board, 0, std::i32::MIN, std::i32::MAX, 1, 10);
            return board.put(best_move as usize, self.token);
        } else {
            let (best_move, _score) = self.minimax(board, 0, std::i32::MIN, std::i32::MAX, 1, 15);
            return board.put(best_move as usize, self.token);
        }
    }

}

