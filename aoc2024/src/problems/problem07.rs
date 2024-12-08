use log::{trace, debug, info, warn, error}; // trace, debug, info, warn, error
use crate::util::RetType;
// use regex::Regex;

#[allow(dead_code)]
fn _get_rid_of_log_unused_import_warnings() {
    trace!("Example trace.");
    debug!("Example debug.");
    info!("Example info.");
    warn!("Example warn.");
    error!("Example error.");
}

pub fn parse_equations(input: Vec<String>) -> Vec<(u128, Vec<u128>)> {
    let mut equations_raw: Vec<(u128, Vec<u128>)> = Vec::new();
    for eq in input {
        let parts1: Vec<_> = eq.split(':').collect();
        let (total, nums) = (parts1[0].parse::<u128>().unwrap(), parts1[1]);
        let num_vec = nums.trim()
                          .split(' ')
                          .into_iter()
                          .map(|x| x.parse::<u128>().unwrap())
                          .collect();
        equations_raw.push((total, num_vec));
    }
    equations_raw
}

pub fn add_mult(equations: Vec<(u128, Vec<u128>)>) -> u128 {
    let mut calibration_result = 0;
    // let mut count = 0;
    for (total, nums) in equations {
        if add_mult_single(total, nums) {
            calibration_result += total;
        }
    }
    calibration_result
}

pub fn add_mult_cat(equations: Vec<(u128, Vec<u128>)>) -> u128 {
    let mut calibration_result = 0;
    for (total, nums) in equations {
        if add_mult_cat_single(total, nums) {
            calibration_result += total;
            // print!("True\n");
        } else {
            // print!("False\n");
        }
    }
    calibration_result
}

pub fn add_mult_single(total: u128, nums: Vec<u128>) -> bool {
    if nums.len() == 1 && total == nums[0] {
        return true;  // Found solution
    }
    else if nums.len() == 1 {
        return false; // Exceeded solution
    }
    // Try add
    let mut new_nums = Vec::new();
    new_nums.push(nums[0]+nums[1]);
    for i in 2..nums.len() {
        new_nums.push(nums[i]);
    }
    if add_mult_single(total, new_nums.clone()) {
        return true;
    }
    // Try mult
    let mut new_nums = Vec::new();
    new_nums.push(nums[0]*nums[1]);
    for i in 2..nums.len() {
        new_nums.push(nums[i]);
    }
    return add_mult_single(total, new_nums.clone());

}

pub fn add_mult_cat_single(total: u128, nums: Vec<u128>) -> bool {
    if nums.len() == 1 && total == nums[0] {
        return true;  // Found solution
    }
    else if nums.len() == 1 {
        return false; // Exceeded solution
    }
    // Try add
    let mut new_nums = Vec::new();
    new_nums.push(nums[0]+nums[1]);
    for i in 2..nums.len() {
        new_nums.push(nums[i]);
    }
    if add_mult_cat_single(total, new_nums.clone()) {
        return true;
    }
    // Try mult
    let mut new_nums = Vec::new();
    new_nums.push(nums[0]*nums[1]);
    for i in 2..nums.len() {
        new_nums.push(nums[i]);
    }
    if add_mult_cat_single(total, new_nums.clone()) {
        return true;
    }
    // Try mult
    let mut new_nums = Vec::new();
    new_nums.push(nums[1] + nums[0]*(10u128).pow(( (nums[1] as f32).log10() + 1.0) as u32));
    for i in 2..nums.len() {
        new_nums.push(nums[i]);
    }
    return add_mult_cat_single(total, new_nums.clone());
}

pub fn problem_071(input: Vec<String>) -> RetType {
    let parsed_input = parse_equations(input);
    return RetType::U128(add_mult(parsed_input));
}

pub fn problem_072(input: Vec<String>) -> RetType {
    let parsed_input = parse_equations(input);
    // debug!("{:?}", parsed_input);
    return RetType::U128(add_mult_cat(parsed_input));
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
   fn test_addmult() {
        init();

        let input = vec![
            "190: 10 19".to_string(),
            "3267: 81 40 27".to_string(),
            "83: 17 5".to_string(),
            "156: 15 6".to_string(),
            "7290: 6 8 6 15".to_string(),
            "161011: 16 10 13".to_string(),
            "192: 17 8 14".to_string(),
            "21037: 9 7 18 13".to_string(),
            "292: 11 6 16 20".to_string(),
        ];

        let parsed_input = parse_equations(input);
        assert_eq!(3749, add_mult(parsed_input));
    }

    #[test]
    fn test_part1() {
        init();
        let input1 = vec![
            "15: 1 2 3 4 5".to_string(),
        ];

        assert_eq!(problem_071(input1), RetType::U128(15));

        let input1 = vec![
            "15: 1 2 3 4 5".to_string(),
            "120: 1 2 3 4 5".to_string(),
        ];

        assert_eq!(problem_071(input1), RetType::U128(135));

        let input1 = vec![
            "15: 1 2 3 4 5".to_string(),
            "120: 1 2 3 4 5".to_string(),
            "50: 1 2 3 4 5".to_string(),
            "29: 1 2 3 4 5".to_string(),
            "18: 1 2 3 4 5".to_string(),
            "14: 1 2 3 4 5".to_string(),
        ];

        assert_eq!(problem_071(input1), RetType::U128(246));

        let input1 = vec![
            "25: 1 2 3 4 5".to_string(),
            "10: 1 1 1 1 1 1 1 1 1 1".to_string(),
            "1: 1 1 1 1 1 1 1 1 1 1".to_string(),
            "2: 1 1 1 1 1 1 1 1 1 1".to_string(),
            "3: 1 1 1 1 1 1 1 1 1 1".to_string(),
            "4: 1 1 1 1 1 1 1 1 1 1".to_string(),
            "5: 1 1 1 1 1 1 1 1 1 1".to_string(),
        ];

        assert_eq!(problem_071(input1), RetType::U128(50));

        let input1 = vec![
            "65240: 90 561 48 87 5".to_string(),
        ];
        assert_eq!(problem_072(input1), RetType::U128(65240));
    }

    #[test]
    fn test_7part2() {
        init();
        let input1 = vec![
            "70734: 9 8 3 1 741 61 6 2 40 7 7".to_string(),
        ];
        assert_eq!(problem_072(input1), RetType::U128(70734));

    }
}