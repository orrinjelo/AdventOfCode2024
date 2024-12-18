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

pub struct ThreeBit {
    reg_a: u128,
    reg_b: u128,
    reg_c: u128,
    pc: usize,
    program: Vec<u8>,
    output: Vec<u8>,
}

impl ThreeBit {
    pub fn new(input: Vec<String>) -> ThreeBit {
        // Register A: 729
        // Register B: 0
        // Register C: 0

        // Program: 0,1,5,4,3,0
        // 12, 9

        ThreeBit {
            reg_a: (input.clone()[0])[12..].parse::<u128>().unwrap(),
            reg_b: (input.clone()[1])[12..].parse::<u128>().unwrap(),
            reg_c: (input.clone()[2])[12..].parse::<u128>().unwrap(),
            pc: 0,
            program: (input[4])[9..].split(',').into_iter().map(|x| x.parse::<u8>().unwrap()).collect(),
            output: Vec::new(),
        }
    }

    pub fn interp(&mut self) -> bool {
        if self.pc >= self.program.len() {
            return false;
        }
        let opcode = self.program[self.pc];
        let mut operand = self.program[self.pc+1] as u128;
        let mut combo_operand = || {
            if operand > 3 {
                if operand == 4 {
                    operand = self.reg_a;
                } else if operand == 5 {
                    operand = self.reg_b;
                } else if operand == 6 {
                    operand = self.reg_c;
                } else {
                    error!("Invalid operand.");
                }
            }
        };
        // debug!("a:{} b:{} c:{}", self.reg_a, self.reg_b, self.reg_c);
        // debug!("op:{} and:{}", opcode, 0);
        if opcode == 0 {
            // adv
            combo_operand();
            self.reg_a = self.reg_a / 2u128.pow(operand as u32);
            self.pc += 2;
        } else if opcode == 1 {
            // bxl
            self.reg_b = self.reg_b ^ operand;
            self.pc += 2;
        } else if opcode == 2 {
            // bst
            combo_operand();
            self.reg_b = operand % 8;
            self.pc += 2;
        } else if opcode == 3 {
            // jnz
            if self.reg_a != 0 {
                self.pc = operand as usize;
            } else {
                self.pc += 2;
            }
        } else if opcode == 4 {
            // bxc
            self.reg_b = self.reg_b ^ self.reg_c;
            self.pc += 2;
        } else if opcode == 5 {
            // out
            combo_operand();
            self.output.push(operand as u8 % 8);
            self.pc += 2;
        } else if opcode == 6 {
            // bdv
            combo_operand();
            self.reg_b = self.reg_a / 2u128.pow(operand as u32);
            self.pc += 2;
        } else if opcode == 7 {
            // cdv
            combo_operand();
            self.reg_c = self.reg_a / 2u128.pow(operand as u32);
            self.pc += 2;            
        } else {
            error!("Invalid op code.");
            return false;
        }
        return true;
    }

    pub fn reset(&mut self, reg_a: u128) {
        self.output = Vec::new();
        self.pc = 0;
        self.reg_a = reg_a;
        self.reg_b = 0;
        self.reg_c = 0;
    }
}

// pub fn tree_me(tb: &mut ThreeBit, starting_value: u64, best: u64) -> Option<u64> {
//     // Get a starting poing
//     tb.reset(starting_value);
//     while tb.interp() {
//         if tb.output != tb.program[0..tb.output.len()] {
//             break;
//         }
//     }

//     if tb.output == tb.program {
//         return Some(starting_value);
//     }

//     let mut current_best = tb.output.len()-1;

//     if best < current_best {
//         let delta = 2.pow(((starting_value as f32).log2()).floor() as u32);
        
//     }
    
//     None
// }

pub fn problem_171(input: Vec<String>) -> RetType {
    let mut tb = ThreeBit::new(input);

    while tb.interp() {
    }

    return RetType::STRING(format!("{:?}", tb.output));
}

pub fn problem_172(input: Vec<String>) -> RetType {
    let mut tb = ThreeBit::new(input);
    
    let mut init_value = 6995444u128; // arbitrary starting value, ofc
    let mut pow = 23u32;
    let mut best = 5;
    while tb.output != tb.program {
        let delta = 2u128.pow(pow);
        tb.reset(init_value + delta);
        while tb.interp() {
            if tb.output != tb.program[0..tb.output.len()] {
                debug!("best output: {:?}", tb.output);
                if tb.output.len() - 1 > best {
                    best = tb.output.len() - 1;
                    init_value += delta;
                }
                break;
            } 
        }
        pow += 1;
    }

    return RetType::U128(init_value);
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
    fn test_3bit() {
        init();

        let input = vec![
            "Register A: 729".to_string(),
            "Register B: 0".to_string(),
            "Register C: 0".to_string(),
            "".to_string(),
            "Program: 0,1,5,4,3,0".to_string(),
        ];

        let mut tb = ThreeBit::new(input);

        assert_eq!(tb.reg_a, 729);
        assert_eq!(tb.reg_b, 0);
        assert_eq!(tb.reg_c, 0);
        assert_eq!(tb.program, vec![0, 1, 5, 4, 3, 0]);

        while tb.interp() {
        }

        assert_eq!(tb.output, vec![4,6,3,5,6,3,5,2,1,0]);
    }

    #[test]
    fn test_3bit_part2() {
        let input = vec![
            "Register A: 2024".to_string(),
            "Register B: 0".to_string(),
            "Register C: 0".to_string(),
            "".to_string(),
            "Program: 0,3,5,4,3,0".to_string(),
        ];
        assert_eq!(problem_172(input), RetType::I32(117440));
    }
}