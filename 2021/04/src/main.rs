#[derive(Copy, Clone, Debug)]
struct BingoCell {
    number: u32,
    marked: bool,
}

// impl std::fmt::Display for BingoCell {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         write!(f, "{}", self.number)
//     }
// }

fn main() {
    let file = std::fs::read_to_string("input.txt").unwrap();
    let input = file
        .lines()
        .collect::<Vec<&str>>();

    let numbers = input[0].split(",").map(|x| x.parse::<u32>().unwrap()).collect::<Vec<_>>();
    let mut bingo_boards: Vec<[[BingoCell; 5]; 5]> = vec![];

    let mut n = 2;
    while n < input.len() {
        let mut bingo_board = [[BingoCell { number: 0, marked: false }; 5]; 5];
        for (i, line) in input[n..n+5].iter().enumerate() {
            for (j, number) in line.split_whitespace().enumerate() {
                bingo_board[i][j] = BingoCell {
                    number: number.parse::<u32>().unwrap(),
                    marked: false,
                };
            }
        }
        bingo_boards.push(bingo_board);
        n += 6;
    }

    let mut bingo_boards_p2 = bingo_boards.clone();

    {
        let mut end = (0, 0);
        'main: for x in numbers.clone() {
            for (board_n, bingo_board) in bingo_boards.iter_mut().enumerate() {
                let mut col_marked = true;
                for (i, bingo_row) in bingo_board.iter_mut().enumerate() {
                    let mut row_marked = true;
                    for (j, bingo_cell) in bingo_row.iter_mut().enumerate() {
                        if bingo_cell.number == x {
                            bingo_cell.marked = true;
                        } else if !bingo_cell.marked {
                            row_marked = false;
                            if i == j {
                                col_marked = false;
                            }
                        }
                    }
                    if row_marked {
                        end = (x, board_n);
                        break 'main;
                    }
                }
                if col_marked {
                    end = (x, board_n);
                    break 'main;
                }
            }
        }

        let (last, winner) = end;
        let mut sum = 0;
        for bingo_row in bingo_boards[winner] {
            for bingo_cell in bingo_row {
                if !bingo_cell.marked {
                    sum += bingo_cell.number;
                }
            }
        }

        println!("Part 1: {}", sum * last);
    }

    {
        let mut end = (0, 0);
        let mut boards_marks = vec![];
        for _ in bingo_boards.iter().enumerate() {
            boards_marks.push(false);
        }

        for x in numbers {
            'boards: for (board_n, bingo_board) in bingo_boards_p2.iter_mut().enumerate() {
                if boards_marks[board_n] {
                    continue 'boards;
                }

                let mut col_marked: i32 = -1;
                for (i, bingo_row) in bingo_board.iter_mut().enumerate() {
                    let mut row_marked = true;
                    for (j, bingo_cell) in bingo_row.iter_mut().enumerate() {
                        if bingo_cell.number == x {
                            bingo_cell.marked = true;
                            col_marked = j as i32;
                        } else if !bingo_cell.marked {
                            row_marked = false;
                        }
                    }
                    if row_marked {
                        end = (x, board_n);
                        boards_marks[board_n] = true;
                    }
                }
                let mut col_marked_bool = true;
                if col_marked >= 0 {
                    for bingo_row in bingo_board.iter_mut() {
                        if !bingo_row[col_marked as usize].marked {
                            col_marked_bool = false;
                        }
                    }
                }
                if col_marked_bool && col_marked >= 0 {
                    end = (x, board_n);
                    boards_marks[board_n] = true;
                }
            }
        }

        let (last, winner) = end;
        let mut sum = 0;
        for bingo_row in bingo_boards_p2[winner] {
            for bingo_cell in bingo_row {
                if !bingo_cell.marked {
                    sum += bingo_cell.number;
                }
            }
        }

        println!("Part 2: {}", sum * last)
    }
}
