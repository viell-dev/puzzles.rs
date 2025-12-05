# Programming puzzles and challenges

[![Rust](https://github.com/viell-dev/puzzles.rs/actions/workflows/rust.yml/badge.svg)](https://github.com/viell-dev/puzzles.rs/actions/workflows/rust.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

This monorepo contains my solutions to various programming puzzles, challenges,
and problems from various sites. All solutions are made as individual Rust
packages.

**Puzzle Semantics:**

These solutions maintain **semantic fidelity** to each puzzle's self-contained
world. The code mirrors the problem's narrative and constraints as presented,
not the underlying abstractions they might represent.

The approach derives organically from the problem description itself, even when
it happens to map to well-known algorithms underneath.

**Parsing Philosophy:**

Input is parsed into domain models that describe **what the data is**, not how
it will be used. A single parsing pass produces a representation of the problem
space itself; `Vec<Instruction>`, `Grid<Cell>`, etc. independent of any solution
strategy.

The parsed model exists as a self-contained description of the puzzle's world,
from which solutions are then derived.

**Solution Philosophy:**

Solutions prioritize **understanding over optimization**, working directly from
the problem's narrative rather than recognizing it as "just" a textbook
algorithm or mathematical shortcut.

If a puzzle names a concept explicitly (e.g., "sum the first n Fibonacci
numbers"), that concept exists in the problem's vocabulary and can be used. But
if it merely describes a pattern without naming it, the solution derives that
pattern from first principles. **The process of solving is the puzzle.**

## Advent of Code

<details><summary>Show solved puzzle links</summary>

### 2015

1.  [Not Quite Lisp](puzzles/advent_of_code/2015/day01)

</details>
