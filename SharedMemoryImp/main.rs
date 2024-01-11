use std::io;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

const SIZE: usize = 3;

struct Board {
    buffer: [[char; SIZE]; SIZE],
    game_on: bool,
}

impl Board {
    fn new() -> Board {
        Board {
            buffer: [['-'; SIZE]; SIZE],
            game_on: true,
        }
    }

    fn print_board(&self) {
        println!("\n\n\tTic-Tac-Toe\n");
        for i in 0..SIZE {
            for j in 0..SIZE {
                print!(" {} ", self.buffer[i][j]);
                if j < SIZE - 1 {
                    print!("|");
                }
            }
            if i < SIZE - 1 {
                println!("\n---+---+---");
            } else {
                println!();
            }
        }
        println!();
    }

    fn is_full(&self) -> bool {
        !self.buffer.iter().any(|row| row.iter().any(|&cell| cell == '-'))
    }

    fn is_winner(&self, player: char) -> bool {
        let mut diag1 = true;
        let mut diag2 = true;
        for i in 0..SIZE {
            let mut row = true;
            let mut col = true;
            for j in 0..SIZE {
                row &= self.buffer[i][j] == player;
                col &= self.buffer[j][i] == player;
            }
            diag1 &= self.buffer[i][i] == player;
            diag2 &= self.buffer[i][SIZE - 1 - i] == player;
            if row || col {
                return true;
            }
        }
        diag1 || diag2
    }
}

fn player(board: Arc<Mutex<Board>>, player_char: char, sender: mpsc::Sender<()>, receiver: mpsc::Receiver<()>) {
    while board.lock().unwrap().game_on {
        receiver.recv().unwrap();
        {
            let mut b = board.lock().unwrap();
            if !b.game_on {
                break;
            }

            let mut input = String::new();
            println!("Player {}'s turn. Please enter row and column (0, 1, 2): ", player_char);
            io::stdin().read_line(&mut input).unwrap();
            let row: usize = input.trim().parse().expect("Invalid number");
            input.clear();
            println!("Player {} col: ", player_char);
            io::stdin().read_line(&mut input).unwrap();
            let col: usize = input.trim().parse().expect("Invalid number");

            while (b.buffer[row][col] != '-' || b.is_full()) || row >= SIZE || col >= SIZE {
                println!("Invalid move by player {}; please re-enter", player_char);
                // Re-read row and col
            }

            b.buffer[row][col] = player_char;
            b.print_board();

            if b.is_winner(player_char) {
                println!("\n\t ***** player {} wins! congrats! ***** \n", player_char);
                b.game_on = false;
            } else if b.is_full() {
                println!("\n\t No winner; draw game!\n");
                b.game_on = false;
            }
        }

        sender.send(()).unwrap();
    }
}

fn print_rotating_board(step: usize) {
    let frames = [
        "   |   |   \n   |   |   \n   |   |   ",
        " \\ |   | / \n   |   |   \n / |   | \\ ",
        " \\ | / | / \n   |   |   \n / | \\ | \\ ",
        "   |   |   \n---+---+---\n   |   |   ",
    ];

    // Clear the screen
    print!("\x1B[2J\x1B[1;1H");
    println!("{}", frames[step % frames.len()]);
}

fn main() {
    // Display rotating board animation
    for i in 0..8 {
        print_rotating_board(i);
        thread::sleep(Duration::from_millis(200));
    }

    let board = Arc::new(Mutex::new(Board::new()));
    let (send_p1, recv_p2) = mpsc::channel();
    let (send_p2, recv_p1) = mpsc::channel();

    let send_p1_clone = send_p1.clone();
    let board_p1 = board.clone();
    let board_p2 = board.clone();

    let player1 = thread::spawn(move || {
        player(board_p1, 'X', send_p1, recv_p1);
    });

    let player2 = thread::spawn(move || {
        player(board_p2, 'O', send_p2, recv_p2);
    });

    send_p1_clone.send(()).unwrap();
    player1.join().unwrap();
    player2.join().unwrap();
}
