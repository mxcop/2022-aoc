#[macro_use]
mod macros;

fn main() {
  let input = std::fs::read_to_string("puzzles/day-04/input.txt").unwrap();

  let result = day_04::part_two(&input);

  println!("{result}");
}

#[cfg(test)]
mod tests {
  use std::fs::read_to_string;

  #[test] fn day_01_one() { testify!(day_01, "day-01", part_one, 68923); }
  #[test] fn day_01_two() { testify!(day_01, "day-01", part_two, 200044); }

  #[test] fn day_02_one() { testify!(day_02, "day-02", part_one, 8890); }
  #[test] fn day_02_two() { testify!(day_02, "day-02", part_two, 10238); }

  #[test] fn day_03_one() { testify!(day_03, "day-03", part_one, 7908); }
  #[test] fn day_03_two() { testify!(day_03, "day-03", part_two, 2838); }
}
