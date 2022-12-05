#[cfg(test)]
macro_rules! testify {
  ($day_num:ident, $day_path:expr, $part:ident, $expected:expr) => {
    // Format the test file path.
    let file_path = format!("puzzles/{}/input.txt", $day_path);

    // Load the input file into a string.
    let input = read_to_string(file_path).expect("missing input.txt");

    // Run the part.
    let score = $day_num::$part(&input);

    // Assert.
    assert_eq!(score, $expected);
  };
}
