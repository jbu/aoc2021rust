use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn ans_1_1(l: Vec<i32>) -> i32 {
  let tail = l.iter().zip(&l[1..]).map(|x| (x.0 < x.1) as i32);
  return tail.sum();
}

fn ans_1_2(l: Vec<i32>) -> i32 {
  let a = l.iter();
  let b = &l[1..];
  let c = &l[2..];
  let windows: Vec<i32> = a.zip(b)
              .zip(c)
              .map(|((x,y),z)| [x, y, z])
              .map(|x| x[0]+x[1]+x[2])
              .collect();
  let drops = windows.iter()
                     .zip(&windows[1..])
                     .map(|x| (x.0 < x.1) as i32)
                     .sum();
  return drops;
}
fn main() {
    // File hosts must exist in current path before this produces output
    //let t = vec![199,200,208,210,200,207,240,269,260,263];
    let l = lines_from_file("./test_1.txt");
    let t: Vec<_> = l.iter().map(|x| x.parse::<i32>().unwrap()).collect();
    let v = ans_1_2(t);
    println!("{}", v);
}

