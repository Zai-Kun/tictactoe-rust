use std::io::{self, Write};
use std::process::Command;

fn clear_terminal() {
    #[cfg(target_os = "windows")]
    {
        Command::new("cmd")
            .args(&["/C", "cls"])
            .status()
            .except("error")
    }

    #[cfg(not(target_os = "windows"))]
    {
        print!("\x1B[2J\x1B[1;1H");
        std::io::stdout().flush().expect("error while flushing");
    }
}

enum GameOver {
    Winner(char),
    Draw,
    OnGoing,
}

#[derive(Debug)]
struct TicTacToe {
    player_1: char,
    player_2: char,
    board: Vec<char>,
}

impl TicTacToe {
    fn new() -> Self {
        let player_1 = 'X';
        let player_2 = 'O';
        let board = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8'];
        Self {
            player_1,
            player_2,
            board,
        }
    }

    fn print_board(&self) {
        for i in 0..3 {
            if i > 0 {
                println!("---------");
            }
            println!(
                "{} | {} | {}",
                self.board[3 * i],
                self.board[3 * i + 1],
                self.board[3 * i + 2]
            );
        }
    }

    fn turn_to_move(&self) -> char {
        let total_instences_player_1: u8 = self
            .board
            .iter()
            .map(|pos| if pos == &self.player_1 { 1 } else { 0 })
            .sum();
        let total_instences_player_2: u8 = self
            .board
            .iter()
            .map(|pos| if pos == &self.player_2 { 1 } else { 0 })
            .sum();
        if total_instences_player_1 <= total_instences_player_2 {
            self.player_1
        } else {
            self.player_2
        }
    }

    fn make_move(&mut self, mv: usize) {
        let turn = self.turn_to_move();
        self.board[mv] = turn;
    }

    fn undo_move(&mut self, mv: usize) {
        self.board[mv] = std::char::from_digit(mv as u32, 10).unwrap();
    }

    fn is_move_valid(&self, mv: usize) -> bool {
        if mv >= self.board.len() {
            return false;
        }

        if self.board[mv] != self.player_1 && self.board[mv] != self.player_2 {
            true
        } else {
            false
        }
    }

    fn game_over(&self) -> GameOver {
        let winning_positions = [
            [0, 1, 2],
            [3, 4, 5],
            [6, 7, 8],
            [0, 3, 6],
            [1, 4, 7],
            [2, 5, 8],
            [0, 4, 8],
            [2, 4, 6],
        ];
        for player in [self.player_1, self.player_2] {
            for winning_position in winning_positions {
                if player == self.board[winning_position[0]]
                    && self.board[winning_position[0]] == self.board[winning_position[1]]
                    && self.board[winning_position[1]] == self.board[winning_position[2]]
                {
                    return GameOver::Winner(player);
                };
            }
        }

        for pos in self.board.iter() {
            if *pos != self.player_1 && *pos != self.player_2 {
                return GameOver::OnGoing;
            }
        }
        GameOver::Draw
    }

    fn evaluate(&self) -> i8 {
        match self.game_over() {
            GameOver::Draw => 0,
            GameOver::Winner(player) => {
                if player == self.player_1 {
                    1
                } else {
                    -1
                }
            }
            GameOver::OnGoing => panic!("why the hell did you call me when the game was on going?"),
        }
    }

    fn get_all_moves(&self) -> Vec<usize> {
        (0..self.board.len())
            .filter(|pos| self.board[*pos] != self.player_1 && self.board[*pos] != self.player_2)
            .collect()
    }

    fn minimax(&mut self, mut alpha: i8, mut beta: i8) -> i8 {
        match self.game_over() {
            GameOver::OnGoing => {}
            _ => {
                return self.evaluate();
            }
        }

        let maximizing = self.turn_to_move() == self.player_1;
        if maximizing {
            let mut min_eval: i32 = std::i32::MIN;
            for pos in self.get_all_moves() {
                self.make_move(pos);
                let eval = self.minimax(alpha, beta);
                self.undo_move(pos);

                min_eval = std::cmp::max(min_eval, eval as i32);
                alpha = std::cmp::max(alpha, eval);
                if beta <= alpha {
                    break;
                }
            }
            return min_eval as i8;
        } else {
            let mut max_eval: i32 = std::i32::MAX;
            for pos in self.get_all_moves() {
                self.make_move(pos);
                let eval = self.minimax(alpha, beta);
                self.undo_move(pos);

                max_eval = std::cmp::min(max_eval, eval as i32);
                beta = std::cmp::min(beta, eval);
                if beta <= alpha {
                    break;
                }
            }
            return max_eval as i8;
        }
    }

    fn best_move(&mut self) -> i8 {
        let maximizing = self.turn_to_move() == self.player_1;
        let mut evaluations_of_moves: Vec<Vec<i8>> = Vec::new();
        for pos in self.get_all_moves() {
            self.make_move(pos);
            evaluations_of_moves.push(vec![pos as i8, self.minimax(std::i8::MIN, std::i8::MAX)]);
            self.undo_move(pos)
        }
        let best_move = if maximizing {
            evaluations_of_moves.iter().max_by_key(|sub_vec| sub_vec[1])
        } else {
            evaluations_of_moves.iter().min_by_key(|sub_vec| sub_vec[1])
        };

        match best_move {
            Some(sub_vec) => sub_vec[0],
            None => -1,
        }
    }
}

fn main() {
    clear_terminal();
    println!("Welcome to the Simpel TicTacToe game");
    loop {
        println!("1. Human vs Human\n2. Human vs Computer\n3. Exit");
        let user_input = input("Pick an option (1, 2, 3): ");
        match user_input.as_str() {
            "1" => start_game(false),
            "2" => start_game(true),
            "3" => break,
            _ => println!("Invalid option, please pick a valid option."),
        }
    }

    println!("Thanks for playing, cya")
}

fn start_game(vs_computer: bool) {
    clear_terminal();
    let mut tictactoe = TicTacToe::new();
    loop {
        tictactoe.print_board();
        match tictactoe.game_over() {
            GameOver::Draw => {
                println!("Draw!");
                break;
            }
            GameOver::Winner(player) => {
                println!("Player {} has won!", player);
                break;
            }
            _ => {}
        }
        if tictactoe.turn_to_move() == tictactoe.player_1 || !vs_computer {
            let user_input =
                input(&format!("\n{}'s turn: ", tictactoe.turn_to_move())).parse::<usize>();
            if let Ok(value) = user_input {
                if tictactoe.is_move_valid(value) {
                    tictactoe.make_move(value);
                    clear_terminal();
                    continue;
                }
            }
        } else {
            let mv = tictactoe.best_move();
            tictactoe.make_move(mv as usize);
            clear_terminal();
            continue;
        }
        clear_terminal();
        println!("Invalid number!");
    }
}

fn input(msg: &str) -> String {
    print!("{}", msg);
    io::stdout().flush().expect("error flushing");

    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("error reading stdin");
    user_input.trim().to_string()
}
