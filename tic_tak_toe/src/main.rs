use std::io::{self, Write};

fn main() {
    let mut board = Vec::new();
    for _ in 0..2 {
        board.push(Vec::new());
    }
    for col in 0..2 {
        board[col].push(String::from("　"));
    }

    println!("Game start!!");
    loop {
        clear_display();
        disp_board(&board);
        let row = input_row();
        let col = input_column();
        write_board(&mut board, row, col);
    }
}

fn disp_board(board: &Vec<Vec<String>>) {
    for row in board {
        for cell in row {
            println!("{}", cell);
        }
    }
}

fn write_board(board: &mut Vec<Vec<String>>, row: usize, col: usize) {
    board[row][col] = "○".to_string();
}

fn input_row() -> usize {
    print!("row>");
    let mut row = String::new();
    io::stdin().read_line(&mut row).expect("Text Read error.");

    let row :usize = row.parse().expect("Row is not number");

    if row > 3 {
        panic!("Row number is invalid");
    }
    row
}

fn input_column() -> usize {
    print!("col>");
    let mut col = String::new();
    io::stdin().read_line(&mut col).expect("Text Read error.");

    let col :usize = col.parse().expect("Column is not number");

    if col > 3 {
        panic!("Column number is invalid");
    }
    col
}

fn clear_display() {
    io::stdout().write("\u{001b}c".as_bytes()).unwrap();
}
