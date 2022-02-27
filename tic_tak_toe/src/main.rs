use std::io::{self, Write};

fn main() {
    let mut board = Vec::new();
    for _ in 0..3 {
        let mut row = Vec::new();
        for _ in 0..3 {
            row.push(String::from("*"));
        }
        board.push(row);
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

fn disp_board(board: &[Vec<String>]) {
    let mut disp = String::new();

    for row in board.iter() {
        for cell in row.iter() {
            disp = disp.clone() + cell;
        }
        disp = disp.clone() + "\n";
    }
    print!("{}", disp);
}

fn write_board(board: &mut Vec<Vec<String>>, row: usize, col: usize) {
    board[row][col] = "1".to_string();
}

fn input_row() -> usize {
    print!("row>");
    io::stdout().flush().unwrap();
    let mut row = String::new();
    io::stdin().read_line(&mut row).expect("Text Read error.");
    let row = row.replace("\n", "");
    let row = row.parse::<usize>().unwrap_or_else(|_| panic!("Row is not number {}", row));

    if row > 3 {
        panic!("Row number is invalid");
    }
    row
}

fn input_column() -> usize {
    print!("col>");
    io::stdout().flush().unwrap();
    let mut col = String::new();
    io::stdin().read_line(&mut col).expect("Text Read error.");
    let col = col.replace("\n", "");
    let col = col.parse::<usize>().unwrap_or_else(|_| panic!("Column is not number {}", col));

    if col > 3 {
        panic!("Column number is invalid");
    }
    col
}

fn clear_display() {
    io::stdout().write_all("\u{001b}c".as_bytes()).unwrap();
    io::stdout().flush().unwrap();
}
