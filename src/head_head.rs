use std::io;

fn main() {
    let mut board = vec![' '; 9];
    let mut current_player = 'X';
    let mut final_move = None;

    loop {
        println!("\n{}", display_board(&board));

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

                    current_player = if current_player == 'X' { 'O' } else { 'X' };
                } else {
                    println!("That position is already occupied. Try again.");
                }
            }
            _ => {
                println!("Invalid input. Please enter a number between 1 and 9.");
            }
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
        board[0], board[1], board[2],
        board[3], board[4], board[5],
        board[6], board[7], board[8]
    )
}

fn get_player_input(_board: &[char], current_player: char) -> String {
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
    for i in 0..3 {
        if (board[i] == player && board[i + 3] == player && board[i + 6] == player)
            || (board[3 * i] == player && board[3 * i + 1] == player && board[3 * i + 2] == player)
        {
            return true;
        }
    }
    (board[0] == player && board[4] == player && board[8] == player)
        || (board[2] == player && board[4] == player && board[6] == player)
}

