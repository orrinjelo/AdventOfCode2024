use log::{trace, debug, info, warn, error}; // trace, debug, info, warn, error
use crate::util::RetType;
use regex::Regex;

#[allow(dead_code)]
fn _get_rid_of_log_unused_import_warnings() {
    trace!("Example trace.");
    debug!("Example debug.");
    info!("Example info.");
    warn!("Example warn.");
    error!("Example error.");
}

#[derive(Clone, Debug, PartialEq)]
pub struct ClawMachine {
    button_a: (u32, u32),
    button_b: (u32, u32), 
    prize: (u64, u64),
}

impl ClawMachine {
    pub fn new(a: String, b: String, c: String) -> ClawMachine {
        let re_a = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)").unwrap();
        let re_b = Regex::new(r"Button B: X\+(\d+), Y\+(\d+)").unwrap();
        let re_c = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();

        // debug!("{}", a);
        // debug!("{}", b);
        // debug!("{}", c);

        let cap_a = re_a.captures(&a).unwrap();
        let cap_b = re_b.captures(&b).unwrap();
        let cap_c = re_c.captures(&c).unwrap();
        
        ClawMachine {
            button_a: (cap_a[1].parse::<u32>().unwrap(), cap_a[2].parse::<u32>().unwrap()),
            button_b: (cap_b[1].parse::<u32>().unwrap(), cap_b[2].parse::<u32>().unwrap()),
            prize: (cap_c[1].parse::<u64>().unwrap(), cap_c[2].parse::<u64>().unwrap()),
        }
    }

    /**
     * A = (a.0, b.0)  inv(A) = det^-1 * (b.1, -b.0)   (x) = inv(A) * (prize.0)   = det^-1 * ( b.1*prize.0 - b.0*prize.1)
     *     (a.1, b.1)                    (-a.1, a.0)   (y)            (prize.1)              ( a.0*prize.1 - a.1*prize.0)
     */
    #[allow(dead_code)]
    #[allow(unused_parens)]
    pub fn solve(&self) -> (u64, u64) {
        let tol = 0.01;
        let det = 1. / ((self.button_a.0*self.button_b.1) as f64 - (self.button_a.1*self.button_b.0) as f64);
        // debug!("det: {}", det);
        let xf = (((self.button_b.1 as u64 * self.prize.0) as f64 - (self.button_b.0 as u64 * self.prize.1) as f64) * det);
        let mut x = xf.round() as u64;
        let yf = (((self.button_a.0 as u64 * self.prize.1) as f64 - (self.button_a.1 as u64 * self.prize.0) as f64) * det);
        let mut y = yf.round() as u64;
        if (xf - x as f64).abs() > tol || (yf - y as f64).abs() > tol {
            x = 0;
            y = 0;
        }
        (x,y)
    }

    pub fn dumb_solve(&self) -> (u64, u64) {
        let mut lowest = (100, 100);
        for a in 0..101 {
            for b in 0..101 {
                if self.button_a.0 as u64 * a + self.button_b.0 as u64 * b == self.prize.0 && self.button_a.1 as u64 * a + self.button_b.1 as u64 * b == self.prize.1 {
                    if lowest.0*3 + lowest.1 > a*3 + b {
                        lowest = (a, b)
                    }
                }
            }
        }
        if lowest == (100, 100) {
            return (0, 0)
        }
        lowest
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
    let mut count = 0;
    let res = claws.clone().into_iter().map(|x| {
        let sol = x.dumb_solve();
        if sol != (0,0) {
            count += 1;
        }
        sol.0 * 3 + sol.1 * 1
    }).sum::<u64>();
    debug!("Valid entries: {}/{}", count, claws.len());
    return RetType::U64(res);
}

pub fn problem_132(input: Vec<String>) -> RetType {
    let claws = parse_input(input);
    let res = claws.clone().into_iter().map(|mut x| {
        x.prize.0 += 10000000000000;
        x.prize.1 += 10000000000000;
        let sol = x.solve();
        sol.0 * 3 + sol.1 * 1
    }).sum::<u64>();

    return RetType::U64(res);
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
            "Button A: X+94, Y+34".to_string(),
            "Button B: X+22, Y+67".to_string(),
            "Prize: X=8400, Y=5400".to_string(),
            "".to_string(),
            "Button A: X+26, Y+66".to_string(),
            "Button B: X+67, Y+21".to_string(),
            "Prize: X=12748, Y=12176".to_string(),
            "".to_string(),
            "Button A: X+17, Y+86".to_string(),
            "Button B: X+84, Y+37".to_string(),
            "Prize: X=7870, Y=6450".to_string(),
            "".to_string(),
            "Button A: X+69, Y+23".to_string(),
            "Button B: X+27, Y+71".to_string(),
            "Prize: X=18641, Y=10279".to_string(),
            "".to_string(),
        ];

        let claws = parse_input(input);

        assert_eq!(claws[0].solve(), (80, 40)); // 80*3 and 40*1
        assert_eq!(claws[1].solve(), (0, 0));
        assert_eq!(claws[2].solve(), (38, 86));
        assert_eq!(claws[3].solve(), (0, 0));

        assert_eq!(claws[0].dumb_solve(), (80, 40)); // 80*3 and 40*1
        assert_eq!(claws[1].dumb_solve(), (0, 0));
        assert_eq!(claws[2].dumb_solve(), (38, 86));
        assert_eq!(claws[3].dumb_solve(), (0, 0));

   }

}