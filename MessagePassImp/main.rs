use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::io::{self, Write};

const SIZE: usize = 3;

fn main() {
    let (tx1, rx1) = mpsc::channel(); 
    let (tx2, rx2) = mpsc::channel(); 

    let board: [[char; SIZE]; SIZE] = [['-'; SIZE]; SIZE]; 

    
    let player1 = thread::spawn(move || {
        let mut board = board;
        for _ in 0..SIZE * SIZE / 2 + SIZE % 2 { 
            print_board(&board);
            let (row, col) = get_player_move('X');
            board[row][col] = 'X'; 
            tx1.send(board).unwrap(); 

            if let Ok(new_board) = rx2.recv() { 
                board = new_board;
                let winner = check_winner(&board);
                if winner != 'C' {
                    announce_winner(winner);
                    break;
                }
            }
        }
    });

    
    let player2 = thread::spawn(move || {
        let mut board = board;
        for _ in 0..SIZE * SIZE / 2 { 
            if let Ok(new_board) = rx1.recv() { 
                board = new_board;
                print_board(&board);
                let winner = check_winner(&board);
                if winner != 'C' {
                    announce_winner(winner);
                    break;
                }
                let (row, col) = get_player_move('O');
                board[row][col] = 'O'; 
                tx2.send(board).unwrap(); 
            }
        }
    });

    player1.join().unwrap();
    player2.join().unwrap();

    println!("Final board state:");
    print_board(&board);
}

fn get_player_move(player: char) -> (usize, usize) {
    println!("Player {}, enter your move (row column): ", player);
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let parts: Vec<usize> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
    (parts[0], parts[1])
}

fn print_board(board: &[[char; SIZE]; SIZE]) {
    println!("\nCurrent board:");
    for row in board.iter() {
        for &cell in row.iter() {
            print!("{} ", cell);
        }
        println!();
    }
}

fn check_winner(board: &[[char; SIZE]; SIZE]) -> char {
    for i in 0..SIZE {
        if board[i][0] != '-' && board[i].iter().all(|&x| x == board[i][0]) {
            return board[i][0]; 
        }
        if board[0][i] != '-' && (0..SIZE).all(|j| board[j][i] == board[0][i]) {
            return board[0][i]; 
        }
    }

    if board[0][0] != '-' && (0..SIZE).all(|i| board[i][i] == board[0][0]) {
        return board[0][0]; 
    }
    if board[0][SIZE-1] != '-' && (0..SIZE).all(|i| board[i][SIZE-i-1] == board[0][SIZE-1]) {
        return board[0][SIZE-1]; 
    }

    if board.iter().all(|row| row.iter().all(|&cell| cell != '-')) {
        return 'D'; 
    }

    'C'
}

fn announce_winner(winner: char) {
    match winner {
        'X' => println!("Player X wins!"),
        'O' => println!("Player O wins!"),
        'D' => println!("It's a draw!"),
        _ => (),
    }
}

