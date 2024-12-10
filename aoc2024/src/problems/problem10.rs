use log::{trace, debug, info, warn, error}; // trace, debug, info, warn, error
use crate::util::RetType;
use std::collections::HashSet;
// use std::fmt;

#[allow(dead_code)]
fn _get_rid_of_log_unused_import_warnings() {
    trace!("Example trace.");
    debug!("Example debug.");
    info!("Example info.");
    warn!("Example warn.");
    error!("Example error.");
}

pub fn parse_input(input: Vec<String>) -> Vec<Vec<u8>> {
    input.into_iter()
         .map(|x| x.chars()
                           .into_iter()
                           .map(|y| y.to_digit(10).unwrap() as u8)
                           .collect())
         .collect()
}

pub fn explore(input: Vec<Vec<u8>>, loc: (usize, usize), elev: u8) -> HashSet<(usize, usize)> {
    let mut peaks: HashSet<(usize, usize)> = HashSet::new();
    if elev == 9 {
        peaks.insert(loc);
        return peaks;
        // return 1;
    }
    // let mut count = 0u32;
    if loc.1 > 0 && input[loc.1 - 1][loc.0] == elev+1 {
        let ret = explore(input.clone(), (loc.0, loc.1-1), elev+1);
        for p in ret {
            peaks.insert(p);
        }
    }
    if loc.0 > 0 && input[loc.1][loc.0 - 1] == elev+1 {
        let ret = explore(input.clone(), (loc.0-1, loc.1), elev+1);
        for p in ret {
            peaks.insert(p);
        }
    }
    if loc.1 < input[0].len()-1 && input[loc.1 + 1][loc.0] == elev+1 {
        let ret = explore(input.clone(), (loc.0, loc.1+1), elev+1);
        for p in ret {
            peaks.insert(p);
        }
    }
    if loc.0 < input.len()-1 && input[loc.1][loc.0+1] == elev+1 {
        let ret = explore(input.clone(), (loc.0+1, loc.1), elev+1);
        for p in ret {
            peaks.insert(p);
        }
    }
    // return count;
    return peaks;
}

pub fn explore_rating(input: Vec<Vec<u8>>, loc: (usize, usize), elev: u8) -> u32 {
    if elev == 9 {
        return 1;
    }
    let mut count = 0u32;
    if loc.1 > 0 && input[loc.1 - 1][loc.0] == elev+1 {
        count += explore_rating(input.clone(), (loc.0, loc.1-1), elev+1);
    }
    if loc.0 > 0 && input[loc.1][loc.0 - 1] == elev+1 {
        count += explore_rating(input.clone(), (loc.0-1, loc.1), elev+1);
    }
    if loc.1 < input[0].len()-1 && input[loc.1 + 1][loc.0] == elev+1 {
        count += explore_rating(input.clone(), (loc.0, loc.1+1), elev+1);
    }
    if loc.0 < input.len()-1 && input[loc.1][loc.0+1] == elev+1 {
        count += explore_rating(input.clone(), (loc.0+1, loc.1), elev+1);
    }
    return count;
}

pub fn find_trails(input: Vec<Vec<u8>>) -> u32 {
    let mut trailheads: Vec<(usize, usize)> = Vec::new();
    let mut trails: Vec<u32> = Vec::new();
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            if input[y][x] == 0 {
                trailheads.push((x,y));
                let peaks = explore(input.clone(), (x,y), 0);
                trails.push(peaks.len() as u32);
            }
        }
    }
    trails.into_iter().sum()
}

pub fn find_trail_ratings(input: Vec<Vec<u8>>) -> u32 {
    let mut trailheads: Vec<(usize, usize)> = Vec::new();
    let mut trails: Vec<u32> = Vec::new();
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            if input[y][x] == 0 {
                trailheads.push((x,y));
                trails.push(explore_rating(input.clone(), (x,y), 0));
            }
        }
    }
    trails.into_iter().sum()
}


pub fn problem_101(input: Vec<String>) -> RetType {
    return RetType::U32(find_trails(parse_input(input)));
}

pub fn problem_102(input: Vec<String>) -> RetType {
    return RetType::U32(find_trail_ratings(parse_input(input)));
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
   fn test_stuff() {
        init();

        let input = vec![
            "89010123".to_string(),
            "78121874".to_string(),
            "87430965".to_string(),
            "96549874".to_string(),
            "45678903".to_string(),
            "32019012".to_string(),
            "01329801".to_string(),
            "10456732".to_string(),
        ];

        assert_eq!(problem_101(input.clone()), RetType::U32(36));
        assert_eq!(problem_102(input), RetType::U32(81));
    }
}