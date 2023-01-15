use std::fs::File;

use std::io::Read;
use std::{env, vec};

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[derive(Debug)]
struct Life {
    size: Vec<usize>,
    field: Vec<Vec<Cell>>,
}

impl Life {
    fn add(first: usize, second: isize) -> Option<usize> {
        if second.is_negative() {
            first.checked_sub(second.wrapping_abs() as u32 as usize)
        } else {
            first.checked_add(second as usize)
        }
    }

    fn new(file: impl Into<String>) -> Self {
        let s: String = file.into();
        let mut file = File::open(s).expect("Unable to read file");
        let mut field = String::new();
        let _ = file.read_to_string(&mut field).unwrap();

        let parsed_field: Vec<Vec<Cell>> = field
            .lines()
            .skip(1)
            .map(|row| {
                row.chars()
                    .map(|c| {
                        return if c == '#' { Cell::Alive } else { Cell::Dead };
                    })
                    .collect()
            })
            .collect();
        Self {
            size: vec![parsed_field.len(), parsed_field[0].len()],
            field: parsed_field,
        }
    }

    fn live_neighbor_count(&self, row: usize, col: usize) -> u8 {
        let mut count = 0;

        for i in [-1, 0, 1] {
            for j in [-1, 0, 1] {
                let mut idx1 = Self::add(row, i).or(Some(0)).unwrap();
                let mut idx2 = Self::add(col, j).or(Some(0)).unwrap();
                if idx1 >= self.size[0] {
                    idx1 = row;
                }
                if idx2 >= self.size[1] {
                    idx2 = col;
                }
                if idx1 == row && idx2 == col {
                    continue;
                }
                count += self.field[idx1][idx2] as u8;
            }
        }

        count
    }

    fn next(&mut self) {
        self.field = self
            .field
            .iter()
            .enumerate()
            .map(|(row_idx, row)| {
                row.iter()
                    .enumerate()
                    .map(|(col, cell)| {
                        let alives = self.live_neighbor_count(row_idx, col);
                        return match cell {
                            Cell::Dead => {
                                if alives == 3 {
                                    Cell::Alive
                                } else {
                                    Cell::Dead
                                }
                            }
                            Cell::Alive => {
                                if alives > 1 && alives < 4 {
                                    Cell::Alive
                                } else {
                                    Cell::Dead
                                }
                            }
                        };
                    })
                    .collect()
            })
            .collect();
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut filename = String::new();
    if args.len() <= 1 {
        filename += "input.txt";
    } else {
        filename += args[1].as_str();
    }

    let mut life = Life::new(filename);
    for _ in 0..20 {
        life.next();
        for i in 0..life.size[0] {
            for j in 0..life.size[1] {
                print!(
                    "{}",
                    if life.field[i][j] == Cell::Alive {
                        '#'
                    } else {
                        '.'
                    }
                );
            }
            println!();
        }
        println!();
    }
}
