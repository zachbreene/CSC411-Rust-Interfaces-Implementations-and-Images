//C. Wyatt Polasek + Zach Breene
//Array2 Abstraction Sudoku Solutions

/*For help solving this program, we used the following resources:
GitHub Copilot
https://doc.rust-lang.org/std/vec/struct.Vec.html
https://doc.rust-lang.org/rust-by-example/trait/iter.html
https://doc.rust-lang.org/std/option/enum.Option.html
https://doc.rust-lang.org/reference/expressions/range-expr.html#:~:text=Expression%20RangeFullExpr%20%3A%20,7
https://docs.rs/csc411_image/latest/csc411_image/
https://doc.rust-lang.org/std/process/index.html
https://stackoverflow.com/questions/66362625/why-is-rusts-expect-called-expect
https://www.programiz.com/rust/unwrap-and-expect
https://stackoverflow.com/questions/28273169/how-do-i-convert-between-numeric-types-safely-and-idiomatically
https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.skip
https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.take
CSC411 Notes 10/3/23
CSC411 Notes 10/5/23
*/

//Initialization of dependencies
extern crate array2;
extern crate csc411_image;

use array2::Array2;
use csc411_image::{GrayImage, Read, Gray};
use std::env;
//process is used to exit the program if the sudoku is invalid
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    //If a filename is provided, read the image. Otherwise, read from stdin.
    let filename = if args.len() > 1 { Some(&args[1]) } else { None };
    //.map(|x| x.as_str()) was added to fix a type error. It was a part of our notes on 10/3, where we spoke about iterators being lazy.
    let gray_image = GrayImage::read(filename.map(|x| x.as_str())).expect("Failed to read image");
    
    //Convert GrayImage to Array2
    let array2 = Array2::from_row_major(gray_image.width as usize, gray_image.height as usize, gray_image.pixels);

    //Checking Sudoku rules
    if is_valid_sudoku(&array2) {
        process::exit(0);
    } else {
        process::exit(1);
    }
}

//Function to ensure that the given sudoku board is valid
fn is_valid_sudoku(array2: &Array2<Gray>) -> bool {
    let size = 9;

    //This loop uses the Array2RowMajor iterator to check the rows for duplicates to see if the sudoku is valid
    for row in 0..size {
        let mut row_values = Vec::new();
        //The skip method creates an iterator that skips the first n elements of another iterator. Once those elements are skipped, the subsequent elements are yielded
        //The take method creates an iterator that only iterates over the first n elements of another iterator. Once those n elements have been yielded, the take iterator returns None
        let row_iter = array2.iter_row_major().skip(row * size).take(size);
        for value in row_iter {
            //Had to use .value as u8 to convert Gray to u8 to fix error
            //Stack Overflow post that helped here: https://stackoverflow.com/questions/28273169/how-do-i-convert-between-numeric-types-safely-and-idiomatically
            let value = value.value as u8;
            if value == 0 || row_values.contains(&value) {
                return false;
            }
            row_values.push(value);
        }
    }

    //This loop uses the Array2ColMajor iterator to check the columns for duplicates to see if the sudoku is valid
    for col in 0..size {
        let mut col_values = Vec::new();
        //The skip method creates an iterator that skips the first n elements of another iterator. Once those elements are skipped, the subsequent elements are yielded
        //The take method creates an iterator that only iterates over the first n elements of another iterator. Once those n elements have been yielded, the take iterator returns None
        let col_iter = array2.iter_col_major().skip(col * size).take(size);
        for value in col_iter {
            //Had to use .value as u8 to convert Gray to u8 to fix error
            //Stack Overflow post that helped here: https://stackoverflow.com/questions/28273169/how-do-i-convert-between-numeric-types-safely-and-idiomatically
            let value = value.value as u8;
            if value == 0 || col_values.contains(&value) {
                return false;
            }
            col_values.push(value);
        }
    }

    //This for loop check if the subgrids are 3x3
    for i in (0..size).step_by(3) {
        for j in (0..size).step_by(3) {
            let mut subgrid_values = Vec::new();
            for x in i..i+3 {
                for y in j..j+3 {
                    let value = array2.get(x, y).expect("Index out of bounds").value as u8;  //Convert Gray to u8
                    if value == 0 || value > 9 || subgrid_values.contains(&value) { //Double checking if the values are between 1 and 9
                        return false;
                    }
                    subgrid_values.push(value);
                }
            }
        }
    }
    true
}

