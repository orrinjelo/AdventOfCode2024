use log::{trace, debug, info, warn, error}; // trace, debug, info, warn, error
use crate::util::RetType;
use std::fmt;
use std::collections::HashSet;

#[allow(dead_code)]
fn _get_rid_of_log_unused_import_warnings() {
    trace!("Example trace.");
    debug!("Example debug.");
    info!("Example info.");
    warn!("Example warn.");
    error!("Example error.");
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Direction {
    N, E, S, W
}

#[derive(Clone, Debug, PartialEq)]
pub struct Guard {
    position: (i32, i32),
    direction: Direction,
    history: Vec<Vec<u8>>,
    map: Vec<Vec<u8>>,
    path: HashSet<((i32, i32), Direction)>,
    initial_state: ((i32, i32), Direction),
    looped: bool,
}

impl fmt::Display for Guard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.map[0].len() {
            let _ = write!(f, "\n");
            for x in 0..self.map.len() {
                let _result = 
                write!(f, "{}", if self.history[x][y] == 1 {
                                   'X'
                                } else {
                                    if self.map[x][y] == 1 {
                                        '#'
                                    } else {
                                        if self.position == (x as i32, y as i32) {
                                            'o'
                                        } else {
                                            '.'
                                        }
                                    }
                                });
            }
        }

        fmt::Result::Ok(())
    }
}

impl Guard {
    pub fn new(pos: (i32, i32), dir: Direction, map_size: (usize, usize)) -> Guard {
        Guard {
            position: pos,
            direction: dir.clone(),
            history: vec!(vec!(0; map_size.1); map_size.0),
            map: vec!(vec!(0; map_size.1); map_size.0),
            path: HashSet::new(),
            initial_state: (pos, dir),
            looped: false,
        }
    }

    pub fn do_your_duty(&mut self) {
        while self.position.0 >= 0 && self.position.0 < self.map.len() as i32 &&
              self.position.1 >= 0 && self.position.1 < self.map[0].len() as i32 {
                self.step();
                if self.initial_state == (self.position, self.direction.clone()) 
                  || self.path.contains(&(self.position, self.direction.clone())) {
                    self.looped = true;
                    break;
                }
        }
    }

    pub fn step (&mut self) {
        // Leave a poop
        self.history[self.position.0 as usize][self.position.1 as usize] = 1;
        if self.initial_state != (self.position, self.direction.clone()) {
            self.path.insert((self.position, self.direction.clone()));
        }

        let mut stepped = false;
        let mut rotations = 0;

        while !stepped && rotations < 4 {
            // Marching orders
            match self.direction {
                Direction::N => {
                    if self.position.1 > 0 && self.map[self.position.0 as usize][(self.position.1 - 1) as usize] != 0 {
                        self.direction = Direction::E;
                        // self.position.0 += 1;
                        rotations += 1;
                    } else {
                        self.position.1 -= 1;
                        stepped = true;
                    }
                },
                Direction::E => {
                    if self.position.0 < (self.map.len()-1) as i32 && self.map[(self.position.0 + 1) as usize][self.position.1 as usize] != 0 {
                        self.direction = Direction::S;
                        // self.position.1 += 1;
                        rotations += 1;
                    } else {
                        self.position.0 += 1;
                        stepped = true;
                    }
                },
                Direction::S => {
                    if self.position.1 < (self.map[0].len()-1) as i32 && self.map[self.position.0 as usize][(self.position.1 + 1) as usize] != 0 {
                        self.direction = Direction::W;
                        // self.position.0 -= 1;
                        rotations += 1;
                    } else {
                        self.position.1 += 1;
                        stepped = true;
                    }
                },
                Direction::W => {
                    if self.position.0 > 0 && self.map[(self.position.0-1) as usize][self.position.1 as usize] != 0 {
                        self.direction = Direction::N;
                        // self.position.1 -= 1;
                        rotations += 1;
                    } else {
                        self.position.0 -= 1;
                        stepped = true;
                    }
                },
            }
        }
    }
    
}

