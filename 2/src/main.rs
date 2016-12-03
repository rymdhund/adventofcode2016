use std::cmp::{max, min};

fn part1(input: &str) -> i32 {
  // we pretend that the numpad is 0..8 to make the calculations nicer :)

  fn mov(p: i32, dir: char) -> i32 {
     match dir {
      'U' => max(p - 3, p % 3),
      'D' => min(p + 3, (p % 3) + 6),
      'L' => max(p - 1, (p / 3) * 3),
      'R' => min(p + 1, (p / 3) * 3 + 2),
      _ => panic!("Strange direction"),
    }
  }

  let mut res = 0;
  let mut pos = 5 - 1; // convert 1..9 -> 0..8
  for l in input.lines() {
    pos = l.chars().fold(pos, mov);
    res = res * 10 + pos + 1; // the "+1" converts 0..8 -> 1..9
  }
  res
}

fn part2(input: &str) -> String {
  // we do all calculations in this coordinate system
  //                (0, 2)
  //        (-1, 1) (0, 1) (1, 1)
  // (-2,0) (-1, 0) (0, 0) (1, 0) (2, 0)
  //        (-1,-1) (0,-1) (1,-1)
  //                (0,-2)

  fn coord_to_digit((x, y): (i32, i32)) -> i32 {
    x - y * 4 + ( y / 2) * 2 + 7
  }

  fn mov((x, y): (i32, i32), dir: char) -> (i32, i32) {
    let next = match dir {
      'U' => (x, y+1),
      'D' => (x, y-1),
      'L' => (x-1, y),
      'R' => (x+1, y),
      _ => panic!("Strange direction"),
    };
    // check that we're not moving out of the board
    if next.0.abs() + next.1.abs() > 2 { (x, y) } else { next }
  }

  let mut pos = (-2, 0); // this corresponds to 5
  let mut res = String::new();
  for l in input.lines() {
    pos = l.chars().fold(pos, mov);
    res.push_str(&format!("{:X}", coord_to_digit(pos)));
  }
  res
}

fn main() {
  let input = include_str!("../input.txt");
  println!("{}", part1(input));
  println!("{}", part2(input));
}
