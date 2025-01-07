use std::{
    fmt::{self, Display},
    io,
    ops::{Index, IndexMut},
};

use input_reader::{InputReader, ReadError};

#[cfg(test)]
const GRID_SIZE: usize = 6;
#[cfg(not(test))]
const GRID_SIZE: usize = 100;

// Constants that determine when lights stay on or turn on
const ON_STAYS_ON_COUNT: [u32; 2] = [2, 3]; // A light stays on with 2 or 3 lit neighbors
const OFF_TURNS_ON_COUNT: [u32; 1] = [3]; // A light turns on with exactly 3 lit neighbors

#[derive(Debug)]
enum GridError {
    ReadError(ReadError),
    IoError(std::io::Error),
    InvalidChar(char),
    InvalidDimension {
        dimension: &'static str,
        row: Option<usize>,
        got: usize,
        expected: usize,
    },
}

impl Display for GridError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GridError::ReadError(err) => write!(f, "{}", err),
            GridError::IoError(err) => write!(f, "IO error: {}", err),
            GridError::InvalidChar(c) => write!(f, "Invalid character in input: '{}'", c),
            GridError::InvalidDimension {
                dimension,
                row,
                got,
                expected,
            } => {
                if let Some(row) = row {
                    write!(
                        f,
                        "Invalid {} count at row {}: got {}, expected {}",
                        dimension, row, got, expected
                    )
                } else {
                    write!(
                        f,
                        "Invalid {} count: got {}, expected {}",
                        dimension, got, expected
                    )
                }
            }
        }
    }
}

impl From<ReadError> for GridError {
    fn from(error: ReadError) -> Self {
        GridError::ReadError(error)
    }
}

impl From<std::io::Error> for GridError {
    fn from(error: std::io::Error) -> Self {
        GridError::IoError(error)
    }
}

#[derive(Debug, Clone)]
struct Grid([[bool; GRID_SIZE]; GRID_SIZE]);

