use log::{trace, debug, info, warn, error}; // trace, debug, info, warn, error
use crate::util::RetType;
use regex::Regex;
use ndarray::prelude::*;
use ndarray_linalg::Solve;
// use std::{cmp::Ordering::{Equal, Greater, Less}, fmt};

#[allow(dead_code)]
fn _get_rid_of_log_unused_import_warnings() {
    trace!("Example trace.");
    debug!("Example debug.");
    info!("Example info.");
    warn!("Example warn.");
    error!("Example error.");
}

#[derive(Clone, Debug, PartialEq)]
struct ClawMachine {
    button_a: (u32, u32),
    button_b: (u32, u32), 
    prize: (u32, u32),
}

impl ClawMachine {
    pub fn new(a: String, b: String, c: String) -> ClawMachine {
        let re_a = Regex::new(r"Button A: X+(\d+), Y+(\d+)").unwrap();
        let re_b = Regex::new(r"Button B: X+(\d+), Y+(\d+)").unwrap();
        let re_c = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();

        let cap_a = re_a.captures(&a).unwrap();
        let cap_b = re_b.captures(&b).unwrap();
        let cap_c = re_c.captures(&c).unwrap();
        
        ClawMachine {
            button_a: (cap_a[1].parse::<u32>().unwrap(), cap_a[2].parse::<u32>().unwrap()),
            button_b: (cap_b[1].parse::<u32>().unwrap(), cap_b[2].parse::<u32>().unwrap()),
            prize: (cap_c[1].parse::<u32>().unwrap(), cap_c[2].parse::<u32>().unwrap()),
        }
    }

    pub fn solve(&self) -> (u32, u32) {
        let a: Array2<u32> = array![[self.button_a.0, self.button_a.1], [self.button_b.0, self.button_b.1]];
        let b: Array1<u32> = array![self.prize.0, self.prize.1];
        let x = a.solve_into(b).unwrap();
        (x.0, x.1)
    }
}

pub fn parse_input(input: Vec<String>) -> Vec<ClawMachine> {
    let mut cmv = Vec::new();
    for i in 0..input.len()/4 {
        cmv.push(ClawMachine::new(input[0 + i*4].clone(), input[1 + i*4].clone(), input[2 + i*4].clone()));
    }
    cmv
}


pub fn problem_131(input: Vec<String>) -> RetType {
    let claws = parse_input(input);
    
    return RetType::U32(0);
}

pub fn problem_132(input: Vec<String>) -> RetType {
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
    fn test_solve() {
        init();

        let input = vec![
            "Button A: X+94, Y+34".to_string();
            "Button B: X+22, Y+67".to_string();
            "Prize: X=8400, Y=5400".to_string();
            "".to_string();
            "Button A: X+26, Y+66".to_string();
            "Button B: X+67, Y+21".to_string();
            "Prize: X=12748, Y=12176".to_string();
            "".to_string();
            "Button A: X+17, Y+86".to_string();
            "Button B: X+84, Y+37".to_string();
            "Prize: X=7870, Y=6450".to_string();
            "".to_string();
            "Button A: X+69, Y+23".to_string();
            "Button B: X+27, Y+71".to_string();
            "Prize: X=18641, Y=10279".to_string();
        ];

        let claws = parse_input(input);

        assert_eq!(claws.solve(), (94, 34));

   }

}