use std::ops::RangeInclusive;

/// Parse a single range.
fn parse_range(range: &str) -> RangeInclusive<u32> {
  let mut bounds = range.split('-');

  let start = bounds.next().expect("range missing first value")
    .parse::<u32>().expect("first value not a u32");

  let end = bounds.next().expect("range missing second value")
    .parse::<u32>().expect("second value not a u32");

  start..=end
}

/// Parse a single line of the input file.
fn parse_line(line: &str) -> (RangeInclusive<u32>, RangeInclusive<u32>) {
  let mut ranges = line.split(',');

  let a_range 
    = parse_range(ranges.next().expect("missing first range"));
  let b_range 
    = parse_range(ranges.next().expect("missing first range"));

  (a_range, b_range)
}

/// Check if a range fits completely within another range.
fn range_contains(a: &RangeInclusive<u32>, b: &RangeInclusive<u32>) -> bool {
  a.contains(&b.start()) && a.contains(&b.end())
}

/// Check if a range intersects with another range.
fn range_intersects(a: &RangeInclusive<u32>, b: &RangeInclusive<u32>) -> bool {
  a.contains(&b.start()) || a.contains(&b.end())
}

/// In how many assignment pairs does one range fully contain the other?
pub fn part_one(input: &str) -> u32 {

  let lines = input.lines();
  let mut containing: u32 = 0;

  for line in lines {
    let (a, b) = parse_line(line);
    if range_contains(&a, &b) || range_contains(&b, &a) {
      containing += 1;
    }
  }

  containing
}

/// In how many assignment pairs do the ranges overlap?
pub fn part_two(input: &str) -> u32 {
  
  let lines = input.lines();
  let mut containing: u32 = 0;

  for line in lines {
    let (a, b) = parse_line(line);
    if range_intersects(&a, &b) || range_intersects(&b, &a) {
      containing += 1;
    }
  }

  containing
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn part_one_works() {
    let score = part_one(INPUT);
    assert_eq!(score, 2);
  }

  #[test]
  fn part_two_works() {
    let score = part_two(INPUT);
    assert_eq!(score, 4);
  }

  const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

}