impl Default for Grid {
    fn default() -> Self {
        Self([[false; GRID_SIZE]; GRID_SIZE])
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..GRID_SIZE {
            for &cell in &self[row] {
                write!(f, "{}", if cell { '#' } else { '.' })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Grid {
    fn new() -> Self {
        Self::default()
    }

    fn count_lights_on(&self) -> usize {
        self.0
            .iter()
            .flat_map(|row| row.iter())
            .filter(|&&cell| cell)
            .count()
    }

    fn set_corners_on(&mut self) {
        self[0][0] = true;
        self[0][GRID_SIZE - 1] = true;
        self[GRID_SIZE - 1][0] = true;
        self[GRID_SIZE - 1][GRID_SIZE - 1] = true;
    }
}

impl Index<usize> for Grid {
    type Output = [bool; GRID_SIZE];

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Grid {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

fn parse_bool(c: char) -> Result<bool, GridError> {
    match c {
        '#' => Ok(true),
        '.' => Ok(false),
        c => Err(GridError::InvalidChar(c)),
    }
}

fn parse_lines<I>(lines: I) -> Result<Grid, GridError>
where
    I: Iterator<Item = io::Result<String>>,
{
    let mut grid = Grid::new();

    let lines: Vec<_> = lines.collect::<Result<Vec<_>, _>>()?;

    if lines.len() != GRID_SIZE {
        return Err(GridError::InvalidDimension {
            dimension: "row",
            row: None,
            got: lines.len(),
            expected: GRID_SIZE,
        });
    }

    for (row_idx, line) in lines.iter().enumerate() {
        let chars: Vec<_> = line.chars().collect();
        if chars.len() != GRID_SIZE {
            return Err(GridError::InvalidDimension {
                dimension: "column",
                row: Some(row_idx + 1),
                got: chars.len(),
                expected: GRID_SIZE,
            });
        }

        grid[row_idx]
            .iter_mut()
            .zip(chars)
            .try_for_each(|(cell, c)| -> Result<(), GridError> {
                *cell = parse_bool(c)?;
                Ok(())
            })?;
    }

    Ok(grid)
}

fn read_input() -> Result<Grid, GridError> {
    let lines = InputReader::new()
        .with_path("./input.txt")
        .read_streaming()?
        .filter(|line| line.as_ref().map_or(false, |l| !l.trim().is_empty()));

    parse_lines(lines)
}

fn count_neighbors(grid: &Grid, row: usize, col: usize) -> usize {
    // Calculate bounds for the 3x3 grid around the cell, handling edge cases
    let row_start = row.saturating_sub(1);
    let col_start = col.saturating_sub(1);
    let row_end = (row + 2).min(GRID_SIZE);
    let col_end = (col + 2).min(GRID_SIZE);

    // Count all lit neighbors (excluding the cell itself)
    let mut count = 0;
    for r in row_start..row_end {
        for c in col_start..col_end {
            if (r != row || c != col) && grid[r][c] {
                count += 1;
            }
        }
    }

    count
}

fn next_state(grid: &Grid) -> Grid {
    let mut next = Grid::default();

    for row in 0..GRID_SIZE {
        for col in 0..GRID_SIZE {
            let cell = grid[row][col];
            let neighbors = count_neighbors(grid, row, col);

            // A light stays on with ON_STAYS_ON_COUNT lit neighbors
            // A light turns on with OFF_TURNS_ON_COUNT lit neighbors
            next[row][col] = if cell {
                ON_STAYS_ON_COUNT.contains(&(neighbors as u32))
            } else {
                OFF_TURNS_ON_COUNT.contains(&(neighbors as u32))
            };
        }
    }

    next
}

fn next_state_with_corners_on(grid: &Grid) -> Grid {
    let mut next = next_state(grid);
    next.set_corners_on();

    next
}

fn main() -> Result<(), GridError> {
    let grid = read_input()?;

    let final_grid = (0..100).fold(grid.clone(), |grid, _| next_state(&grid));

    let count = final_grid.count_lights_on();

    println!("Number of lights on after 100 steps: {}", count);

    // Turn on corners for step 2:
    let mut grid_with_corners = grid;
    grid_with_corners.set_corners_on();

    let final_grid_with_corners = (0..100).fold(grid_with_corners, |grid, _| {
        next_state_with_corners_on(&grid)
    });

    let count_with_corners = final_grid_with_corners.count_lights_on();

    println!(
        "Number of lights on after 100 steps with corners on: {}",
        count_with_corners
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_lines(lines: Vec<&str>) -> impl Iterator<Item = io::Result<String>> + '_ {
        lines.into_iter().map(|s| Ok(s.to_string()))
    }

    #[test]
    fn test_valid_grid() {
        let input = vec![
            "......", //
            "...#..", //
            "..##..", //
            "..#...", //
            "......", //
            "......", //
        ];
        let result = parse_lines(create_test_lines(input));
        assert!(result.is_ok());
    }

    #[test]
    fn test_too_few_rows() {
        let input = vec![
            "......", //
            "...#..", //
            "..##..", //
            "..#...", //
            "......", //
                      // missing row
        ];
        let err = parse_lines(create_test_lines(input)).unwrap_err();
        assert!(matches!(
            err,
            GridError::InvalidDimension {
                dimension: "row",
                row: None,
                got: 5,
                expected: 6,
            }
        ));
    }

    #[test]
    fn test_too_many_rows() {
        let input = vec![
            "......", //
            "...#..", //
            "..##..", //
            "..#...", //
            "......", //
            "......", //
            "......", // extra row
        ];
        let err = parse_lines(create_test_lines(input)).unwrap_err();
        assert!(matches!(
            err,
            GridError::InvalidDimension {
                dimension: "row",
                row: None,
                got: 7,
                expected: 6,
            }
        ));
    }

    #[test]
    fn test_too_few_columns() {
        let input = vec![
            "......", //
            "...#..", //
            "..##.",  // missing a column
            "..#...", //
            "......", //
            "......", //
        ];
        let err = parse_lines(create_test_lines(input)).unwrap_err();
        assert!(matches!(
            err,
            GridError::InvalidDimension {
                dimension: "column",
                row: Some(3),
                got: 5,
                expected: 6,
            }
        ));
    }

    #[test]
    fn test_too_many_columns() {
        let input = vec![
            "......",  //
            "...#..",  //
            "..##...", // extra column
            "..#...",  //
            "......",  //
            "......",  //
        ];
        let err = parse_lines(create_test_lines(input)).unwrap_err();
        assert!(matches!(
            err,
            GridError::InvalidDimension {
                dimension: "column",
                row: Some(3),
                got: 7,
                expected: 6,
            }
        ));
    }

    #[test]
    fn test_invalid_character() {
        let input = vec![
            "......", //
            "...#..", //
            "..x#..", // invalid 'x' character
            "..#...", //
            "......", //
            "......", //
        ];
        let err = parse_lines(create_test_lines(input)).unwrap_err();
        assert!(matches!(err, GridError::InvalidChar('x')));
    }

    #[test]
    fn test_count_neighbors() {
        let input = vec![
            ".#.#..", //
            "...#..", //
            ".#.#..", //
            "..#...", //
            "......", //
            "......", //
        ];
        let grid = parse_lines(create_test_lines(input)).unwrap();

        // Test positions around the pattern
        assert_eq!(count_neighbors(&grid, 0, 1), 0); // Top-left light: no adjacent neighbors
        assert_eq!(count_neighbors(&grid, 0, 3), 1); // Top-right light: 1 neighbor (at 1,3)
        assert_eq!(count_neighbors(&grid, 2, 1), 1); // Middle-left light: 1 neighbor (at 3,2)
        assert_eq!(count_neighbors(&grid, 2, 3), 2); // Middle-right light: 2 neighbors (at 1,3 and 3,2)
        assert_eq!(count_neighbors(&grid, 3, 2), 2); // Bottom light: 2 neighbors (from 2,1 and 2,3)

        // Test empty positions
        assert_eq!(count_neighbors(&grid, 0, 0), 1); // Top-left corner: 1 neighbor (at 0,1)
        assert_eq!(count_neighbors(&grid, 1, 2), 5); // Center space: 5 neighbors (at 0,1, 0,3, 1,3, 2,1, and 2,3)
        assert_eq!(count_neighbors(&grid, 5, 5), 0); // Bottom-right corner: no neighbors
    }

    #[test]
    fn test_next_state_sequence() {
        let initial = vec![
            ".#.#.#", //
            "...##.", //
            "#....#", //
            "..#...", //
            "#.#..#", //
            "####..", //
        ];
        let expected_steps = [
            vec![
                "..##..", //
                "..##.#", //
                "...##.", //
                "......", //
                "#.....", //
                "#.##..", //
            ],
            vec![
                "..###.", //
                "......", //
                "..###.", //
                "......", //
                ".#....", //
                ".#....", //
            ],
            vec![
                "...#..", //
                "......", //
                "...#..", //
                "..##..", //
                "......", //
                "......", //
            ],
            vec![
                "......", //
                "......", //
                "..##..", //
                "..##..", //
                "......", //
                "......", //
            ],
        ];

        let mut grid = parse_lines(create_test_lines(initial)).unwrap();

        for (step, expected) in expected_steps.iter().enumerate() {
            grid = next_state(&grid);
            let expected_grid = parse_lines(create_test_lines(expected.clone())).unwrap();
            assert_eq!(
                format!("{}", grid),
                format!("{}", expected_grid),
                "Failed at step {}",
                step + 1
            );
        }
    }

    #[test]
    fn test_next_state_with_corners_on_sequence() {
        let initial = vec![
            "##.#.#", //
            "...##.", //
            "#....#", //
            "..#...", //
            "#.#..#", //
            "####.#", //
        ];
        let expected_steps = [
            vec![
                "#.##.#", //
                "####.#", //
                "...##.", //
                "......", //
                "#...#.", //
                "#.####", //
            ],
            vec![
                "#..#.#", //
                "#....#", //
                ".#.##.", //
                "...##.", //
                ".#..##", //
                "##.###", //
            ],
            vec![
                "#...##", //
                "####.#", //
                "..##.#", //
                "......", //
                "##....", //
                "####.#", //
            ],
            vec![
                "#.####", //
                "#....#", //
                "...#..", //
                ".##...", //
                "#.....", //
                "#.#..#", //
            ],
            vec![
                "##.###", //
                ".##..#", //
                ".##...", //
                ".##...", //
                "#.#...", //
                "##...#", //
            ],
        ];

        let mut grid = parse_lines(create_test_lines(initial)).unwrap();

        for (step, expected) in expected_steps.iter().enumerate() {
            grid = next_state_with_corners_on(&grid);
            let expected_grid = parse_lines(create_test_lines(expected.clone())).unwrap();
            assert_eq!(
                format!("{}", grid),
                format!("{}", expected_grid),
                "Failed at step {}",
                step + 1
            );
        }
    }
}
