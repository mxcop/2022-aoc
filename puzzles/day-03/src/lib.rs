/// Get the priority of an item.
fn char_to_prio(ch: char) -> u8 {
  match ch {
    'a'..='z' => ch as u8 - 96,
    'A'..='Z' => ch as u8 - 38,
    _ => 0
  }
}

/// Find the common character within two string slices.
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

/// Find the common character within three string slices.
fn triple_common(str_a: &str, str_b: &str, str_c: &str) -> char {
  for item_a in str_a.chars() {
    for item_b in str_b.chars() {
      for item_c in str_c.chars() {
        if item_a == item_b && item_a == item_c {
          return item_a;
        }
      }
    }
  }
  unreachable!();
}

/// Find the common item within each rucksack and sum their priority.
pub fn part_one(input: &str) -> u32 {
  
  let mut lines = input.lines();
  let mut sum: u32 = 0;

  while let Some(line) = lines.next() {
    let compartment_a = &line[0..line.len()/2];
    let compartment_b = &line[line.len()/2..line.len()];
  
    let common_item = find_common(compartment_a, compartment_b);
    sum += char_to_prio(common_item) as u32;
  }
  
  sum
}

/// Find the badge of a group of three elves and sum their priority.
pub fn part_two(input: &str) -> u32 {

  let mut lines = input.lines();
  let mut sum: u32 = 0;

  while let Some(rucksack_a) = lines.next() {
    let rucksack_b = lines.next().expect("Uncomplete group");
    let rucksack_c = lines.next().expect("Uncomplete group");

    let badge = triple_common(rucksack_a, rucksack_b, rucksack_c);
    sum += char_to_prio(badge) as u32;
  }

  sum
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
  fn part_two_works() {
    let score = part_two(INPUT);
    assert_eq!(score, 70);
  }

  const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

}
