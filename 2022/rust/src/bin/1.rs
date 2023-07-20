mod consts;
use crate::consts::DATA_DIR;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

fn main() {
  let data_fpath = Path::new(DATA_DIR).join("1.txt");
  let mut sum: i32 = 0;
  let mut highest: i32 = -1;
  if let Ok(lines) = read_lines(data_fpath) {
    for line in lines {
      if let Ok(calories) = line {
        // println!("{calories}");
        if let Ok(cal_int) = i32::from_str(&calories) {
          sum += cal_int;
        }
        if calories.is_empty() {
          if sum > highest {
            highest = sum;
          }
          sum = 0;
        }
      }
    }    
  }
  println!("{highest}")
}

// TODO: Move to helpers file.
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}