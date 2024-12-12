use log::{trace, debug, info, warn, error}; // trace, debug, info, warn, error
use crate::util::RetType;
use std::collections::HashSet;
// use std::{cmp::Ordering::{Equal, Greater, Less}, fmt};

#[allow(dead_code)]
fn _get_rid_of_log_unused_import_warnings() {
    trace!("Example trace.");
    debug!("Example debug.");
    info!("Example info.");
    warn!("Example warn.");
    error!("Example error.");
}

pub fn parse_input(input: Vec<String>) -> Vec<Vec<char>> {
    input.into_iter()
         .map(|x| x.chars().into_iter().collect())
         .collect()
}

pub fn identify_crops(input: Vec<Vec<char>>) -> HashSet<char> {
    let mut crops = HashSet::new();
    for v in input {
        for c in v {
            crops.insert(c);
        }
    }
    crops
}

// fn pt_cmp(a: &(u32, u32), other: &(u32, u32)) -> std::cmp::Ordering {
//     if a.0 < other.0 || ( a.0 == other.0 && a.1 < other.1 ) {
//         Less
//     } else if a.0 > other.0 || ( a.0 == other.0 && a.1 > other.1 ) {
//         Greater
//     } else {
//         Equal
//     }
// }


// /**
//  * Cross product of OA and OB, returns magnitude
//  * Positive for CW, negative for CCW
//  */
// fn cross_product(o: (u32, u32), a: (u32, u32), b: (u32, u32)) -> i32 {
//     let left = ((a.0 as i32 - o.0 as i32) * (b.1 as i32 - o.1 as i32));
//     let right = ((a.1 as i32 - o.1 as i32) * (b.0 as i32 - o.0 as i32));
//     if left > right {
//         1
//     } else if left < right {
//         -1
//     } else {
//         0
//     }
// }

pub fn calc_hull(input: Vec<Vec<char>>, crop_type: char) -> (u32, u32) {
    // First, find location of all crops
    let mut crops: Vec<(u32, u32)> = Vec::new();
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            if input[y][x] == crop_type {
                crops.push((x as u32,y as u32));
            }
        }
    }

    // debug!("Crops for {}: {:?}", crop_type, crops);

    // Now try to figure out the hull or something
    let mut fences = 0;
    for crop in crops.clone() {
        // Check all sides
        let mut friendly_sides = 0;
        if crop.0 > 0 && input[crop.1 as usize][crop.0 as usize-1] == crop_type {
            friendly_sides += 1;
        } 
        if crop.0 < input[0].len() as u32 - 1 && input[crop.1 as usize][crop.0 as usize+1] == crop_type {
            friendly_sides += 1;
        }
        if crop.1 > 0 && input[crop.1 as usize-1][crop.0 as usize] == crop_type {
            friendly_sides += 1;
        }
        if crop.1 < input.len() as u32 - 1 && input[crop.1 as usize+1][crop.0 as usize] == crop_type {
            friendly_sides += 1;
        }
        // debug!("crop: {:?}, sides: {}", crop, friendly_sides);
        fences += 4 - friendly_sides;
    }

    (fences, crops.len() as u32)
}

pub fn problem_121(input: Vec<String>) -> RetType {
    let parsed = parse_input(input);
    let crops = identify_crops(parsed.clone());
    let mut s = 0u32;
    for crop in crops {
        let (p, a) = calc_hull(parsed.clone(), crop);
        s += p*a;
    }
    return RetType::U32(s);
}

pub fn problem_122(input: Vec<String>) -> RetType {
    return RetType::U64(0);
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
    fn test_hull() {
        init();

        let input = vec![
            "AAAA".to_string(),
            "BBCD".to_string(),
            "BBCC".to_string(),
            "EEEC".to_string(),
        ];

        let parsed = parse_input(input.clone());

        assert_eq!((10, 4), calc_hull(parsed.clone(), 'A'));
        assert_eq!((8, 4), calc_hull(parsed.clone(), 'B'));
        assert_eq!((10, 4), calc_hull(parsed.clone(), 'C'));
        assert_eq!((4, 1), calc_hull(parsed.clone(), 'D'));
        assert_eq!((8, 3), calc_hull(parsed.clone(), 'E'));

        assert_eq!(problem_121(input), RetType::U32(140));
    }

    #[test]
    fn test_part1() {
        init();

        let input = vec![
            "RRRRIICCFF".to_string(),
            "RRRRIICCCF".to_string(),
            "VVRRRCCFFF".to_string(),
            "VVRCCCJFFF".to_string(),
            "VVVVCJJCFE".to_string(),
            "VVIVCCJJEE".to_string(),
            "VVIIICJJEE".to_string(),
            "MIIIIIJJEE".to_string(),
            "MIIISIJEEE".to_string(),
            "MMMISSJEEE".to_string(),
        ];

        let res = problem_121(input);
        assert_eq!(RetType::U32(1930), res);
    }

}