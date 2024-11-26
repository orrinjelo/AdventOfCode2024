use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::fmt;

pub mod macros;

/// Utility function to read lines from a file
/// Opens and reads a file, returns a vector of strings 
///  wrapped in a Result
/// 
/// # Arguments
/// filename - String filename path
///
/// # Returns
/// Result of a Vector of Strings
fn lines_from_file(filename: String) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

/// Load strings from a file
///
/// # Arguments
/// filename - String filename path
///
/// # Returns
/// A Vector of strings
pub fn load_file(filename: String) -> Vec<String> {
    lines_from_file(filename)
        .expect("Could not read from file")
} 

#[allow(dead_code)]
pub fn flatten<T>(nested: Vec<Vec<T>>) -> Vec<T> {
    nested.into_iter().flatten().collect()
}

#[allow(dead_code)]
#[derive(Clone, PartialEq)]
pub enum RetType {
    U32(u32),
    I32(i32),
    U64(u64),
    U128(u128),
    STRING(String),
}

impl fmt::Debug for RetType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RetType::U32(x) => write!(f, "{:?}", x),
            RetType::I32(x) => write!(f, "{:?}", x),
            RetType::U64(x) => write!(f, "{:?}", x),
            RetType::U128(x) => write!(f, "{:?}", x),
            RetType::STRING(x) => write!(f, "{:?}", x),
        }
        
    }
}

impl fmt::Display for RetType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RetType::U32(x) => write!(f, "{}", x),
            RetType::I32(x) => write!(f, "{}", x),
            RetType::U64(x) => write!(f, "{}", x),
            RetType::U128(x) => write!(f, "{}", x),
            RetType::STRING(x) => write!(f, "{}", x),
        }
        
    }
}