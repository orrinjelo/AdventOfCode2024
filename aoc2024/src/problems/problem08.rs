use log::{trace, debug, info, warn, error}; // trace, debug, info, warn, error
use crate::util::RetType;
// use regex::Regex;
use std::collections::{HashMap, HashSet};

#[allow(dead_code)]
fn _get_rid_of_log_unused_import_warnings() {
    trace!("Example trace.");
    debug!("Example debug.");
    info!("Example info.");
    warn!("Example warn.");
    error!("Example error.");
}

pub fn parse_map(input: Vec<String>) -> ((u32, u32), HashMap<char, Vec<(u32, u32)>>) {
    let mut map: HashMap<char, Vec<(u32, u32)>> = HashMap::new();
    for y in 0..input.len() {
        for x in 0..input[y].len() {
            let key = input[y].as_bytes()[x] as char;
            if key != '.' {
                if map.contains_key(&(key)) {
                    map.get_mut(&key).expect("Invalid key.").push((x as u32,y as u32));
                } else {
                    map.insert(key, Vec::new() );
                    map.get_mut(&key).expect("Invalid key.").push((x as u32,y as u32));
                }
            }
        }
    }
    ((input[0].len() as u32, input.len() as u32), map)
}

pub fn calc_antinodes(map: HashMap<char, Vec<(u32, u32)>>, map_size: (u32, u32)) -> u32 {
    let mut antinodes: HashSet<(u32, u32)> = HashSet::new();
    for (_k, v) in map {
        for i in 0..v.len() {
            for j in 0..v.len() {
                if i == j {
                    continue;
                }
                let a_node = (2*v[i].0 as i32 - v[j].0 as i32, 2*v[i].1 as i32 - v[j].1 as i32);
                if a_node.0 >= map_size.0 as i32 || a_node.0 < 0 || a_node.1 >= map_size.1 as i32 || a_node.1 < 0 {
                    continue;
                }
                antinodes.insert((a_node.0 as u32, a_node.1 as u32));
            }
        }
    }

    // debug!("{:?}", map_size);
    // debug!("{:?}", antinodes);
    antinodes.len() as u32
}


pub fn calc_antinodes_harmonics(map: HashMap<char, Vec<(u32, u32)>>, map_size: (u32, u32)) -> u32 {
    let mut antinodes: HashSet<(u32, u32)> = HashSet::new();
    for (_k, v) in map.clone() {
        for i in 0..v.len() {
            for j in 0..v.len() {
                if i == j {
                    continue;
                }
                for k in 0..map_size.0 {
                    let a_node = ((v[i].0 as i32 - v[j].0 as i32)*(k as i32) + v[i].0 as i32, (v[i].1 as i32 - v[j].1 as i32)*(k as i32) + v[i].1 as i32);
                    if a_node.0 >= map_size.0 as i32 || a_node.0 < 0 || a_node.1 >= map_size.1 as i32 || a_node.1 < 0 {
                        continue;
                    }
                    antinodes.insert((a_node.0 as u32, a_node.1 as u32));
                }
            }
        }
    }
    print_map(map, antinodes.clone(), map_size);
    antinodes.len() as u32
}

pub fn print_map(map: HashMap<char, Vec<(u32, u32)>>, antinodes: HashSet<(u32, u32)>, map_size: (u32, u32)) {
    let mut pmap: Vec<Vec<char>> = vec![vec!['.'; map_size.0 as usize]; map_size.1 as usize];
    for antinode in antinodes {
        pmap[antinode.1 as usize][antinode.0 as usize] = 'X';
    }
    for (ant, locs) in map {
        for loc in locs {
            pmap[loc.1 as usize][loc.0 as usize] = ant;
        }
    }

    for line in pmap {
        print!("{}\n",
            line.into_iter()
                .collect::<String>()
        );
    }
}

pub fn problem_081(input: Vec<String>) -> RetType {
    let (size, parsed_input) = parse_map(input);
    return RetType::U32(calc_antinodes(parsed_input, size));
}

pub fn problem_082(input: Vec<String>) -> RetType {
    let (size, parsed_input) = parse_map(input);
    return RetType::U32(calc_antinodes_harmonics(parsed_input, size));
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
   fn test_count_antinodes() {
        init();

        let input = vec![
            "............".to_string(),
            "........0...".to_string(),
            ".....0......".to_string(),
            ".......0....".to_string(),
            "....0.......".to_string(),
            "......A.....".to_string(),
            "............".to_string(),
            "............".to_string(),
            "........A...".to_string(),
            ".........A..".to_string(),
            "............".to_string(),
            "............".to_string(),
        ];

        let (size, map) = parse_map(input);

        assert_eq!(calc_antinodes(map, size), 14);

    }

    #[test]
   fn test_count_antinode_harmonics() {
        init();

        let input = vec![
            "............".to_string(),
            "........0...".to_string(),
            ".....0......".to_string(),
            ".......0....".to_string(),
            "....0.......".to_string(),
            "......A.....".to_string(),
            "............".to_string(),
            "............".to_string(),
            "........A...".to_string(),
            ".........A..".to_string(),
            "............".to_string(),
            "............".to_string(),
        ];

        let (size, map) = parse_map(input);

        assert_eq!(calc_antinodes_harmonics(map, size), 34);

    }
}