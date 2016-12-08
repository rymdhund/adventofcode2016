extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;
use std::iter;

fn next_character(hash: &str, n: usize) -> Option<String> {
  if hash.starts_with("00000") {
    Some(hash.chars().skip(5).take(n).collect::<String>())
  } else {
    None
  }
}

fn part1(door_id: &str) -> String {
  let mut sh = Md5::new();
  (0..).filter_map(|i| {
    sh.input_str(&format!("{}{}", door_id, i));
    let hash = sh.result_str();
    sh.reset();
    next_character(&hash, 1)
  }).take(8).collect::<String>()
}

#[test]
fn test_next_character() {
  assert_eq!(Some("a".to_string()), next_character("00000a", 1));
  assert_eq!(None, next_character("abcd", 1));
  assert_eq!(Some("ab".to_string()), next_character("00000ab", 2));
}

#[test]
fn test_part1() {
  assert_eq!("18f47a30", part1("abc"));
}

fn part2(door_id: &str) -> String {
  let mut sh = Md5::new();
  let mut pw: Vec<char> = iter::repeat(' ').take(8).collect();
  for i in 0.. {
    sh.input_str(&format!("{}{}", door_id, i));
    let hash = sh.result_str();
    sh.reset();
    if let Some(chars) = next_character(&hash, 2) {
      let a = chars.chars().nth(0).unwrap() as usize - ('0' as usize);
      let b = chars.chars().nth(1).unwrap();
      if a < 8 && pw[a] == ' ' {
        pw[a] = b;
        println!("{}", pw.clone().into_iter().collect::<String>());
      }
    }
    if !pw.contains(&' ') {
      break;
    }
  }
  pw.into_iter().collect()
}

fn main() {
  println!("{}", part1("ugkcyxxp"));
  println!("{}", part2("ugkcyxxp"));
}
