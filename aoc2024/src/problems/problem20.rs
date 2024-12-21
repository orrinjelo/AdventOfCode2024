use std::fmt;
use itertools::structs;
use log::{trace, debug, info, warn, error}; // trace, debug, info, warn, error
use crate::util::RetType;

#[derive(Clone, Debug, PartialEq)]
struct Chtron {
    maze: Vec<Vec<u8>>,
    start: (usize, usize),
    end: (usize, usize),
    cheated: bool,
    pc: (usize, usize),
    path_count: u32,
}

impl fmt::Display for Chtron {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let _ = write!(f, "\n");
        for y in 0..self.maze.len() {
            for x in 0..self.maze[0].len() {
                match &self.maze[y][x] {
                    0 => { // Space
                        let _ = write!(f, ".");
                    },
                    1 => { // Wall
                        let _ = write!(f, "#");
                    },
                    2 => { // Start
                        let _ = write!(f, "S");
                    },
                    3 => { // End
                        let _ = write!(f, "E");
                    },
                    4 => { // Traversed
                        let _ = write!(f, "*");
                    },
                    _ => { // Unknown
                        let _ = write!(f, "?");
                    }
                }
            }
            let _ = write!(f, "\n");
        }
        let _ = write!(f, "\n");
        fmt::Result::Ok(())
    }
}

impl Chtron {
    pub fn new(input: Vec<String>) -> Chtron {
        let mut start = (0usize,0usize);
        let mut end = (0usize,0usize);
        let mut pc = (0usize,0usize);
        let mut row = 0usize;
        let mut col: usize = 0usize;
        let mut maze = input.into_iter().map( |x| {
            row += 1;
            col = 0;
            x.chars().into_iter().map(|y| {
                col += 1;
                match &y {
                    '.' => 0,
                    '#' => 1,
                    'S' => {
                        start = (col-1, row-1);
                        2
                    },
                    'E' => {
                        end = (col-1, row-1);
                        3
                    },
                    _ => 0
                }
            }).collect() 
        }).collect();

        Chtron {
            maze: maze,
            start: start,
            end: end,
            cheated: false,
            pc: start,
            path_count: 0,
        }
    }

    pub fn step(&mut self) -> Result<bool, &'static str> {
        let (x,y) = self.pc;

        if self.maze[y][x] == 1 || self.maze[y][x] == 4 {
            // Cheaters never prosper.
            return Err("Segmentation Fault!");
        }

        if self.maze[y][x] == 3 {
            // Oh, that's weird...did I fall asleep?
            return Ok(true);
        }

        if x > 0 && self.maze[y][x-1] % 3 == 0 {
            if self.maze[y][x] != 2 {
                self.maze[y][x] = 4;
            }
            self.pc = (x-1, y);
            self.path_count += 1;
            if self.maze[y][x-1] == 3 {
                return Ok(true);
            }
        } else if y > 0 && self.maze[y-1][x] % 3 == 0 {
            if self.maze[y][x] != 2 {
                self.maze[y][x] = 4;
            }
            self.pc = (x, y-1);
            self.path_count += 1;
            if self.maze[y-1][x] == 3 {
                return Ok(true);
            }
        } else if self.maze[y][x+1] % 3 == 0 {
            if self.maze[y][x] != 2 {
                self.maze[y][x] = 4;
            }
            self.pc = (x+1, y);
            self.path_count += 1;
            if self.maze[y][x+1] == 3 {
                return Ok(true);
            }
        } else if self.maze[y+1][x] % 3 == 0 {
            if self.maze[y][x] != 2 {
                self.maze[y][x] = 4;
            }
            self.pc = (x, y+1);
            self.path_count += 1;
            if self.maze[y+1][x] == 3 {
                return Ok(true);
            }
        }

