use log::{trace, debug, info, warn, error}; // trace, debug, info, warn, error
use crate::util::RetType;
use std::collections::HashMap;

#[allow(dead_code)]
fn _get_rid_of_log_unused_import_warnings() {
    trace!("Example trace.");
    debug!("Example debug.");
    info!("Example info.");
    warn!("Example warn.");
    error!("Example error.");
}

/**
 * Problem #01, Part 1
 */
pub fn problem_011(input: Vec<String>) -> RetType {
    return RetType::U32( {
        let mut left_vec: Vec<u32> = Vec::new();
        let mut right_vec: Vec<u32> = Vec::new();
        for x in input.into_iter() {
            let mut vals = x.split("   ");
            let left = vals.next().unwrap().parse::<u32>().unwrap();
            let right = vals.next().unwrap().parse::<u32>().unwrap();
            left_vec.push(left);
            right_vec.push(right);
        }
        left_vec.sort();
        right_vec.sort();
        let mut distsum: u32 = 0;
        for i in 0..left_vec.len() {
            let dist = if left_vec[i] > right_vec[i] {left_vec[i]-right_vec[i]} else {right_vec[i]-left_vec[i]};
            distsum += dist;
        }
        distsum
    })
}

/**
 * Problem #01, Part 2
 */
pub fn problem_012(input: Vec<String>) -> RetType {
    RetType::U32({
        let mut left_vec: Vec<u32> = Vec::new();
        let mut right_map = HashMap::<u32, u32>::new();
        for x in input.into_iter() {
            let mut vals = x.split("   ");
            let left = vals.next().unwrap().parse::<u32>().unwrap();
            let right = vals.next().unwrap().parse::<u32>().unwrap();
            left_vec.push(left);
            right_map.entry(right).and_modify(|c| *c += 1).or_insert(1);
        }
        
        left_vec.into_iter()
                .map(|x| x * match right_map.get(&x) {
                    Some(y) => *y,
                    None => 0,
                })
                .sum()
    })

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
    fn test_part011() {
        init();
        let input_str = vec!(
            "3   4".to_string(),
            "4   3".to_string(),
            "2   5".to_string(),
            "1   3".to_string(),
            "3   9".to_string(),
            "3   3".to_string()
        );

        assert_eq!(problem_011(input_str), RetType::U32(11));
    }

    #[test]
    fn test_part012() {
        init();
        let input_str = vec!(
            "3   4".to_string(),
            "4   3".to_string(),
            "2   5".to_string(),
            "1   3".to_string(),
            "3   9".to_string(),
            "3   3".to_string()
        );

        assert_eq!(problem_012(input_str), RetType::U32(31));
    }

}