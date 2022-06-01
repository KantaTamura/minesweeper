mod random;

use std::collections::HashSet;
use random::random_range;

pub type Position = (usize, usize);

pub enum OpenResult {
    Mine,
    NoMine(u8)
}

#[derive(Debug)]
pub struct Minesweeper {
    width: usize,
    height: usize,
    open_fields: HashSet<Position>,
    mines: HashSet<Position>,
    flagged_fields: HashSet<Position>,
}

impl Minesweeper {
    pub fn new(width: usize, height: usize, mine_count: usize) -> Minesweeper {
        Minesweeper {
            width,
            height,
            open_fields: HashSet::new(),
            mines: {
                let mut mines = HashSet::new();

                while mines.len() < mine_count {
                    mines.insert((random_range(0, width), random_range(0, height)));
                }

                mines
            },
            flagged_fields: HashSet::new(),
        }
    }

    pub fn neighbors(&self, (x, y): Position) -> Vec<Position> {
        (x.max(1) - 1 ..= (x + 1).min(self.width - 1))
            .flat_map(|i| (y.max(1) - 1 ..= (y + 1).min(self.height - 1)).map(move |j| (i, j)))
            .collect()
    }

    pub fn open(&mut self, position: Position) -> OpenResult {
        self.open_fields.insert(position);

        let is_mine = self.mines.contains(&position);

        if is_mine {
            OpenResult::Mine
        } else {
            OpenResult::NoMine(0)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Minesweeper;

    #[test]
    fn test() {
        let ms = Minesweeper::new(10, 10, 5);

        println!("{:?}", ms);
    }

    #[test]
    fn neighbors_test() {
        let ms = Minesweeper::new(10, 10, 5);
        let test_case_position: Vec<(usize, usize)> = (0 .. 11).flat_map(|i| (0 .. 11).map(move |j| (i, j))).collect();

        println!("{:?}", ms);

        for p in test_case_position {
            let buf = ms.neighbors(p);
            println!("{:?}", p);
            println!("{:?}", buf);
        }
    }
}
