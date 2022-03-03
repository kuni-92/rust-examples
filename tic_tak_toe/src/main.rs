use std::io::{self, Write};

fn main() {
    let mut board = Vec::new();
    for _ in 0..3 {
        let mut row = Vec::new();
        for _ in 0..3 {
            row.push(String::from("0"));
        }
        board.push(row);
    }

    println!("Game start!!");
    let mut turn = 1;
    for _ in 0..9 {
        clear_display();
        println!("Turn {}", turn);
        disp_board(&board);
        let row = input_row();
        let col = input_column();
        write_board(turn.to_string(), &mut board, row, col);

        if check_board(turn, &board) {
            clear_display();
            println!("Win {} !!!", turn);
            disp_board(&board);
            break;
        }
        if turn == 1 {turn = 2} else {turn = 1}
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

fn write_board(turn: String, board: &mut Vec<Vec<String>>, row: usize, col: usize) {
    board[row][col] = turn;
}

fn input_row() -> usize {
    print!("row>");
    io::stdout().flush().unwrap();
    let mut row = String::new();
    io::stdin().read_line(&mut row).expect("Text Read error.");
    let row = row.replace("\n", "");
    let mut row = row.parse::<usize>().unwrap_or_else(|_| panic!("Row is not number {}", row));
    row -= 1;

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
    let mut col = col.parse::<usize>().unwrap_or_else(|_| panic!("Column is not number {}", col));
    col -= 1;

    if col > 3 {
        panic!("Column number is invalid");
    }
    col
}

fn clear_display() {
    io::stdout().write_all("\u{001b}c".as_bytes()).unwrap();
    io::stdout().flush().unwrap();
}

fn check_board(num: u32, board: &[Vec<String>]) -> bool {
    let check_table = [
        [
            [true,   true,  true],
            [false, false, false],
            [false, false, false],
        ],
        [
            [false, false, false],
            [true,   true,  true],
            [false, false, false],
        ],
        [
            [false, false, false],
            [false, false, false],
            [true,   true,  true],
        ],
        [
            [true, false, false],
            [true, false, false],
            [true, false, false],
        ],
        [
            [false, true, false],
            [false, true, false],
            [false, true, false],
        ],
        [
            [false, false, true],
            [false, false, true],
            [false, false, true],
        ],
        [
            [true,  false, false],
            [false,  true, false],
            [false, false, true],
        ],
        [
            [false, false, true],
            [false,  true, false],
            [true,  false, false],
        ],
    ];

    let mut result = true;
    for checker in check_table.iter() {
        result = true;
        for row in 0..3 {
            for col in 0..3 {
                if checker[row][col] {
                    if num != board[row][col].parse::<u32>().unwrap() {
                        result = false;
                    }
                }
            }
        }

        if result {
            break;
        }
    }
    result
}
