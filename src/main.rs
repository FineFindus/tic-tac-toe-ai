use std::io::{self, Write};

use board::Board;

use crate::board::PlaceValue;

mod board;

fn main() {
    play_game();
}

fn play_game() {
    let mut board = Board::new();
    let mut turns = 0;

    while !board.is_finished() {
        turns += 1;
        println!("{}", board);
        if turns % 2 == 0 {
            //second player (ai) turn
            board.place_value(read_input(&board), PlaceValue::O);
            continue;
        }
        //first player turn
        board.place_value(read_input(&board), PlaceValue::X);
    }

    println!("{}", board);
    match board.eval_winner() {
        Some(board::PlaceValue::X) => println!("Player X wins"),
        Some(board::PlaceValue::O) => println!("Player O wins"),
        _ => println!("It's a draw"),
    };
}

fn read_input(board: &Board) -> usize {
    print!("Chose a cell [0-8]: ");
    std::io::stdout()
        .lock()
        .flush()
        .expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    match input.trim().parse::<usize>() {
        Ok(cell) if board.get_cell(cell).is_none() => {
            return cell;
        }
        _ => {
            println!("Invalid value. Try again");
            return read_input(board);
        }
    };
}