        Ok(false)
    }

    pub fn cheat(&mut self) {
        let (x,y) = self.pc;
        if x > 1 && self.maze[y][x-1] == 1 {
            if self.maze[y][x] != 2 {
                self.maze[y][x] = 4;
            }
            self.maze[y][x-1] = 4;
            self.pc = (x-2, y);
            self.path_count += 2;
        } else if y > 1 && self.maze[y-1][x] == 1 {
            if self.maze[y][x] != 2 {
                self.maze[y][x] = 4;
            }
            self.maze[y-1][x] = 4;
            self.pc = (x, y-2);
            self.path_count += 2;
        } else if x < self.maze[y].len() - 2 && self.maze[y][x+1] == 1 {
            if self.maze[y][x] != 2 {
                self.maze[y][x] = 4;
            }
            self.maze[y][x+1] = 4;
            self.pc = (x+2, y);
            self.path_count += 2;
        } else if y < self.maze.len() - 2 && self.maze[y+1][x] == 1 {
            if self.maze[y][x] != 2 {
                self.maze[y][x] = 4;
            }
            self.maze[y+1][x] = 4;
            self.pc = (x, y+2);
            self.path_count += 2;
        }
    }

    pub fn complete_journey(&mut self) -> Result<bool, &'static str> {
        loop {
            match self.step() {
                Ok(x) => {
                    if x {
                        return Ok(true);
                    }
                },
                Err(x) => {
                    warn!("Invalid journey! Pruning the timeline. ({})", x);
                    return Ok(false);
                }
            }
        }
        // Err("Not sure how I got here...")
    }

}

pub fn problem_201(input: Vec<String>) -> RetType {
    return RetType::U32(0);
}

pub fn problem_202(input: Vec<String>) -> RetType {
    return RetType::U32(0);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init() {
        match env_logger::try_init() {
            Ok(_) => {
                info!("Initializing logging...");
            },
            Err(_) => {

            }
        }
    }

    #[test]
    fn test_tronmaze() {
        init();

        let input = vec![
            "###############".to_string(),
            "#...#...#.....#".to_string(),
            "#.#.#.#.#.###.#".to_string(),
            "#S#...#.#.#...#".to_string(),
            "#######.#.#.###".to_string(),
            "#######.#.#...#".to_string(),
            "#######.#.###.#".to_string(),
            "###..E#...#...#".to_string(),
            "###.#######.###".to_string(),
            "#...###...#...#".to_string(),
            "#.#####.#.###.#".to_string(),
            "#.#...#.#.#...#".to_string(),
            "#.#.#.#.#.#.###".to_string(),
            "#...#...#...###".to_string(),
            "###############".to_string(),
        ];

        let mut cht = Chtron::new(input);
        debug!("pc: {:?}", cht.pc);
        debug!("{}", cht);

        cht.complete_journey();

        debug!("pc: {:?}", cht.pc);
        debug!("{}", cht);


        assert_eq!(84, cht.path_count);
    }

    
    #[test]
    fn test_cheat() {
        init();

        let input = vec![
            "###############".to_string(),
            "#...#...#.....#".to_string(),
            "#.#.#.#.#.###.#".to_string(),
            "#S#...#.#.#...#".to_string(),
            "#######.#.#.###".to_string(),
            "#######.#.#...#".to_string(),
            "#######.#.###.#".to_string(),
            "###..E#...#...#".to_string(),
            "###.#######.###".to_string(),
            "#...###...#...#".to_string(),
            "#.#####.#.###.#".to_string(),
            "#.#...#.#.#...#".to_string(),
            "#.#.#.#.#.#.###".to_string(),
            "#...#...#...###".to_string(),
            "###############".to_string(),
        ];

        let mut cht = Chtron::new(input);
        debug!("pc: {:?}", cht.pc);
        debug!("{}", cht);

        for _ in 0..12 {
            cht.step();
        }
        cht.cheat();

        cht.complete_journey();

        debug!("pc: {:?}", cht.pc);
        debug!("{}", cht);


        assert_eq!(72, cht.path_count);
    }
}