pub fn parse_map(input: Vec<String>) -> Guard {
    let map_size = (input[0].len(), input.len());
    let mut g = Guard::new((-1,-1), Direction::N, map_size);
    let mut x = -1i32;
    let mut y = -1i32;
    g.map = input.into_iter()
                 .map(|line| {
                    y += 1;
                    x = -1;
                    line.as_str()
                        .chars()
                        .into_iter()
                        .map(|c| {
                            x += 1;
                            match c {
                                '.' => 0,
                                '#' => 1,
                                '^' => { 
                                    g.direction = Direction::N;
                                    g.position = (x, y);
                                    g.initial_state = ((x,y), Direction::N);
                                    0
                                },
                                '>' => { 
                                    g.direction = Direction::E;
                                    g.position = (x, y);
                                    g.initial_state = ((x,y), Direction::E);
                                    0
                                },
                                'v' => { 
                                    g.direction = Direction::S;
                                    g.position = (x, y);
                                    g.initial_state = ((x,y), Direction::S);
                                    0
                                },
                                '<' => { 
                                    g.direction = Direction::W;
                                    g.position = (x, y);
                                    g.initial_state = ((x,y), Direction::W);
                                    0
                                },
                                _ => 0,
                            }
                        })
                        .collect()
                }).collect(); 
    
    // Transpose
    let len = g.map[0].len();
    let mut iters: Vec<_> = g.map.into_iter().map(|n| n.into_iter()).collect();
    g.map = (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect()
        })
        .collect::<Vec<Vec<u8>>>();
    g
}


pub fn problem_061(input: Vec<String>) -> RetType {
    let mut guard = parse_map(input);
    guard.do_your_duty();
    return RetType::U32(
        guard.history.into_iter()
                     .map(|line| line.into_iter().map(|x| x as u32).sum::<u32>())
                     .sum::<u32>()
    );
}

pub fn problem_062(input: Vec<String>) -> RetType {
    let mut guard = parse_map(input);
    guard.do_your_duty();

    let mut loopy_timelines = 0;
    let filtered_path = guard.path.into_iter()
        .map(|x| x.0)
        .collect::<HashSet<_>>();
    for spot in filtered_path.clone() {
        let mut temporal_guard = Guard::new(
            guard.initial_state.clone().0, 
            guard.initial_state.clone().1,
            (guard.map.len(), guard.map[0].len()),
        );
        temporal_guard.map = guard.map.clone();
        temporal_guard.map[spot.0 as usize][spot.1 as usize] = 1;

        temporal_guard.do_your_duty();

        if temporal_guard.looped {
            loopy_timelines += 1;
        }
    }

    return RetType::U32(loopy_timelines);
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
    fn test_guard_time() {
        init();
        let input = vec!(
            "....#.....".to_string(),
            ".........#".to_string(),
            "..........".to_string(),
            "..#.......".to_string(),
            ".......#..".to_string(),
            "..........".to_string(),
            ".#..^.....".to_string(),
            "........#.".to_string(),
            "#.........".to_string(),
            "......#...".to_string(),
        );

        let mut guard = parse_map(input);

        // debug!("{}", guard);

        // guard.step();
        // guard.step();
        // guard.step();
        // guard.step();
        // guard.step();
        // guard.step();
        // guard.step();

        // debug!("{}", guard);
        // debug!("{:?}", guard);

        guard.do_your_duty();

        // debug!("{:?}", guard);

        assert_eq!(guard.history.into_iter()
                                .map(|line| line.into_iter().map(|x| x as u32).sum::<u32>())
                                .sum::<u32>(),
                   41);

        let mut loopy_timelines = 0;
        let filtered_path = guard.path.into_iter()
            .map(|x| x.0)
            .collect::<HashSet<_>>();
        for spot in filtered_path.clone() {
            let mut temporal_guard = Guard::new(
                guard.initial_state.clone().0, 
                guard.initial_state.clone().1,
                (guard.map.len(), guard.map[0].len()),
            );
            temporal_guard.map = guard.map.clone();
            temporal_guard.map[spot.0 as usize][spot.1 as usize] = 1;

            temporal_guard.do_your_duty();

            if temporal_guard.looped {
                loopy_timelines += 1;
                // debug!("Found {:?}", (spot.0 as usize, spot.1 as usize));
            }
        }

        assert_eq!(loopy_timelines, 6);
    }

    #[test]
    fn test_part2() {
        let input = vec![
            "..#..".to_string(),
            ".#.#.".to_string(),
            "..^..".to_string(),
            ".#.#.".to_string(),
            "..#..".to_string(),
        ];

        let mut guard = parse_map(input);
        guard.do_your_duty();
    
        let mut loopy_timelines = 0;
        let filtered_path = guard.path.into_iter()
            .map(|x| x.0)
            .collect::<HashSet<_>>();
        for spot in filtered_path.clone() {
            let mut temporal_guard = Guard::new(
                guard.initial_state.clone().0, 
                guard.initial_state.clone().1,
                (guard.map.len(), guard.map[0].len()),
            );
            temporal_guard.map = guard.map.clone();
            temporal_guard.map[spot.0 as usize][spot.1 as usize] = 1;
    
            temporal_guard.do_your_duty();
    
            if temporal_guard.looped {
                loopy_timelines += 1;
            }

            // debug!("{}", temporal_guard);
        }

        assert_eq!(loopy_timelines, 1);

    }

}