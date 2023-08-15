use std::io;
use rand::Rng;

fn main() {
    let mut board = vec![' '; 9]; // Initialize the game board
    let mut current_player = 'X';
    let mut final_move = None;

    loop {
        if final_move.is_none() {
            println!("\n{}", display_board(&board));
        }

        if current_player == 'X' {
            let player_input = get_player_input(&board, current_player);
            let position = player_input.trim().parse::<usize>();

            match position {
                Ok(pos) if pos >= 1 && pos <= 9 => {
                    let index = pos - 1;
                    if board[index] == ' ' {
                        board[index] = current_player;
                        if check_winner(&board, current_player) {
                            final_move = Some(pos);
                        } else if board.iter().all(|&cell| cell != ' ') {
                            final_move = Some(pos);
                        }
                        current_player = 'O';
                    } else {
                        println!("That position is already occupied. Try again.");
                    }
                }
                _ => println!("Invalid input. Please enter a number between 1 and 9."),
            }
        } else {
            // Computer's turn
            let computer_move = get_computer_move(&board);
            board[computer_move] = current_player;
            if check_winner(&board, current_player) {
                final_move = Some(computer_move + 1);
            } else if board.iter().all(|&cell| cell != ' ') {
                final_move = Some(computer_move + 1);
            }
            current_player = 'X';
        }

        if let Some(move_pos) = final_move {
            println!("\n{}", display_board(&board));
            if check_winner(&board, 'X') {
                println!("Player X wins! Final move: {}", move_pos);
            } else if check_winner(&board, 'O') {
                println!("Player O wins! Final move: {}", move_pos);
            } else {
                println!("It's a draw! Final move: {}", move_pos);
            }
            break;
        }
    }
}

fn display_board(board: &[char]) -> String {
    format!(
        "
 {} | {} | {}
---|---|---
 {} | {} | {}
---|---|---
 {} | {} | {}
",
        board[0], board[1], board[2], board[3], board[4], board[5], board[6], board[7], board[8]
    )
}

fn get_player_input(board: &[char], current_player: char) -> String {
    loop {
        let mut input = String::new();
        println!("\n{}'s turn. Enter a position (1-9):", current_player);
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input.");
        if input.trim().is_empty() {
            continue;
        }
        return input;
    }
}

fn check_winner(board: &[char], player: char) -> bool {
    // Check rows, columns, and diagonals for a win
    for i in 0..3 {
        if (board[i] == player && board[i + 3] == player && board[i + 6] == player)
            || (board[3 * i] == player && board[3 * i + 1] == player && board[3 * i + 2] == player)
        {
            return true;
        }
    }

    if (board[0] == player && board[4] == player && board[8] == player)
        || (board[2] == player && board[4] == player && board[6] == player)
    {
        return true;
    }

    false
}

fn get_computer_move(board: &[char]) -> usize {
    let mut empty_cells: Vec<usize> = Vec::new();
    for (index, &cell) in board.iter().enumerate() {
        if cell == ' ' {
            empty_cells.push(index);
        }
    }
    
    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..empty_cells.len());
    empty_cells[random_index]
}
