use std::cmp::Ordering::{Equal, Less, Greater};
use std::cmp::Ordering;
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

pub fn parse_print_order(input: Vec<String>) -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
    let mut imode = false;
    let mut page_order: Vec<(u32, u32)> = Vec::new();
    let mut updates: Vec<Vec<u32>> = Vec::new();
    for line in input {
        if line == "".to_string() {
            imode = true;
        } else if !imode {
            let pages = line.split('|')
                        .into_iter()
                        .map(|x| x.parse::<u32>().unwrap())
                        .collect::<Vec<u32>>();
            let (a, b) = (pages[0], pages[1]);
            page_order.push((a,b));
        } else if imode {
            updates.push(
                line.split(',')
                    .into_iter()
                    .map(|x| x.parse::<u32>().unwrap())
                    .collect()
            );
        }
    }
    (page_order, updates)
}


#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
struct ElfNumber {
    val: u32
}

impl ElfNumber {
    pub fn new(x: u32) -> ElfNumber {
        ElfNumber {
            val: x,
        }
    }

    pub fn compare(&self, b: &ElfNumber, digest: Vec<(u32, u32)>) -> Ordering {
        if *self == *b {
            return Equal;
        }
        for entry in digest {
            if self.val == entry.0 && b.val == entry.1 {
                return Less;
            } else if self.val == entry.1 && b.val == entry.0 {
                return Greater;
            }
        }
        return Equal; // unknown
    }
}

pub fn sort_elfwise(v: Vec<u32>, digest: Vec<(u32, u32)>) -> Vec<u32> {
    let mut elfv = v.into_iter()
     .map(|x| ElfNumber::new(x))
     .collect::<Vec<ElfNumber>>();
    elfv.sort_by(|a, b| a.compare(b, digest.clone()));
    elfv.into_iter()
     .map(|x| x.val)
     .collect()
}

pub fn is_valid_order(v: Vec<u32>, digest: Vec<(u32, u32)>) -> bool {
    let left = sort_elfwise(v.clone(), digest);
    // debug!("sorted: {:?}, v: {:?}", left.clone(), v.clone());
    left == v
}

pub fn problem_051(input: Vec<String>) -> RetType {
    let (po, u) = parse_print_order(input);
    let mut res = 0u32;
    for ubi in u {
        if is_valid_order(ubi.clone(), po.clone()) {
            res += ubi[(ubi.len() / 2) as usize];
        }
    }
    return RetType::U32(res);
}

pub fn problem_052(input: Vec<String>) -> RetType {
    let (po, u) = parse_print_order(input);
    let mut res = 0u32;
    for ubi in u {
        if !is_valid_order(ubi.clone(), po.clone()) {
            let rubi = sort_elfwise(ubi.clone(), po.clone());
            res += rubi[(ubi.len() / 2) as usize];
        }
    }
    return RetType::U32(res);
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
    fn test_parse_input() {
        init();
        let input = vec!(
            "47|53".to_string(),
            "97|13".to_string(),
            "97|61".to_string(),
            "97|47".to_string(),
            "75|29".to_string(),
            "61|13".to_string(),
            "75|53".to_string(),
            "29|13".to_string(),
            "97|29".to_string(),
            "53|29".to_string(),
            "61|53".to_string(),
            "97|53".to_string(),
            "61|29".to_string(),
            "47|13".to_string(),
            "75|47".to_string(),
            "97|75".to_string(),
            "47|61".to_string(),
            "75|61".to_string(),
            "47|29".to_string(),
            "75|13".to_string(),
            "53|13".to_string(),
            "".to_string(),
            "75,47,61,53,29".to_string(),
            "97,61,53,29,13".to_string(),
            "75,29,13".to_string(),
            "75,97,47,61,53".to_string(),
            "61,13,29".to_string(),
            "97,13,75,29,47".to_string()
        );

        let (po, u) = parse_print_order(input);

        // debug!("po: {:?}", po);
        // debug!("u: {:?}", u);

        assert_eq!(is_valid_order(u[0].clone(), po.clone()), true);
        assert_eq!(is_valid_order(u[1].clone(), po.clone()), true);
        assert_eq!(is_valid_order(u[2].clone(), po.clone()), true);
        assert_eq!(is_valid_order(u[3].clone(), po.clone()), false);
        assert_eq!(is_valid_order(u[4].clone(), po.clone()), false);
        assert_eq!(is_valid_order(u[5].clone(), po.clone()), false);

    }

}