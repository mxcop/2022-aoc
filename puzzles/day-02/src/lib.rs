#[derive(PartialEq, Debug)]
enum Move {
  Unknown,
  Rock,
  Paper,
  Scissors,
}

#[derive(PartialEq, Debug)]
enum Outcome {
  Unknown,
  Loss,
  Draw,
  Win,
}

impl Move {
  pub fn from_char(ch: char) -> Self {
    match ch {
      'A' | 'X' => Move::Rock,
      'B' | 'Y' => Move::Paper,
      'C' | 'Z' => Move::Scissors,
      _ => Move::Unknown
    }
  }

  pub fn into_score(&self, opponent: &Move) -> u32 {
    match (self, opponent) {
      (Move::Rock, Move::Scissors) => 1 + 6,
      (Move::Rock, Move::Rock) => 1 + 3,
      (Move::Rock, Move::Paper) => 1 + 0,

      (Move::Paper, Move::Rock) => 2 + 6,
      (Move::Paper, Move::Paper) => 2 + 3,
      (Move::Paper, Move::Scissors) => 2 + 0,

      (Move::Scissors, Move::Paper) => 3 + 6,
      (Move::Scissors, Move::Scissors) => 3 + 3,
      (Move::Scissors, Move::Rock) => 3 + 0,
      _ => 0
    }
  }
}

impl Outcome {
  pub fn from_char(ch: char) -> Self {
    match ch {
      'X' => Outcome::Loss,
      'Y' => Outcome::Draw,
      'Z' => Outcome::Win,
      _ => Outcome::Unknown
    }
  }

  pub fn into_score(&self, opponent: &Move) -> u32 {
    match (self, opponent) {
      (Outcome::Loss, Move::Rock) => 0 + 3,
      (Outcome::Draw, Move::Rock) => 3 + 1,
      (Outcome::Win, Move::Rock) => 6 + 2,

      (Outcome::Loss, Move::Paper) => 0 + 1,
      (Outcome::Draw, Move::Paper) => 3 + 2,
      (Outcome::Win, Move::Paper) => 6 + 3,

      (Outcome::Loss, Move::Scissors) => 0 + 2,
      (Outcome::Draw, Move::Scissors) => 3 + 3,
      (Outcome::Win, Move::Scissors) => 6 + 1,
      _ => 0
    }
  }
}

/// Compute your final score if you follow the strategy guide.
pub fn part_one(input: &str) -> u32 {

  let mut lines = input.lines();
  let mut score = 0;

  while let Some(line) = lines.next() {
    let my_move = Move::from_char(line.chars().nth(2).unwrap());
    let op_move = Move::from_char(line.chars().nth(0).unwrap());

    score += my_move.into_score(&op_move);
  }

  score
}

/// Compute your final score after learning the true meaning of the second column.
pub fn part_two(input: &str) -> u32 {

  let mut lines = input.lines();
  let mut score = 0;

  while let Some(line) = lines.next() {
    let op_move = Move::from_char(line.chars().nth(0).unwrap());
    let outcome = Outcome::from_char(line.chars().nth(2).unwrap());

    score += outcome.into_score(&op_move);
  }

  score
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn part_one_works() {
    let score = part_one(INPUT);
    assert_eq!(score, 15);
  }

  #[test]
  fn part_two_works() {
    let score = part_two(INPUT);
    assert_eq!(score, 12);
  }

  const INPUT: &str = "A Y
B X
C Z";

}
