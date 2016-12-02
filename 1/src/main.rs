fn part1(input: &str) -> i32 {
  fn step((x,y): (i32, i32), s: (char, i32)) -> (i32, i32) {
    if s.0 == 'L' {
      (y, s.1-x)
    } else if s.0 == 'R' {
      (-y, s.1+x)
    } else {
      panic!();
    }
  }

  let (x, y) = input.split(", ")
                    .map(|y| (y.chars().nth(0).unwrap(), (&y[1..]).parse::<i32>().unwrap()))
                    .fold((0,0), step);

  x.abs() + y.abs()
}

fn part2(input: &str) -> i32 {
  let steps = input.split(", ")
                   .map(|y| (y.chars().nth(0).unwrap(), (&y[1..]).parse::<i32>().unwrap()))
                   .collect::<Vec<(char, i32)>>();

  fn step((x,y): (i32, i32), dir: i32) -> (i32, i32) {
    match dir {
      0 => (x, y+1),
      1 => (x+1, y),
      2 => (x, y-1),
      3 => (x-1, y),
      _ => panic!(),
    }
  }

  let mut dir = 0;
  let mut pos = (0,0);
  let mut history = Vec::new();
  history.push(pos);
  for (d,dist) in steps {
    dir = if d == 'L' { (dir - 1 + 4) % 4 } else { (dir + 1) % 4 };
    for _ in 0..dist {
      pos = step(pos, dir);
      if history.contains(&pos) {
        return pos.0.abs() + pos.1.abs()
      }
      history.push(pos);
    }
  }
  panic!();
}

fn main() {
  let input = include_str!("../input.txt").trim();
  println!("{}", part1(input));
  println!("{}", part2(input));
}
