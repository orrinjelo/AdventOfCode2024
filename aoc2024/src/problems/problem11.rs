use log::{trace, debug, info, warn, error}; // trace, debug, info, warn, error
use crate::util::RetType;
// use std::collections::{HashMap};
// use std::fmt;

#[allow(dead_code)]
fn _get_rid_of_log_unused_import_warnings() {
    trace!("Example trace.");
    debug!("Example debug.");
    info!("Example info.");
    warn!("Example warn.");
    error!("Example error.");
}

pub fn parse_input(input: Vec<String>) -> Vec<u64> {
    input[0].trim().split(' ')
            .into_iter()
            .map(|x| x.parse::<u64>().unwrap() )
            .collect()
}

pub fn blink(input: Vec<u64>) -> Vec<u64> {
    let mut output: Vec<u64> = Vec::new();

    for rock in input {
        if rock == 0 {
            output.push(1);
        } else if (rock as f64).log10().floor() as u64 % 2 == 1 {
            // n    ||  k = |_Log10_|  ||  (k+1)/2  || 10^(k+1)/2
            // 22           1                1            10
            // 4201         3                2            100
            // 765432       5                3
            let k = (rock as f64).log10().floor() as u64;
            let r = (k+1)/2;
            let m: u64 = 10u64.pow(r as u32);
            output.push( rock / m );
            output.push( rock % m );
        } else {
            output.push(rock*2024);
        }
    }

    output
}

// #[derive(Clone, Debug, PartialEq)]
// enum Stone {
//     SINGLE(u64),
//     DOUBLE((u64, u64)),
// }

// #[derive(Clone, Debug, PartialEq)]
// struct Blinkenator {
//     cache: HashMap<u64, u64>,
//     start: Vec<u64>,
// }

// impl Blinkenator {
//     pub fn new(v: Vec<u64>) -> Blinkenator {
//         let mut b = Blinkenator {
//             cache: HashMap::new(),
//             start: v,
//         };
//         b
//     }

//     pub fn blinken(&mut self, blinks: usize) -> u64 {
//         let mut stones = Vec::new();
//         let mut hist = Vec::new();
//         let mut current = Vec::new();
//         let mut to_do = Vec::new();

//         for n in self.start {
//             if self.cache.contains_key(&n) {
//                 current[*self.cache.get(&n).unwrap() as usize] += 1;
//             } else {
//                 self.cache.insert(n, self.cache.len() as u64);
//                 to_do.push(n);
//                 current.push(1);
//             }
//         }

//         for _ in 0..blinks {
//             let nums = to_do.clone();
//             to_do = Vec::new();

//             let mut i = |n| {
//                 let size = self.cache.len();
//                 *self.cache.entry(n).or_insert_with(|| {
//                     to_do.push(n);
//                     size
//                 })
//             };

//             for n in nums {
//                 let (a, b) = if n == 0 { 
//                     (i(1), usize::MAX)
//                 } else {
//                     let digs = n.ilog10() + 1;
//                     if digs % 2 == 0 {

//                     }
//                 }
//             }
//         }

//         0
//     }

//     pub fn digest(&mut self) {
//         for rock in 1..500_000 {
//             let res = blink(vec![rock]);
//             if res.len() == 1 {
//                 self.cache.insert(rock,  Stone::SINGLE(res[0]));    
//             } else {
//                 self.cache.insert(rock,  Stone::DOUBLE((res[0], res[1])));    
//             }
//         }
//     }
// }
//     pub fn blinken_weiter(&mut self, rock: u64, num: usize) -> u64 {
//         if self.cache.contains_key(&rock) {
//             return *self.cache.get(&rock).unwrap();
//         }
//     }
// }

pub fn problem_111(input: Vec<String>) -> RetType {
    let mut v = parse_input(input);
        
    for _ in 0..25 {
        v = blink(v);
    }

    return RetType::U32(v.len() as u32);
}

pub fn problem_112(_input: Vec<String>) -> RetType {
    return RetType::U32(0);
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
            "125 17".to_string(),
        ];

        let mut v = parse_input(input);
        
        for _ in 0..25 {
            v = blink(v);
        }

        assert_eq!(v.len(), 55312);
    }

    #[test]
    fn test_more_blinken() {
        init();

        // let input = vec![
        //     "125 17".to_string(),
        // ];

        // let v = parse_input(input);
        // let mut blinkenator = Blinkenator::new(v.clone());

        // let mut count = 0u64;
        // for val in v.clone() {
        //     let x = blinkenator.blinken(val, 25);
        //     count += x;
        // }

        // assert_eq!(count, 55312);

        // let input = vec![
        //     "125 17".to_string(),
        // ];

        // let mut v = parse_input(input);
        
        // // let mut count = 0u64;
        // // for val in v {
        // //     count += blinkenator.blinken(val, 75);
        // // }
        // let mut blinkenator = Blinkenator::new(v.clone());
        // blinkenator.digest();
        
        // debug!("i: {}, count: {}", 25, blinkenator.blinken(125, 25));

        // debug!("{:?}", blinkenator.cache);
    }
}