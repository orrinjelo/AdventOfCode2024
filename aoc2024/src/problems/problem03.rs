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

pub fn find_mults(input: String) -> u64 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut results = vec![];
    for (_, [a, b]) in re.captures_iter(&input).map(|c| c.extract()) {
        // debug!("a: {}, b: {}", a, b);
        results.push((a.parse::<u64>().unwrap(), b.parse::<u64>().unwrap()));
    }

    // debug!("Results: {:?}", results);

    results.into_iter()
           .map(|(a,b)| a * b)
           .sum()
}


pub fn find_mults_and_dos(input: String) -> u64 {
    let re = Regex::new(r"(?:mul\((\d+,\d+)\))|(?:(do)\(\))|(?:(don)'t\(\))").unwrap();
    let mut results = vec![];
    let mut listen = true;
    for (_, [msg]) in re.captures_iter(&input).map(|c| c.extract()) {
        // debug!("msg: {}", msg);
        if msg == "do" {
            listen = true;
        } else if msg == "don" {
            listen = false;
        } else if listen {
            // debug!("msg: {}", msg);
            let mut parts = msg.split(',');
            // debug!("parts: {:?}", parts);
            let a = parts.next().unwrap();
            let b = parts.next().unwrap();
            // debug!("a: {}, b: {}", a, b);
            results.push((a.parse::<u64>().unwrap(), b.parse::<u64>().unwrap()));
        }
    }

    // debug!("Results: {:?}", results);

    results.into_iter()
           .map(|(a,b)| a * b)
           .sum()
}

pub fn problem_031(input: Vec<String>) -> RetType {
    return RetType::U64(
        input.into_iter()
             .map(|x| find_mults(x))
             .sum()
    );
}

pub fn problem_032(input: Vec<String>) -> RetType {
    return RetType::U64(
        input.into_iter()
             .map(|x| find_mults_and_dos(x))
             .sum()
    );
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
    fn test_find_mults() {
        init();

        assert_eq!(find_mults("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))".to_string()), 161);
    }

    #[test]
    fn test_find_mults_dos() {
        init();

        assert_eq!(find_mults_and_dos("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))".to_string()), 48);
    }

}