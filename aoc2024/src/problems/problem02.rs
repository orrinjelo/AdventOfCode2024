use log::{trace, debug, info, warn, error}; // trace, debug, info, warn, error
use crate::util::RetType;
// use std::num;
// use std::collections::HashMap;

#[allow(dead_code)]
fn _get_rid_of_log_unused_import_warnings() {
    trace!("Example trace.");
    debug!("Example debug.");
    info!("Example info.");
    warn!("Example warn.");
    error!("Example error.");
}

pub fn parse_levels(input: Vec<String>) -> Vec<Vec<i32>> {
    let mut lines: Vec<Vec<i32>> = Vec::new();
    for line in input {
        lines.push(
            line.split(' ').into_iter().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>()
        );
    }

    lines
}

pub fn is_safe_levels(levels: Vec<i32>) -> bool {
    let mut state = 0; // 0 = unset, 1 = inc, 2 = dec
    for i in 1..levels.len() {
        if (levels[i-1] - levels[i]).abs() > 3 || levels[i-1] == levels[i] {
            // debug!("Fails step size: {}, {}", levels[i-1], levels[i]);
            return false;
        }
        if levels[i-1] > levels[i] {
            if state == 0 {
                state = 2;
            } else if state == 1 {
                // debug!("Fails dec.");
                return false;
            }
        } else {
            if state == 0 {
                state = 1;
            } else if state == 2 {
                // debug!("Fails inc.");
                return false;
            }
        }
    }
    return true;
}

#[allow(dead_code)]
pub fn is_safe_enough_levels(levels: Vec<i32>) -> bool {
    let mut state = 0; // 0 = unset, 1 = inc, 2 = dec
    let mut maybe_unsafe = 0; // 0 = safe, 1 = fault, 2 = unsafe
    let mut last_state = levels[0];
    for i in 1..levels.len() {
        if (last_state - levels[i]).abs() > 3 || last_state == levels[i] {
            if maybe_unsafe >= 1 {
                // debug!("Fails step size: {}, {}", last_state, levels[i]);
                return false;
            } else {
                maybe_unsafe += 1;
            }
        } else if last_state > levels[i] {
            if state == 0 {
                state = 2;
                last_state = levels[i];
            } else if state == 1 {
                // debug!("Fails dec.");
                if maybe_unsafe >= 1 {
                    return false;
                } else {
                    maybe_unsafe += 1;
                }
            } else {
                last_state = levels[i];
            }
        } else {
            if state == 0 {
                state = 1;
                last_state = levels[i];
            } else if state == 2 {
                // debug!("Fails inc.");
                if maybe_unsafe >= 1 {
                    return false;
                } else {
                    maybe_unsafe += 1;
                }
            } else {
                last_state = levels[i];
            }
        }
    }
    return true;
}

pub fn is_really_safe_enough(levels: Vec<i32>) -> bool {
    if is_safe_levels(levels.clone()) {
        return true;
    }
    // Gonna get messy...
    for idx in 0..levels.len() {
        let mut levels_copy = levels.clone();
        levels_copy.remove(idx);
        if is_safe_levels(levels_copy) {
            return true; // Found one!
        }
    }

    return false;
}

/**
 * Problem #02, Part 1
 */
pub fn problem_021(input: Vec<String>) -> RetType {
    return RetType::U32(
        parse_levels(input)
            .into_iter()
            .map(|x| if is_safe_levels(x) {1} else {0})
            .sum()
    )
}

/**
 * Problem #02, Part 2
 */
pub fn problem_022(input: Vec<String>) -> RetType {
    return RetType::U32(
        parse_levels(input)
            .into_iter()
            .map(|x| if is_really_safe_enough(x) {1} else {0})
            .sum()
    )
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
    fn test_02_parse_levels() {
        let input_str = vec!(
            "7 6 4 2 1".to_string(),
            "1 2 7 8 9".to_string(),
            "9 7 6 2 1".to_string(),
            "1 3 2 4 5".to_string(),
            "8 6 4 4 1".to_string(),
            "1 3 6 7 9".to_string()
        );

        let res = parse_levels(input_str);

        assert_eq!(res.clone(), vec!(
            vec!(7, 6, 4, 2, 1),
            vec!(1, 2, 7, 8, 9),
            vec!(9, 7, 6, 2, 1),
            vec!(1, 3, 2, 4, 5),
            vec!(8, 6, 4, 4, 1),
            vec!(1, 3, 6, 7, 9),
        ));

        assert_eq!(is_safe_levels(res[0].clone()), true);
        assert_eq!(is_safe_levels(res[1].clone()), false);
        assert_eq!(is_safe_levels(res[2].clone()), false);
        assert_eq!(is_safe_levels(res[3].clone()), false);
        assert_eq!(is_safe_levels(res[4].clone()), false);
        assert_eq!(is_safe_levels(res[5].clone()), true);
    }

    #[test]
    fn test_part021() {
        init();
        let input_str = vec!(
            "7 6 4 2 1".to_string(),
            "1 2 7 8 9".to_string(),
            "9 7 6 2 1".to_string(),
            "1 3 2 4 5".to_string(),
            "8 6 4 4 1".to_string(),
            "1 3 6 7 9".to_string()
        );

        assert_eq!(problem_021(input_str), RetType::U32(2));
    }

    #[test]
    fn test_maybe_safe() {
        assert_eq!(is_safe_enough_levels(vec!(7, 6, 4, 2, 1)), true);
        assert_eq!(is_safe_enough_levels(vec!(1, 2, 7, 8, 9)), false);
        assert_eq!(is_safe_enough_levels(vec!(9, 7, 6, 2, 1)), false);
        assert_eq!(is_safe_enough_levels(vec!(1, 3, 2, 4, 5)), true);
        assert_eq!(is_safe_enough_levels(vec!(8, 6, 4, 4, 1)), true);
        assert_eq!(is_safe_enough_levels(vec!(1, 3, 6, 7, 9)), true);
    }

    #[test]
    fn test_really_maybe_safe() {
        assert_eq!(is_really_safe_enough(vec!(7, 6, 4, 2, 1)), true);
        assert_eq!(is_really_safe_enough(vec!(1, 2, 7, 8, 9)), false);
        assert_eq!(is_really_safe_enough(vec!(9, 7, 6, 2, 1)), false);
        assert_eq!(is_really_safe_enough(vec!(1, 3, 2, 4, 5)), true);
        assert_eq!(is_really_safe_enough(vec!(8, 6, 4, 4, 1)), true);
        assert_eq!(is_really_safe_enough(vec!(1, 3, 6, 7, 9)), true);
    }

    #[test]
    fn test_part022() {
        init();
        let input_str = vec!(
            "7 6 4 2 1".to_string(),
            "1 2 7 8 9".to_string(),
            "9 7 6 2 1".to_string(),
            "1 3 2 4 5".to_string(),
            "8 6 4 4 1".to_string(),
            "1 3 6 7 9".to_string()
        );

        assert_eq!(problem_022(input_str), RetType::U32(4));
    }

}