fn parse_line(line: &str) -> (String, String, i32) {
  let a = line.rsplitn(2, '-').collect::<Vec<&str>>();
  (a[1].to_string(),
   a[0].chars().filter(|&c| c >= 'a' && c <= 'z').collect::<String>(),
   a[0].chars().filter(|&c| c >= '0' && c <= '9').collect::<String>().parse::<i32>().unwrap())
}

fn max(s: &str) -> char {
  let count = |s: &str, c| s.chars().filter(|&x| x == c).count();

  let mut char_counts = s.chars()
                         .map(|c| (usize::max_value() - count(s, c), c))
                         .collect::<Vec<(usize, char)>>();
  char_counts.sort();
  char_counts[0].1
}

fn valid_room(room: &str, check: &str) -> bool {
  let remove = |s: &str, c| s.chars().filter(|&x| x != c).collect::<String>();
  let mut s = room.chars().filter(|&c| c != '-').collect::<String>();
  let mut c = String::new();
  for _ in 0..5 {
    let m = max(&s);
    c.push(m);
    s = remove(&s, m);
  }
  c == check
}

fn part1(input: &str) -> i32 {
  input.lines().map(|l| parse_line(l))
               .filter_map(|(room, check, num)| if valid_room(&room, &check) { Some(num) } else { None })
               .sum::<i32>()
}

fn decrypt(room: &str, num: i32) -> String {
  let num = (num % 26) as u8;
  String::from_utf8(room.bytes()
                        .map(|c| if c == ('-' as u8) { ' ' as u8 } else { ((c - 97u8 + num) % 26u8) + 97u8 })
                        .collect::<Vec<u8>>())
  .unwrap()
}

fn part2(input: &str) -> i32 {
  input.lines()
       .map(|l| parse_line(l))
       .filter_map(|(room, check, num)| if valid_room(&room, &check) && decrypt(&room, num).contains("northpole") {
                                          Some(num)
                                        } else {
                                          None
                                        })
       .nth(0).unwrap()
}

fn main() {
  let input = include_str!("../input.txt");
  println!("{}", part1(input));
  println!("{}", part2(input));
}
