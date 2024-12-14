use log::{trace, debug, info, warn, error}; // trace, debug, info, warn, error
use crate::util::RetType;
use std::fmt::Debug;
use std::fs::{OpenOptions, File};
use std::io::prelude::*;
// use regex::Regex;

#[allow(dead_code)]
fn _get_rid_of_log_unused_import_warnings() {
    trace!("Example trace.");
    debug!("Example debug.");
    info!("Example info.");
    warn!("Example warn.");
    error!("Example error.");
}

#[derive(Clone, Debug, PartialEq)]
pub struct Robit {
    pos: (i32, i32),
    vel: (i32, i32),
}

pub fn parse_input(input: Vec<String>) -> Vec<Robit> {
    let mut robits = Vec::new();
    for line in input {
        let parts = line.split(' ').into_iter().collect::<Vec<&str>>();
        let pospart = parts[0].split('=').collect::<Vec<&str>>();
        let velpart = parts[1].split('=').collect::<Vec<&str>>();
        let poscoords = pospart[1].split(',').map(|x| x.parse::<i32>().unwrap() ).collect::<Vec<i32>>();
        let velcoords = velpart[1].split(',').map(|x| x.parse::<i32>().unwrap() ).collect::<Vec<i32>>();
        robits.push(
            Robit {
                pos: (poscoords[0], poscoords[1]),        
                vel: (velcoords[0], velcoords[1]),
            }
        );
    }
    robits
}

pub fn move_robits(robits: &mut Vec<Robit>, width: i32, height: i32) {    
    robits.into_iter()
          .for_each(|f| {
              f.pos = ((f.pos.0 + f.vel.0) % width, (f.pos.1 + f.vel.1) % height);
              if f.pos.0 < 0 {
                f.pos.0 += width;
              }
              if f.pos.1 < 0 {
                f.pos.1 += height;
              }
          });
}

pub fn write_robits(robits: Vec<Robit>, width: i32, height: i32, step: usize, file: &mut File) {
    let mut grid = vec![vec![0;height as usize];width as usize];
    robits.into_iter()
          .for_each(|f| {
            grid[f.pos.0 as usize][f.pos.1 as usize] = 1;
          });
    file.write("Step ".to_string().as_bytes()).unwrap();
    file.write(step.to_string().as_bytes()).unwrap();
    file.write("\n".to_string().as_bytes()).unwrap();
    grid.into_iter()
        .for_each(|row| 
            {
                file.write(
                    row.into_iter().map(|c| if c == 0 {" ".to_string()} else {"X".to_string()}).collect::<String>().as_bytes()
                ).unwrap();
                file.write("\n".to_string().as_bytes()).unwrap();
            }
        );
    file.write("\n".to_string().as_bytes()).unwrap();
}

pub fn calc_quads(robits: Vec<Robit>, width: i32, height: i32) -> i64 {
    let mut i = 0;
    let mut ii = 0;
    let mut iii = 0;
    let mut iv = 0;

    robits.clone().into_iter()
          .for_each(|f| {
              if f.pos.0 > width/2 {
                if f.pos.1 > height/2 {
                    i += 1;
                } else if f.pos.1 < height/2 {
                    iv += 1;
                }
              } else if f.pos.0 < width/2 {
                if f.pos.1 > height/2 {
                    ii += 1;
                } else if f.pos.1 < height/2{
                    iii += 1;
                }
              }
          });
    // debug!("robits: {:?}", robits);
    // debug!("i:{}, ii:{}, iii:{}, iv:{}", i, ii, iii, iv);
    i*ii*iii*iv
}

pub fn problem_141(input: Vec<String>) -> RetType {
    let width = 101;
    let height = 103;

    let mut robits = parse_input(input);
    for _ in 0..100 {
        move_robits(&mut robits, width, height);
    }
    return RetType::I64(calc_quads(robits, width, height));
}

pub fn problem_142(input: Vec<String>) -> RetType {
    let mut data_file = File::create("hohoho.txt").expect("creation failed");
    let mut robits = parse_input(input);
    let width = 101;
    let height = 103;
    for i in 0..10000 {
        move_robits(&mut robits, width, height);

        if i % 101 == 6 {
            write_robits(robits.clone(), width, height, i, &mut data_file);
        }
    }

    return RetType::U64(8087);
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
    fn test_robits() {
        init();

        let input = vec![
            "p=0,4 v=3,-3".to_string(),
            "p=6,3 v=-1,-3".to_string(),
            "p=10,3 v=-1,2".to_string(),
            "p=2,0 v=2,-1".to_string(),
            "p=0,0 v=1,3".to_string(),
            "p=3,0 v=-2,-2".to_string(),
            "p=7,6 v=-1,-3".to_string(),
            "p=3,0 v=-1,-2".to_string(),
            "p=9,3 v=2,3".to_string(),
            "p=7,3 v=-1,2".to_string(),
            "p=2,4 v=2,-3".to_string(),
            "p=9,5 v=-3,-3".to_string(),
        ];

        
        let mut robits = parse_input(input);
        for _ in 0..100 {
            move_robits(&mut robits, 11,7);
        }
        assert_eq!(calc_quads(robits, 11, 7), 12);
   }

}