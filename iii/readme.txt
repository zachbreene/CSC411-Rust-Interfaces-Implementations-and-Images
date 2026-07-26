C. Wyatt Polasek & Zach Breene
Assignment 2 - iii
readme.txt


Acknowledgements:

For help solving the array2 program, we used the following resources:
- GitHub Copilot
- https://doc.rust-lang.org/std/vec/struct.Vec.html
- https://doc.rust-lang.org/rust-by-example/trait/iter.html
- https://doc.rust-lang.org/std/option/enum.Option.html
- https://stackoverflow.com/questions/32304595/whats-the-difference-between-self-and-self
- https://doc.rust-lang.org/reference/expressions/range-expr.html#:~:text=Expression%20RangeFullExpr%20%3A%20,7
- https://stackoverflow.com/questions/27175685/how-to-allocate-space-for-a-vect-in-rust
- CSC411 Notes 10/3/23
- CSC411 Notes 10/5/23

For help solving the sudoku program, we used the following resources:
- GitHub Copilot
- https://doc.rust-lang.org/std/vec/struct.Vec.html
- https://doc.rust-lang.org/rust-by-example/trait/iter.html
- https://doc.rust-lang.org/std/option/enum.Option.html
- https://doc.rust-lang.org/reference/expressions/range-expr.html#:~:text=Expression%20RangeFullExpr%20%3A%20,7
- https://docs.rs/csc411_image/latest/csc411_image/
- https://doc.rust-lang.org/std/process/index.html
- https://stackoverflow.com/questions/66362625/why-is-rusts-expect-called-expect
- https://www.programiz.com/rust/unwrap-and-expect
- https://stackoverflow.com/questions/28273169/how-do-i-convert-between-numeric-types-safely-and-idiomatically
- https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.skip
- https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.take
- CSC411 Notes 10/3/23
- CSC411 Notes 10/5/23


Implementation:
We have succesfully implemented both Part A: array2 and Part B: sudoku. Part A of this assignment 
uses the vector abstraction Vec to create a two-dimensional, polymorphic array called Array2. Part B
of this assignment takes a portable graymap file as input and checks if it represents a solved sudoku puzzle.


Critical parts of Design Checklist:
1. What is the abstract thing you are trying to represent?
    
    ● In this assignment, the abstract thing we are trying to represent
    is a polymorphic 2D array named Array2 that can contain
    elements of any type.


2. What functions will you offer, and what are the contracts of that those
functions must meet?

  ● new(width: usize, height: usize, initial_value: T) -> Self { }
        - Initializes a new Array2 with a defined width and height,
          and sets all elements equal to initial_value.

  ● from_row_major(width: usize, height: usize, data: Vec<T>) -> Self { }
        - Constructs a new Array2 from a vector in row major order.

  ● from_col_major(width: usize, height: usize, data: Vec<T>) -> Self { }
        - Constructs a new Array from a vector in column major order.

  ● iter_row_major(&self)-> Array2RowMajor<T>
        - Iterates through an Array2 in row major order
    
  ● iter_col_major(&self) -> Array2ColMajor<T>
        - Iterates through an Array2 in column major order
    
  ● get(&self, width: usize, height: usize) -> Option<&T>
        - Returns a reference to the element at the given
          coordinates


4. What representation will you use, and what invariants will it satisfy?

  ● Array2 will be represented with the values: width and height, as
    well as Vec<T> which is a 1 dimensional vector that contains the
    elements to be included in Array2.

    Invariants:

    - Size of the Array2 should be equal to width * height which
      would be the same length as Vec<T>.
    
    - When using the get() function to access an element at (x,y), x
      should be a value greater than or equal to 0 and less than
      width, and y should be a value greater than or equal to 0 and
      less than height.

  ● This method for designing our program allows us to represent a
    2D array in the most straightforward way possible. Whether
    using the Row major or Column major order, the program can
    provide a method of iterating over the 2 dimensional array with
    a one to one representation of the elements within it
    corresponding to a specific x and y coordinate value.



It took us approximately 16-18 hours to complete this assignment.