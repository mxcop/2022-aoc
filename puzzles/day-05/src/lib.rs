// Made with the help of <https://www.youtube.com/watch?v=6b2ymQWldoE>

mod stack;

use nom::{IResult, bytes::complete::tag, character::{complete::{self, alpha1, newline, digit1, multispace1}}, sequence::{delimited, preceded}, branch::alt, multi::{separated_list1, many1}};
use stack::Stack;

#[derive(Debug)]
struct Move {
  pub num: u32,
  pub from: usize,
  pub to: usize
}

impl Move {
  pub fn new(num: &str, from: &str, to: &str) -> Self {
    Self {
      num: num.parse().expect("not a number"),
      from: from.parse().expect("not a number"),
      to: to.parse().expect("not a number")
    }
  }
}

/// move # from # to #
fn parse_cmd(input: &str) -> IResult<&str, Move> {
  let (input, _) = tag("move ")(input)?;
  let (input, num) = digit1(input)?;
  let (input, _) = tag(" from ")(input)?;
  let (input, from) = digit1(input)?;
  let (input, _) = tag(" to ")(input)?;
  let (input, to) = digit1(input)?;

  Ok((input, Move::new(num, from, to)))
}

/// "[c]" or "   "
fn parse_crate(input: &str) -> IResult<&str, Option<&str>> {
  let (input, num) = alt((
    tag("   "),
    delimited(
      complete::char('['), 
      alpha1, 
      complete::char(']')
    )
  ))(input)?;

  let res = match num {
    "   " => None,
    value => Some(value)
  };

  Ok((input, res))
}

/// "crate" -> " " -> "crate" ...
fn parse_line(input: &str) -> IResult<&str, Vec<Option<&str>>> {
  let (input, res) 
    = separated_list1(tag(" "), parse_crate)(input)?;

  Ok((input, res))
}

/// Parse the stacks from the input file.
fn parse_stacks(input: &str) -> IResult<&str, (Vec<Stack<&str>>, Vec<Move>)> {
  let (input, rows) 
    = separated_list1(newline, parse_line)(input)?;

  let mut stacks: Vec<Stack<&str>> = Vec::new();

  for _ in 0..rows.get(0).unwrap().len() {
    stacks.push(Stack::new(vec![]));
  }

  for row in rows {

    for col in 0..row.len() {
      if let Some(Some(item)) = row.get(row.len() - col - 1) {
        stacks.get_mut(row.len() - col - 1).unwrap().push(item);
      }
    }
  }
  
  let (input, _) = newline(input)?;
  let (input, _) = many1(preceded(multispace1, digit1))(input)?;
  let (input, _) = multispace1(input)?;

  let (input, moves) = separated_list1(newline, parse_cmd)(input)?;

  Ok((input, (stacks, moves)))
}

/// After the rearrangement procedure completes, what crate ends up on top of each stack?
pub fn part_one(input: &str) -> String {

  let (_, (mut stacks, moves)) = parse_stacks(input).unwrap();

  for cmd in moves {
    dbg!(&stacks);
    for _ in 0..cmd.num {
      let stack = stacks.get_mut(cmd.from - 1).unwrap();
      let item = stack.pop();
      let stack = stacks.get_mut(cmd.to - 1).unwrap();
      stack.push(item);
    }
  }

  let mut result: Vec<char> = vec![];
  for mut stack in stacks {
    result.push(stack.pop().chars().nth(0).unwrap());
  }

  result.iter().collect::<String>()
}

/// 
pub fn part_two(input: &str) -> String {
  todo!();
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn part_one_works() {
    let score = part_one(INPUT);
    assert_eq!(score, "CMZ");
  }

  #[ignore]
  #[test]
  fn part_two_works() {
    let score = part_two(INPUT);
    assert_eq!(score, "");
  }

  const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 
  
move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

}
