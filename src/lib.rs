mod random;

use std::{
    collections::HashSet,
    fmt::{Display, Write},
};

use random::random_range;

pub type Position = (usize, usize);

pub enum MineOpeningResult {
    Mine,
    NoMine(u8),
}

#[derive(Debug)]
pub struct Sweeper {
    width: usize,
    height: usize,
    fields_open: HashSet<Position>,
    fields_flagged: HashSet<Position>,
    mines: HashSet<Position>,
}

impl Display for Sweeper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let position = (x, y);

                if !self.fields_open.contains(&position) {
                    f.write_str("⬛ ")?;
                } else if self.mines.contains(&position) {
                    f.write_str("💣 ")?;
                } else if self.fields_flagged.contains(&position) {
                    f.write_str("🚩 ")?;
                } else {
                    write!(f, " {} ", self.get_neightbors_mines(position))?;
                }
            }

            f.write_char('\n')?;
        }

        Ok(())
    }
}

impl Sweeper {
    pub fn new(width: usize, height: usize, mine_count: usize) -> Sweeper {
        return Sweeper {
            width,
            height,
            fields_flagged: HashSet::new(),
            fields_open: HashSet::new(),
            mines: {
                let mut mines = HashSet::new();

                while mines.len() < mine_count {
                    mines.insert((random_range(0, width, false), random_range(0, width, false)));
                }

                mines
            },
        };
    }

    pub fn get_neighbors_fields_iter(&self, (x, y): Position) -> impl Iterator<Item = Position> {
        let width = self.width;
        let height = self.height;

        let iter = x.max(1) - 1..=(x + 1).min(width - 1);

        iter.flat_map(move |i| (y.max(1) - 1..=(y + 1).min(height - 1)).map(move |j| (i, j)))
            .filter(move |&position| position != (x, y))
    }

    pub fn get_neightbors_mines(&self, position: Position) -> u8 {
        self.get_neighbors_fields_iter(position)
            .filter(|pos| self.mines.contains(pos))
            .count() as u8
    }

    pub fn open(&mut self, position: Position) -> Option<MineOpeningResult> {
        if self.fields_flagged.contains(&position) {
            return None;
        }

        self.fields_open.insert(position);

        let is_mine: bool = self.mines.contains(&position);

        if is_mine {
            return Some(MineOpeningResult::Mine);
        }

        return Some(MineOpeningResult::NoMine(0));
    }

    pub fn toggle_flag(&mut self, position: Position) {
        if self.fields_open.contains(&position) {
            return;
        }

        if self.fields_flagged.contains(&position) {
            self.fields_flagged.remove(&position);
        }

        self.fields_flagged.insert(position);
    }
}

#[cfg(test)]
mod tests {
    use crate::{random::random_range, Sweeper};

    #[test]
    fn sweeper_instance() {
        let mut sweeper = Sweeper::new(10, 10, 2);

        sweeper.open((random_range(0, 10, true), random_range(0, 10, true)));
        sweeper.open((random_range(0, 10, true), random_range(0, 10, true)));
        sweeper.open((random_range(0, 10, true), random_range(0, 10, true)));

        sweeper.toggle_flag((random_range(0, 10, true), random_range(0, 10, true)));

        println!("{}", sweeper);
    }
}
