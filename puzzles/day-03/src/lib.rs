fn char_to_prio(ch: char) -> u32 {
  match ch {
    'a'..='z' => 0,
    'A'..='Z' => 0,
    _ => 0
  }
}

/// Find a common character within two string slices.
fn find_common(str_a: &str, str_b: &str) -> char {
  for item_a in str_a.chars() {
    for item_b in str_b.chars() {
      if item_a == item_b {
        return item_a;
      }
    }
  }
  unreachable!();
}

/// 
pub fn part_one(input: &str) -> u32 {
  
  let mut lines = input.lines();

  while let Some(line) = lines.next() {
    let compartment_a = &line[0..line.len()/2];
    let compartment_b = &line[line.len()/2..line.len()];
  
    let common_item = find_common(compartment_a, compartment_b);
    //dbg!(common_item as u32);
  }
  
  0
}

#[allow(unused)]
/// 
pub fn part_two(input: &str) -> u32 {
  0
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn part_one_works() {
    let score = part_one(INPUT);
    assert_eq!(score, 157);
  }

  #[test]
  #[ignore]
  fn part_two_works() {
    let score = part_two(INPUT);
    assert_eq!(score, 0);
  }

  const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

}
