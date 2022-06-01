mod random;

use random::random_range;
use std::collections::HashSet;

pub type Position = (usize, usize);

pub enum OpenResult {
    Mine,
    NoMine(u8),
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

    pub fn iter_neighbors(&self, (x, y): Position) -> impl Iterator<Item = Position> {
        let width = self.width;
        let height = self.height;
        (x.max(1) - 1..=(x + 1).min(width - 1))
            .flat_map(move |i| (y.max(1) - 1..=(y + 1).min(height - 1)).map(move |j| (i, j)))
            .filter(move |&pos| pos != (x, y))
    }

    pub fn neighboring_mines(&self, pos: Position) -> u8 {
        self.iter_neighbors(pos)
            .filter(|pos| self.mines.contains(pos))
            .count() as u8
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
}
