fn valid(x: i32, y: i32, z: i32) -> bool {
  x + y > z && x + z > y && y + z > x
}

fn part1(input: &str) -> usize {
  input.lines()
       .map(|l| l.split_whitespace()
                 .map(|value| value.parse::<i32>().unwrap())
                 .collect::<Vec<i32>>())
       .filter(|t| valid(t[0], t[1], t[2]))
       .count()
}

fn part2(input: &str) -> i32 {
  input.lines()
       .map(|l| l.split_whitespace()
                 .map(|value| value.parse::<i32>().unwrap())
                 .collect::<Vec<i32>>())
       .collect::<Vec<Vec<i32>>>()
       .chunks(3)
       .map(|c| valid(c[0][0], c[1][0], c[2][0]) as i32 +
                valid(c[0][1], c[1][1], c[2][1]) as i32 +
                valid(c[0][2], c[1][2], c[2][2]) as i32)
       .sum::<i32>()
}

fn main() {
  let input = include_str!("../input.txt");
  println!("{}", part1(input));
  println!("{}", part2(input));
}
