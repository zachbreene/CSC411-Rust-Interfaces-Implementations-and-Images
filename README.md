<h1 align=center> Interfaces, Implementations, and Images </h1>
<h2 align=center> A CSC 411: Computer Organization Assignment by Zach Breene & C. Wyatt Polasek </h2>
<h4 align=center> Created at the University of Rhode Island, September 2023 </h4>

## Introduction
The purpose of this assignment was to design custom interfaces, adapt standard data structures to solve new problems, and lay a technical foundation for digital image manipulation. The project is split into two primary parts: building a two-dimensional polymorphic array library (`Array2`), and creating a validator program (`sudoku`) that utilizes this library to verify Sudoku puzzle solutions represented as portable graymap images.

---

## Implementation + Functions
### iii/array2/src/lib.rs

This library project contains the logic for the polymorphic 2D array data structure. <br>

&emsp; ***Array2 Data Structure & Invariant Method***

* The abstract concept of a 2D array is represented using a single, one-dimensional `Vec<T>` alongside integer `width` and `height` properties.
* The primary invariant ensures that the size of the array equals the width multiplied by the height, matching the exact length of the internal 1D vector.
* The `get(x: usize, y: usize)` function provides a reference to the element at the requested coordinates safely, ensuring `x` is greater than or equal to 0 and less than the width, and `y` is greater than or equal to 0 and less than the height.

&emsp; ***Constructors & Iteration Method***

* The structure offers multiple constructors, including `new()` for blank initialization with an initial value, `from_row_major()`, and `from_col_major()` to construct the array from pre-existing vectors.
* Custom iterators named `Array2RowMajor` and `Array2ColMajor` are deployed via `iter_row_major()` and `iter_col_major()` to let the user traverse the array in different linear sequences.

### iii/sudoku/src/main.rs

This binary project contains the logic for identifying and validating solved Sudoku puzzles. <br>

&emsp; ***Image Processing & Validation Method***

* The program utilizes the `csc411_image` crate to read a portable graymap (PGM) file either from a command-line argument or from standard input.
* It maps the image's row-major pixels directly into the `Array2` data structure.
* The validator checks a 9x9 graymap to ensure the maximum pixel intensity is nine and that no pixels possess an intensity of zero.
* It verifies that no two pixels have the exact same intensity within any single row, column, or 3x3 submap sector.
* The program purposefully does not print any output to the console; instead, it calls `exit(0)` if the Sudoku puzzle meets all solution criteria, or `exit(1)` if it fails.

---

## Part C: Programming Technique
Alongside the code, this project submission includes a text document (`technique.txt`) detailing a specific programming technique that either partner has incorporated or plans to incorporate into their workflow.

---

## How To Run
**IMPORTANT: Ensure you have a working Rust environment.**

This project workspace is organized within the `iii` parent directory and includes both the array library and the sudoku binary.
* **Array2 Library:** Initialized using `cargo new --lib array2`, this acts strictly as a dependency and does not execute standalone.
* **Sudoku Validator:** Initialized using `cargo new --bin sudoku`, this program requires the `csc411_image` crate and the local `array2` library to be defined in its `Cargo.toml`. You can execute the binary and pass a 9x9 graymap file as an argument or pipe it via standard input. Afterward, check the exit code using `echo $?` in your shell to see if the puzzle was solved successfully.

---

## Contribution
* **Partners:** Zach Breene and C. Wyatt Polasek.
* **Design Methodology:** The project's structure was formulated utilizing a systematic design checklist, establishing the abstract representation, strict function contracts, and data invariants required for the 2D array abstraction before writing the implementation.
