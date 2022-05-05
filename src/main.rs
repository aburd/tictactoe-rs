use anyhow::Result;
use std::io::{self, BufRead};
use tictactoe::*;

fn main() {
    let mut ttt = Tictactoe::new(3, 3);
    println!("Welcome to TIC TAC TOE");
    println!("*airhorn**airhorn**airhorn**airhorn*");
    println!("");

    loop {
        println!("The current board:");
        println!("{}", ttt);

        if let Some(winning_team) = ttt.winner() {
            println!("The winner is: {:?} team", winning_team);
            break;
        }

        if let Ok(row_n) = get_move(&format!("Player {:?}. Enter the row.", ttt.cur_team)) {
            if let Ok(col_n) = get_move(&format!("Player {:?}. Enter the column.", ttt.cur_team)) {
                if let Err(e) = ttt.play_move(col_n - 1, row_n - 1) {
                    match e {
                        Error::GameOver => println!("The game is already over."),
                        Error::AlreadyPlayed => println!("This space has already been played."),
                    }
                }
            } else {
                println!("Invalid column move");
                continue;
            }
        } else {
            println!("Invalid row move");
            continue;
        }
    }

    println!("Thanks for playing, ya filthy animal.");
}

fn get_move(msg: &str) -> Result<usize> {
    println!("{}", msg);
    let input = get_input()?;
    let m = input.trim().parse::<usize>()?;
    Ok(m)
}

fn get_input() -> Result<String> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle.read_line(&mut buffer)?;
    Ok(buffer)
}
