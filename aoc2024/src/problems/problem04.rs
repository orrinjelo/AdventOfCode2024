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

pub fn generate_ws(input: Vec<String>) -> Vec<Vec<char>> {
    input.into_iter()
         .map(|s| s.as_str().chars().into_iter().collect() )
         .collect() 
}

pub fn search_xmas(input: Vec<Vec<char>>) -> u32 {
    let n_cols = input[0].len();
    let n_rows = input.len();

    let mut count = 0u32;
    let mut _found: Vec<(usize, usize, u32)> = Vec::new();

    for r in 0..n_rows {
        for c in 0..n_cols {
            if input[r][c] != 'X' {
                continue;
            }
            // Downwards
            if r < n_rows - 3 {
                if input[r+1][c] == 'M' && input[r+2][c] == 'A' && input[r+3][c] == 'S' {
                    count += 1;
                    _found.push((r, c, 0));
                }
            }
            // Upwards
            if r >= 3 {
                if input[r-1][c] == 'M' && input[r-2][c] == 'A' && input[r-3][c] == 'S' {
                    count += 1;
                    _found.push((r, c, 1));
                }
            }
            // Right
            if c < n_cols - 3 {
                if input[r][c+1] == 'M' && input[r][c+2] == 'A' && input[r][c+3] == 'S' {
                    count += 1;
                    _found.push((r, c, 2));
                }
            }
            // Left
            if c >= 3 {
                if input[r][c-1] == 'M' && input[r][c-2] == 'A' && input[r][c-3] == 'S' {
                    count += 1;
                    _found.push((r, c, 3));
                }
            }
            // Down Left
            if c >= 3 && r < n_rows - 3 {
                if input[r+1][c-1] == 'M' && input[r+2][c-2] == 'A' && input[r+3][c-3] == 'S' {
                    count += 1;
                    _found.push((r, c, 4));
                }
            }
            // Down Right
            if c < n_cols - 3 && r < n_rows - 3 {
                if input[r+1][c+1] == 'M' && input[r+2][c+2] == 'A' && input[r+3][c+3] == 'S' {
                    count += 1;
                    _found.push((r, c, 5));
                }
            }
            // Up Left
            if c >= 3 && r >= 3 {
                if input[r-1][c-1] == 'M' && input[r-2][c-2] == 'A' && input[r-3][c-3] == 'S' {
                    count += 1;
                    _found.push((r, c, 6));
                }
            }
            // Up Right
            if c < n_cols - 3 && r >= 3 {
                if input[r-1][c+1] == 'M' && input[r-2][c+2] == 'A' && input[r-3][c+3] == 'S' {
                    count += 1;
                    _found.push((r, c, 7));
                }
            }
            
        }
    }
    // debug!("{:?}", _found);
    count
}

pub fn search_x_mas(input: Vec<Vec<char>>) -> u32 {
    let n_cols = input[0].len();
    let n_rows = input.len();

    let mut count = 0u32;
    let mut _found: Vec<(usize, usize, u32)> = Vec::new();

    for r in 1..n_rows-1 {
        for c in 1..n_cols-1 {
            if input[r][c] != 'A' {
                continue;
            }
            // Down slope
            if (input[r+1][c+1] == 'M' && input[r-1][c-1] == 'S') || (input[r+1][c+1] == 'S' && input[r-1][c-1] == 'M') {
                // Up slope
                if (input[r+1][c-1] == 'M' && input[r-1][c+1] == 'S') || (input[r+1][c-1] == 'S' && input[r-1][c+1] == 'M') {
                    count += 1;
                    _found.push((r,c,0));
                }
            }
        }
    }

    count
}

pub fn problem_041(input: Vec<String>) -> RetType {
    return RetType::U32(
        search_xmas( generate_ws( input ) )
    );
}

pub fn problem_042(input: Vec<String>) -> RetType {
    return RetType::U32(
        search_x_mas( generate_ws( input ) )
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
    fn test_ws() {
        init();
        let input = vec!(
            "MMMSXXMASM".to_string(),
            "MSAMXMSMSA".to_string(),
            "AMXSXMAAMM".to_string(),
            "MSAMASMSMX".to_string(),
            "XMASAMXAMM".to_string(),
            "XXAMMXXAMA".to_string(),
            "SMSMSASXSS".to_string(),
            "SAXAMASAAA".to_string(),
            "MAMMMXMMMM".to_string(),
            "MXMXAXMASX".to_string(),
        );

        let ws = generate_ws(input);

        assert_eq!(ws.len(), 10);
        assert_eq!(ws[0].len(), 10);

        let count = search_xmas(ws);

        assert_eq!(count, 18);
    }

    #[test]
    fn test_xmass() {
        init();
        let input = vec!(
            "MMMSXXMASM".to_string(),
            "MSAMXMSMSA".to_string(),
            "AMXSXMAAMM".to_string(),
            "MSAMASMSMX".to_string(),
            "XMASAMXAMM".to_string(),
            "XXAMMXXAMA".to_string(),
            "SMSMSASXSS".to_string(),
            "SAXAMASAAA".to_string(),
            "MAMMMXMMMM".to_string(),
            "MXMXAXMASX".to_string(),
        );

        let ws = generate_ws(input);

        let count = search_x_mas(ws);

        assert_eq!(count, 9);
    }

}