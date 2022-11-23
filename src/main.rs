fn main() {
    // Play Tic-Tac-Toe
    let mut playing = true;
    let mut play_again = String::new();
    let mut player_turn = "X";
    let mut winner = "";
    let player1 = get_name(1);
    let player2 = get_name(2);
    let mut score1 = 0;
    let mut score2 = 0;
    let mut board = [" ", " ", " ", " ", " ", " ", " ", " ", " "];
    let mut update = 0;
    display_score(player1.clone(), player2.clone(), score1, score2);
    display_board(board);
    while playing {
        // Play turn
        if player_turn == "X" {
            update = play_turn(player1.clone(), board);
        } else {
            update = play_turn(player2.clone(), board);
        }
        // Update and display board
        board[update] = player_turn;
        display_board(board);
        // Check for winner
        winner = get_winner(board);
        if winner != "" {
            if winner == "X" {
                score1 += 1;
                print!("{} Wins!",player1);
            } else if winner == "O" {
                score2 += 1;
                print!("{} Wins!",player2);
            } else {
                score1 += 1;
                score2 += 1;
                print!("It's a Draw!");
            }
            display_score(player1.clone(), player2.clone(), score1, score2);
            play_again = prompt_play_again();
            if play_again == "n" {
                playing = false;
                println!("Thanks for playing!");
            } else {
                winner = "";
                board = [" ", " ", " ", " ", " ", " ", " ", " ", " "];
            }
        }
        // Change player turn
        player_turn = player(player_turn);
        update = 0;
    }
}

// Get player name
fn get_name(n: i32) -> String {
    use std::io::{stdin, stdout, Write};
    let mut player = String::new();
    while player == "" {
        println!("Enter player{} name: ", n);
        let _ = stdout().flush();
        stdin()
            .read_line(&mut player)
            .expect("Please enter a name.");
        // Check for newline
        if let Some('\n') = player.chars().next_back() {
            player.pop();
        }
        // CHeck for carriage return
        if let Some('\r') = player.chars().next_back() {
            player.pop();
        }
    }
    return player;
}

// Display game board
fn display_score(player1: String, player2: String, score1: i32, score2: i32) -> () {
    println!("\n*********************************************************");
    println!("{}:\tScore: {}", player1, score1);
    println!("{}:\tScore: {}", player2, score2);
    println!("***********************************************************\n");
}

// Display game board
fn display_board(board: [&str; 9]) -> () {
    println!(" {} | {} | {}", board[0], board[1], board[2]);
    println!("---+---+---");
    println!(" {} | {} | {}", board[3], board[4], board[5]);
    println!("---+---+---");
    println!(" {} | {} | {}", board[6], board[7], board[8]);
}

// Play a turn
fn play_turn(player: String, board: [&str; 9]) -> usize {
    use std::io::{stdin, stdout, Write};
    let mut player_move = String::new();
    let mut player_moveint = 0;
    // Get move from player
    while player_moveint < 1 || player_moveint > 9 || board[player_moveint - 1] != " " {
        player_move = "".to_string();
        if player_moveint == 0 {
            println!("{}, choose a square(1-9): ", player);
        } else {
            println!("{}, choose an empty square(1-9): ", player);
        }
        let _ = stdout().flush();
        stdin()
            .read_line(&mut player_move)
            .expect("Please enter a square(1-9).");
        // Convert String input to int/i32
        player_moveint = player_move.trim().parse().unwrap();
    }
    return player_moveint - 1;
}

// Change player turn
fn player(player_turn: &str) -> &str {
    if player_turn == "X" {
        return "O";
    } else {
        return "X";
    }
}

// Check for winner
fn get_winner(board: [&str; 9]) -> &str {
    let no_winner = "";
    if (board[0] == "X" && board[1] == "X" && board[2] == "X") || (board[3] == "X" && board[4] == "X" && board[5] == "X") || (board[6] == "X" && board[7] == "X" && board[8] == "X") || (board[0] == "X" && board[3] == "X" && board[6] == "X") || (board[0] == "X" && board[1] == "X" && board[2] == "X") || (board[1] == "X" && board[4] == "X" && board[7] == "X") || (board[2] == "X" && board[5] == "X" && board[8] == "X") || (board[0] == "X" && board[4] == "X" && board[8] == "X") || (board[2] == "X" && board[4] == "X" && board[6] == "X") {
        return "X";
    } else if (board[0] == "O" && board[1] == "O" && board[2] == "O") || (board[3] == "O" && board[4] == "O" && board[5] == "O") || (board[6] == "O" && board[7] == "O" && board[8] == "O") || (board[0] == "O" && board[3] == "O" && board[6] == "O") || (board[0] == "O" && board[1] == "O" && board[2] == "O") || (board[1] == "O" && board[4] == "O" && board[7] == "O") || (board[2] == "O" && board[5] == "O" && board[8] == "O") || (board[0] == "O" && board[4] == "O" && board[8] == "O") || (board[2] == "O" && board[4] == "O" && board[6] == "O") {
        return "O";
    } else {
        for i in 0..board.len() {
        if board[i] == " "{
            return no_winner;
        }
    }
        return "Draw";
    }
}

// Prompt player to play again
fn prompt_play_again() -> String {
    use std::io::{stdin, stdout, Write};
    let mut play_again = String::new();
    // Get player decision to play again
    while play_again.to_lowercase() != "y" && play_again.to_lowercase() != "n" {
        play_again = "".to_string();
        println!("Would you like to play again? (y/n)");
        let _ = stdout().flush();
        stdin()
            .read_line(&mut play_again)
            .expect("Please enter \"y\" or \"n\".");
        // Check for newline
        if let Some('\n') = play_again.chars().next_back() {
            play_again.pop();
        }
        // Check for carriage return
        if let Some('\r') = play_again.chars().next_back() {
            play_again.pop();
        }
    }
    return play_again;
}
