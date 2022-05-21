mod random;

use std::collections::HashSet;

use random::random_range;

type Position = (usize, usize);

#[derive(Debug)]
struct Sweeper {
    width: usize,
    height: usize,
    fields_open: HashSet<Position>,
    fields_flagged: HashSet<Position>,
    mines: HashSet<Position>,
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
}

#[cfg(test)]
mod tests {
    use crate::Sweeper;

    #[test]
    fn sweeper_instance() {
        let sweeper = Sweeper::new(10, 10, 5);

        println!("Sweeper Instance data: \n{:?}", sweeper);
    }
}
