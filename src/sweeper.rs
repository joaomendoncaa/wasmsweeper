use std::{
    collections::HashSet,
    fmt::{Display, Write},
};

use crate::random::random_range;

pub type Position = (usize, usize);

pub enum MineOpeningResult {
    Mine,
    NoMine(u8),
}

#[derive(Debug, Eq, PartialEq)]
pub enum GameState {
    Waiting,
    Playing,
    Over,
}

#[derive(Debug)]
pub struct Sweeper {
    width: usize,
    height: usize,
    fields_open: HashSet<Position>,
    fields_flagged: HashSet<Position>,
    fields_mines: HashSet<Position>,
    game_moves_amount: usize,
    game_state: GameState,
}

impl Display for Sweeper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let position = (x, y);

                let is_field_closed = !self.fields_open.contains(&position);
                let is_field_flagged = self.fields_flagged.contains(&position);
                let is_mine = self.fields_mines.contains(&position);
                let has_lost = self.game_state == GameState::Over;

                if is_field_closed && has_lost && is_mine {
                    f.write_str("mine ")?;
                    continue;
                }

                if is_field_closed && is_field_flagged {
                    f.write_str("flag ")?;
                    continue;
                }

                if is_field_closed {
                    f.write_str("close ")?;
                    continue;
                }

                if is_mine {
                    f.write_str("mine ")?;
                    continue;
                }

                let mine_count = self.get_neightbor_mines(position);

                if mine_count > 0 {
                    write!(f, " {} ", mine_count)?;
                    continue;
                }

                write!(f, " empty ")?;
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
            fields_mines: {
                let mut mines = HashSet::new();

                while mines.len() < mine_count {
                    mines.insert((random_range(0, width, false), random_range(0, width, false)));
                }

                mines
            },
            game_moves_amount: 0,
            game_state: GameState::Waiting,
        };
    }

    pub fn get_neighbor_fields_iter(&self, (x, y): Position) -> impl Iterator<Item = Position> {
        let width = self.width;
        let height = self.height;

        let iter = x.max(1) - 1..=(x + 1).min(width - 1);

        iter.flat_map(move |i| (y.max(1) - 1..=(y + 1).min(height - 1)).map(move |j| (i, j)))
            .filter(move |&position| position != (x, y))
    }

    pub fn get_neightbor_mines(&self, position: Position) -> u8 {
        self.get_neighbor_fields_iter(position)
            .filter(|pos| self.fields_mines.contains(pos))
            .count() as u8
    }

    pub fn open(&mut self, position: Position) -> Option<MineOpeningResult> {
        if self.fields_open.contains(&position) {
            let mine_count = self.get_neightbor_mines(position);
            let flag_count = self
                .get_neighbor_fields_iter(position)
                .filter(|field_position| self.fields_flagged.contains(field_position))
                .count() as u8;

            if mine_count == flag_count {
                for neighbor_position in self.get_neighbor_fields_iter(position) {
                    let is_open = self.fields_open.contains(&neighbor_position);
                    let is_flagged = self.fields_flagged.contains(&neighbor_position);
                    let is_mine = self.fields_mines.contains(&neighbor_position);

                    if !is_flagged && !is_mine && !is_open {
                        self.open(neighbor_position);
                    }
                }
            }

            return None;
        }

        if self.game_state == GameState::Over || self.fields_flagged.contains(&position) {
            return None;
        }

        self.fields_open.insert(position);

        let is_mine: bool = self.fields_mines.contains(&position);

        if is_mine {
            self.game_state = GameState::Over;
            return Some(MineOpeningResult::Mine);
        }

        let mine_count = self.get_neightbor_mines(position);

        if mine_count == 0 {
            for neightbor in self.get_neighbor_fields_iter(position) {
                let is_closed = !self.fields_open.contains(&position);

                if is_closed {
                    self.open(neightbor);
                }
            }
        }

        return Some(MineOpeningResult::NoMine(mine_count));
    }

    pub fn toggle_flag(&mut self, position: Position) {
        if self.game_state == GameState::Over || self.fields_open.contains(&position) {
            return;
        }

        if self.fields_flagged.contains(&position) {
            self.fields_flagged.remove(&position);
            return;
        }

        self.fields_flagged.insert(position);
    }
}
