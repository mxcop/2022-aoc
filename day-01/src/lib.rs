/// Find the largest amount of calories carried by an elf.
pub fn part_one(input: &str) -> u32 {

  let mut lines = input.lines();
  let mut elfs: Vec<u32> = Vec::new();

  let mut curr_sum: u32 = 0;
  while let Some(line) = lines.next() {
    if line.len() == 0 {
      elfs.push(curr_sum);
      curr_sum = 0;
    } else {
      curr_sum += line.parse::<u32>().unwrap();
    }
  }

  *elfs.iter().max().unwrap()
}

/// Find the sum of the three elfs carrying the largest amount of calories.
pub fn part_two(input: &str) -> u32 {

  let mut lines = input.lines();
  let mut elfs: Vec<u32> = Vec::new();

  let mut curr_sum: u32 = 0;
  while let Some(line) = lines.next() {
    if line.len() == 0 {
      elfs.push(curr_sum);
      curr_sum = 0;
    } else {
      curr_sum += line.parse::<u32>().unwrap();
    }
  }
  elfs.push(curr_sum);

  reverse_insertion_sort(elfs.clone()).iter().take(3).sum::<u32>()
}

/// Insertion sorting algorithm.
fn reverse_insertion_sort(mut vec: Vec<u32>) -> Vec<u32> {
  let mut sorted: bool = true;
  while sorted {
    sorted = false;

    for i in 0..vec.len() {
      if i < vec.len() - 1 && vec[i] < vec[i+1] {
        let a = vec[i];
        vec[i] = vec[i+1];
        vec[i+1] = a;
        sorted = true;
      }
    }
  }

  vec
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn part_one_works() {
    let largest_cal = part_one(INPUT);
    assert_eq!(largest_cal, 24000);
  }

  #[test]
  fn part_two_works() {
    let sum_of_top_3 = part_two(INPUT);
    assert_eq!(sum_of_top_3, 45000);
  }

  const INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

}