//Tests
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_subgrid_invalid() {
        let data = "P2
            9 9
            9
            1 2 3 4 5 6 7 8 9
            0 1 2 3 4 5 6 7 8
            8 9 1 2 3 4 5 6 7
            7 8 9 1 2 3 4 5 6
            6 7 8 9 1 2 3 4 5
            5 6 7 8 9 1 2 3 4
            4 5 6 7 8 9 1 2 3
            3 4 5 6 7 8 9 1 2
            2 3 4 5 6 7 8 9 1
        ";
        let mut file = File::create("subgrid_invalid.pgm").expect("Could not create file");
        file.write_all(data.as_bytes()).expect("Could not write data");
        let gray_image = GrayImage::read(Some("subgrid_invalid.pgm")).expect("Failed to read image");
        let array2 = Array2::from_row_major(gray_image.width as usize, gray_image.height as usize, gray_image.pixels);
        assert!(!is_valid_sudoku(&array2));
    }

    #[test]
    fn test_valid_sudoku_1() {
        let data = "P2
            9 9
            9
            1 2 3 4 5 6 7 8 9
            4 5 6 7 8 9 1 2 3
            7 8 9 1 2 3 4 5 6
            2 3 4 5 6 7 8 9 1
            5 6 7 8 9 1 2 3 4
            8 9 1 2 3 4 5 6 7
            3 4 5 6 7 8 9 1 2
            6 7 8 9 1 2 3 4 5
            9 1 2 3 4 5 6 7 8
        ";
        let mut file = File::create("valid_sudoku_1.pgm").expect("Could not create file");
        file.write_all(data.as_bytes()).expect("Could not write data");
        let gray_image = GrayImage::read(Some("valid_sudoku_1.pgm")).expect("Failed to read image");
        let array2 = Array2::from_row_major(gray_image.width as usize, gray_image.height as usize, gray_image.pixels);
        assert!(is_valid_sudoku(&array2));
    }

    #[test]
    fn test_valid_sudoku_2() {
        let data = "P2
            9 9
            9
            1 4 7 3 6 9 8 2 5
            3 6 9 8 2 5 1 4 7
            8 2 5 1 4 7 3 6 9
            4 7 1 6 9 3 2 5 8
            6 9 3 2 5 8 4 7 1
            2 5 8 4 7 1 6 9 3
            7 1 4 9 3 6 5 8 2
            9 3 6 5 8 2 7 1 4
            5 8 2 7 1 4 9 3 6
        ";
        let mut file = File::create("valid_sudoku_2.pgm").expect("Could not create file");
        file.write_all(data.as_bytes()).expect("Could not write data");
        let gray_image = GrayImage::read(Some("valid_sudoku_2.pgm")).expect("Failed to read image");
        let array2 = Array2::from_row_major(gray_image.width as usize, gray_image.height as usize, gray_image.pixels);
        assert!(is_valid_sudoku(&array2));
    }

    #[test]
    fn test_invalid_sudoku() {
        let data = "P2
            9 9
            9
            1 2 3 4 5 6 7 8 9
            1 2 3 4 5 6 7 8 9
            1 2 3 4 5 6 7 8 9
            1 2 3 4 5 6 7 8 9
            1 2 3 4 5 6 7 8 9
            1 2 3 4 5 6 7 8 9
            1 2 3 4 5 6 7 8 9
            1 2 3 4 5 6 7 8 9
            1 2 3 4 5 6 7 8 9
        ";
        let mut file = File::create("invalid_sudoku.pgm").expect("Could not create file");
        file.write_all(data.as_bytes()).expect("Could not write data");
        let gray_image = GrayImage::read(Some("invalid_sudoku.pgm")).expect("Failed to read image");
        let array2 = Array2::from_row_major(gray_image.width as usize, gray_image.height as usize, gray_image.pixels);
        assert!(!is_valid_sudoku(&array2));
    }

    #[test]
    fn test_large_values_sudoku() {
        let data = "P2
            9 9
            99
            1 2 3 4 5 6 7 8 9
            10 11 12 13 14 15 16 17 18
            19 20 21 22 23 24 25 26 27
            28 29 30 31 32 33 34 35 36
            37 38 39 40 41 42 43 44 45
            46 47 48 49 50 51 52 53 54
            55 56 57 58 59 60 61 62 63
            64 65 66 67 68 69 70 71 72
            73 74 75 76 77 78 79 80 81
        ";
        let mut file = File::create("large_values_sudoku.pgm").expect("Could not create file");
        file.write_all(data.as_bytes()).expect("Could not write data");
        let gray_image = GrayImage::read(Some("large_values_sudoku.pgm")).expect("Failed to read image");
        let array2 = Array2::from_row_major(gray_image.width as usize, gray_image.height as usize, gray_image.pixels);
        assert!(!is_valid_sudoku(&array2));
    }
}

