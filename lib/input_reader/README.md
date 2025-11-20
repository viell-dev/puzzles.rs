# Input Reader

A flexible input reader library for programming puzzle CLI applications (Advent of Code, Project Euler, etc.).

## Overview

`input_reader` provides a unified interface for reading puzzle input from multiple sources with automatic fallback behavior. It handles command-line arguments, file I/O, and interactive stdin input, making it easy to build consistent CLI tools for programming challenges.

## Features

- **Multiple Input Sources**: Read from files, command-line arguments, or standard input
- **Smart Defaults**: Automatic fallback from file → args → stdin
- **Path Resolution**: Different behavior for debug (shared workspace input directory) and release builds (binary-local files)
- **Flexible API**: Consume input as lines or characters
- **Overwrite Protection**: Prompts before overwriting existing input files
- **Fail-Safe Parsing**: Unknown flags are treated as data, not errors

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
input_reader = { path = "../lib/input_reader" }
```

Or if using a workspace:

```toml
[dependencies]
input_reader.workspace = true
```

## Basic Usage

```rust
use input_reader::{read_input, Outcome};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read input with identifier (e.g., "aoc_2025_day01")
    let input = match read_input("aoc_2025_day01")? {
        Outcome::Exit => return Ok(()), // --help was used or no input
        Outcome::Continue(input) => input,
    };

    // Process as lines
    for line in input.lines() {
        let line = line?;
        println!("Line: {}", line);
    }

    Ok(())
}
```

### Processing as Characters

```rust
use input_reader::{read_input, Outcome};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = match read_input("puzzle_id")? {
        Outcome::Exit => return Ok(()),
        Outcome::Continue(input) => input,
    };

    // Process character by character
    for ch in input.chars() {
        let ch = ch?;
        print!("{}", ch);
    }

    Ok(())
}
```

## Command-Line Interface

### Flags

- `-h, --help` - Print help message and exit
- `-i, --input <METHOD>` - Set input method
  - `file` - Read from input file (default)
  - `args` - Read from command-line arguments
  - `stdin` - Read from standard input
- `-s, --save` - Save input to file for future runs
- `-f, --force` - Force operations without confirmation prompts

### Examples

```bash
# Read from file (default behavior)
./puzzle

# Provide input as arguments
./puzzle arg1 arg2 arg3

# Read from stdin
./puzzle --input stdin
# or
echo "input data" | ./puzzle -i stdin

# Save input from args to file
./puzzle --save data1 data2

# Force overwrite existing input file
./puzzle --save --force new1 new2

# Short flags can be grouped
./puzzle -hsf  # help + save + force
./puzzle -sfi stdin  # save + force + input stdin
```

## Path Resolution

The library uses different strategies for locating input files depending on the build type:

### Debug Builds (Development)

- Searches for the workspace root by traversing up from `CARGO_MANIFEST_DIR` to find `.git`
- Uses a shared `input/` directory at the workspace root
- Input files are named: `input/<identifier>.txt`
- Example: `./input/aoc_2025_day01.txt`

**Benefits**: All puzzles share a single input directory, making it easy to manage inputs during development.

### Release Builds (Distribution)

- Looks for `input.txt` in the same directory as the compiled binary
- The identifier is effectively ignored in release builds
- Example: `./puzzle` reads `./input.txt`

**Benefits**: Self-contained binaries with their input file alongside.

## Input Methods

### Auto (Default)

Tries input sources in order until one succeeds:
1. File (if it exists)
2. Command-line arguments (if provided)
3. Standard input (interactive prompt)

```bash
./puzzle  # Uses auto mode by default
```

### File

Only attempts to read from the input file. Exits if the file doesn't exist.

```bash
./puzzle --input file
```

### Args

Only reads from command-line arguments. Each argument becomes a line of input.

```bash
./puzzle --input args line1 line2 line3
```

### Stdin

Prompts for interactive input. Type your input and end with **two blank lines** (press Enter three times).

```bash
./puzzle --input stdin
```

## Saving Input

Use the `--save` flag to persist input to a file:

```bash
# Save args to file
./puzzle --save arg1 arg2

# Save stdin input to file
./puzzle --input stdin --save
```

### Overwrite Protection

By default, if an input file already exists, the library will prompt:

```
Input file already exists. Overwrite? (y/N):
```

Use `--force` to skip the prompt:

```bash
./puzzle --save --force new_data
```

## Example Puzzle CLI Structure

```rust
use input_reader::{read_input, Outcome};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read input
    let input = match read_input("aoc_2025_day01")? {
        Outcome::Exit => return Ok(()),
        Outcome::Continue(input) => input,
    };

    // Parse input
    let numbers: Vec<i32> = input
        .lines()
        .map(|line| line?.parse::<i32>())
        .collect::<Result<_, _>>()?;

    // Solve part 1
    let part1 = solve_part1(&numbers);
    println!("Part 1: {}", part1);

    // Solve part 2
    let part2 = solve_part2(&numbers);
    println!("Part 2: {}", part2);

    Ok(())
}

fn solve_part1(numbers: &[i32]) -> i32 {
    // Your solution here
    numbers.iter().sum()
}

fn solve_part2(numbers: &[i32]) -> i32 {
    // Your solution here
    numbers.iter().product()
}
```

## Error Handling

The library returns descriptive errors:

- `Error::NotFound` - Input file or directory not found
- `Error::Io(io::Error)` - I/O error while reading
- `Error::Var(VarError)` - Environment variable error (debug builds)

## Implementation Notes

### Memory Considerations

The `Input::chars()` method intentionally leaks memory for file-based input to provide an ergonomic API. This is acceptable for short-lived CLI programs where the OS reclaims memory on exit. For long-running applications, use `Input::lines()` instead. See the [API documentation](https://docs.rs/) for details.

### UTF-8 Requirement

All input must be valid UTF-8. The library uses the `utf8-chars` crate for proper character iteration.

## License

Licensed under the same terms as the parent project.
