use log::{trace, debug, info, warn, error}; // trace, debug, info, warn, error
use crate::util::RetType;

#[allow(unused_macros)]
#[macro_export]
macro_rules! ifelse2 {
    ($c:expr, $v:expr, $v1:expr) => {
        if $c {$v} else {$v1}
    };
}

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

pub fn tree_me(tb: &mut ThreeBit, starting_value: u128) -> Option<u128> {
    // Get a starting poing
    tb.reset(starting_value);
    while tb.interp() {
        if tb.output != tb.program[0..tb.output.len()] {
            break;
        }
    }

    if tb.output == tb.program {
        return Some(starting_value);
    }

    let orig_len = tb.output.len();
    let orig = tb.output.clone();

    // Try 8 values
    let pow = starting_value.ilog2();
    let mut history = Vec::new();
    for k in 1..33 {
        // 1 = 2^(pow+1)
        // 2 = 2^(pow+2)
        // 3 = 2^(pow+1)+2^(pow+2)
        // ...
        // 5 => 1, 4
        // 6 => 2, 4
        // 7 => 1, 2, 4
        // 8 => 8
        let mut delta = ifelse2!((k+1)%2==1,1,0) * 2u128.pow(pow+1);
        delta += ifelse2!((k+1)&0x2!=0,1,0) * 2u128.pow(pow+2);
        delta += ifelse2!((k+1)&0x4!=0,1,0) * 2u128.pow(pow+3);
        delta += ifelse2!((k+1)&0x8!=0,1,0) * 2u128.pow(pow+4);
        delta += ifelse2!((k+1)&0x10!=0,1,0) * 2u128.pow(pow+5);
        delta += ifelse2!((k+1)&0x20!=0,1,0) * 2u128.pow(pow+6);
        delta += ifelse2!((k+1)&0x40!=0,1,0) * 2u128.pow(pow+7);
        delta += ifelse2!((k+1)&0x80!=0,1,0) * 2u128.pow(pow+8);
        tb.reset(starting_value + delta);
        while tb.interp() {
            if tb.output != tb.program[0..tb.output.len()] {
                break;
            }
        }
        if tb.output == tb.program {
            return Some(starting_value + delta);
        } else {
            history.push((delta, tb.output.clone()));
        }
    }

    // If we haven't found the one...choose the best to start from.
    let mut best = (0u128, vec![]);

    for entry in history.clone() {
        if entry.1.len() > best.1.len() && entry.1.len() > orig_len {
            best = entry;
        }
    }

    if best.0 == 0u128 {
        debug!("Current: {:?}", orig);
        debug!("Failure: {:?}", history);
        None
    } else {
        tree_me(tb, starting_value + best.0)
    }
    
}

pub fn problem_171(input: Vec<String>) -> RetType {
    let mut tb = ThreeBit::new(input);

    while tb.interp() {
    }

    return RetType::STRING(format!("{:?}", tb.output));
}

pub fn problem_172(input: Vec<String>) -> RetType {
    let mut tb = ThreeBit::new(input);
    
    let mut init_value = 48628u128; // arbitrary starting value, ofc

    let res = tree_me(&mut tb, init_value);

    return RetType::U128(res.unwrap());
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

    // #[test]
    // fn test_3bit() {
    //     init();

    //     let input = vec![
    //         "Register A: 729".to_string(),
    //         "Register B: 0".to_string(),
    //         "Register C: 0".to_string(),
    //         "".to_string(),
    //         "Program: 0,1,5,4,3,0".to_string(),
    //     ];

    //     let mut tb = ThreeBit::new(input);

    //     assert_eq!(tb.reg_a, 729);
    //     assert_eq!(tb.reg_b, 0);
    //     assert_eq!(tb.reg_c, 0);
    //     assert_eq!(tb.program, vec![0, 1, 5, 4, 3, 0]);

    //     while tb.interp() {
    //     }

    //     assert_eq!(tb.output, vec![4,6,3,5,6,3,5,2,1,0]);
    // }

    // #[test]
    // fn test_3bit_part2() {
    //     let input = vec![
    //         "Register A: 2024".to_string(),
    //         "Register B: 0".to_string(),
    //         "Register C: 0".to_string(),
    //         "".to_string(),
    //         "Program: 0,3,5,4,3,0".to_string(),
    //     ];
    //     assert_eq!(problem_172(input), RetType::I32(117440));
    // }
}