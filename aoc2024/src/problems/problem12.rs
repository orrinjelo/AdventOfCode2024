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

pub fn clusters(input: Vec<(u32, u32)>) -> Vec<Vec<(u32, u32)>> {
    let mut used = HashSet::new();
    let mut clusters_vec: Vec<Vec<(u32, u32)>> = Vec::new();

    struct Neighs<'s> { f: &'s dyn Fn(&Neighs, &mut HashSet<(u32, u32)>, (u32,u32)) -> Vec<(u32, u32)> }
    let find_neighbors = Neighs {
        f: &|neigh, used, x: (u32, u32)| -> Vec<(u32, u32)> {
            used.insert(x);
            // debug!("Finding neighbors for {:?}", x);
            let mut neighbors = Vec::new();
            if used.len() == input.len() {
                return neighbors;
            }
            for i in input.clone() {
                if i != x && !used.contains(&i) {
                    if (i.0 as i32 - x.0 as i32).pow(2) + (i.1 as i32 - x.1 as i32).pow(2) == 1 {
                        neighbors.push(i);
                        let nn = (neigh.f)(&neigh, used, i);
                        for n in nn {
                            neighbors.push(n);
                        }
                    }
                }
            }
            neighbors
        }
    };

    let mut idx: usize = 0;
    while idx < input.len() {
        let mut curr = input[idx];
        if !used.contains(&curr) {
            let mut neighbors = (find_neighbors.f)(&find_neighbors, &mut used, curr);
            neighbors.push(curr);
            clusters_vec.push(neighbors);
        }
        idx += 1;
    }

    clusters_vec
}

pub fn calc_hull(input: Vec<Vec<char>>, crop_type: char) -> Vec<(u32, u32, u32)> {
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
    let clusters_vec = clusters(crops.clone());
    // debug!("Clusters for {}: {:?}", crop_type, clusters_vec.clone());
    let mut hulls: Vec<(u32, u32, u32)> = Vec::new();

    for cluster in clusters_vec {
        // Now try to figure out the hull or something
        let mut fences = 0;
        let mut corners = 0;
        for crop in cluster.clone() {
            // Check all sides
            let mut friendly_sides = 0;
            let mut check: u8 = 0;
            if crop.0 > 0 && input[crop.1 as usize][crop.0 as usize-1] == crop_type {
                friendly_sides += 1;
            } else {
                check |= 0x1;
            } 
            if crop.0 < input[0].len() as u32 - 1 && input[crop.1 as usize][crop.0 as usize+1] == crop_type {
                friendly_sides += 1;
            } else {
                check |= 0x4;
            }
            if crop.1 > 0 && input[crop.1 as usize-1][crop.0 as usize] == crop_type {
                friendly_sides += 1;
            } else {
                check |= 0x8;
            }
            if crop.1 < input.len() as u32 - 1 && input[crop.1 as usize+1][crop.0 as usize] == crop_type {
                friendly_sides += 1;
            } else {
                check |= 0x2;
            }
            // debug!("crop: {:?}, sides: {}", crop, friendly_sides);
            fences += 4 - friendly_sides;
            // Do some esoteric mumbo-jumbo to find out corners. :)
            if check % 3 == 0 && check > 0 {
                corners += 1;
            } else if vec![7, 11, 13, 14].contains(&check) {
                corners += 2;
            } else if check == 0xf {
                corners += 4;
            } else if vec![1, 2, 4, 8].contains(&check) {
                corners += 2;
            }
            debug!("crop {} {:?} is a corner?: {}", crop_type, crop, check);
        }
        hulls.push((fences, cluster.len() as u32, corners));
    }
    hulls
    
}

pub fn problem_121(input: Vec<String>) -> RetType {
    let parsed = parse_input(input);
    let crops = identify_crops(parsed.clone());
    let mut s = 0u32;
    for crop in crops {
        for (p, a, _) in calc_hull(parsed.clone(), crop) {
            s += p*a;
        }
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

        assert_eq!((10, 4, 4), calc_hull(parsed.clone(), 'A')[0]);
        assert_eq!((8, 4, 4), calc_hull(parsed.clone(), 'B')[0]);
        assert_eq!((10, 4, 8), calc_hull(parsed.clone(), 'C')[0]);
        assert_eq!((4, 1, 4), calc_hull(parsed.clone(), 'D')[0]);
        assert_eq!((8, 3, 4), calc_hull(parsed.clone(), 'E')[0]);

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

        let parsed = parse_input(input.clone());
        let mut crops: Vec<(u32, u32)> = Vec::new();
        for y in 0..parsed.len() {
            for x in 0..parsed[0].len() {
                if parsed[y][x] == 'I' {
                    crops.push((x as u32,y as u32));
                }
            }
        }

        let clusters_vec = clusters(crops);

        assert_eq!(clusters_vec.len(), 2);

        assert_eq!((18, 12, 10), calc_hull(parsed.clone(), 'R')[0]);
        let ihulls = calc_hull(parsed.clone(), 'I');
        assert_eq!((8, 4, 4), ihulls.clone()[0]);
        assert_eq!((22, 14, 16), ihulls.clone()[1]);


        let res = problem_121(input);
        assert_eq!(RetType::U32(1930), res);
    }

}