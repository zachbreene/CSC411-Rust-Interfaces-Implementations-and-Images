//C. Wyatt Polasek + Zach Breene
//Array2 Abstraction

/*For help solving this program, we used the following resources:
GitHub Copilot
https://doc.rust-lang.org/std/vec/struct.Vec.html
https://doc.rust-lang.org/rust-by-example/trait/iter.html
https://doc.rust-lang.org/std/option/enum.Option.html
https://stackoverflow.com/questions/32304595/whats-the-difference-between-self-and-self
https://doc.rust-lang.org/reference/expressions/range-expr.html#:~:text=Expression%20RangeFullExpr%20%3A%20,7
https://stackoverflow.com/questions/27175685/how-to-allocate-space-for-a-vect-in-rust
CSC411 Notes 10/3/23
CSC411 Notes 10/5/23
*/

//`Array2` struct is a 2D polymorphic array where elements are generic and can be of any type T
//The elements are stored in a 1D Vec<T> and accessed using two indices (the row and column positions)
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Array2<T: Clone> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

impl<T: Clone> Array2<T> {
    /*
    Constructs `Array2` from a given width, height, and initial value
    Each element in `Array2` is initialized to the provided value
    
    # Arguments
        * `width`: the width of `Array2`
        * `height`: the height of `Array2`
        * `initial_value`: the value to initialize each element of `Array2`
    */
    pub fn new(width: usize, height: usize, initial_value: T) -> Self { 
        Self {
            width, height, data: vec![initial_value; width * height],
        }
    }

    /*
    Constructs `Array2` from a vec of elements in row-major order

    # Arguments
        * `width`: the width of `Array2`
        * `height`: the height of `Array2`
        * `data`: a vector of elements in row-major order
    */
    pub fn from_row_major(width: usize, height: usize, data: Vec<T>) -> Self { 
        
        //Since data is already in row major order, we can just return the data to Self
        Self {
            width, height, data,
        }   
    }

    /* 
    Constructs `Array2` from a vec of elements in column-major order
    
    # Arguments
        * `width`: the width of `Array2`
        * `height`: the height of `Array2`
        * `data`: a vector of elements in column-major order
    */
    pub fn from_col_major(width: usize, height: usize, data: Vec<T>) -> Self { 
        //with_capacity() is something that we found on Stack Overflow, linked above in the references.
        let mut changed_data = Vec::with_capacity(width * height);
        //The .. operator is something we saw in class, and talked about with a TA.
        //Simply put, it is a range operator that allows you to iterate over a range of values. Hence 0..width/height.
        for col in 0..width {
            for row in 0..height {
                changed_data.push(data[row * width + col].clone())
            }
        }
        Self {
            width, height, data: changed_data,
        }
    }

    //Return an iterator over the elements of `Array2` in row-major order
    pub fn iter_row_major(&self) -> Array2RowMajor<T> { 
        Array2RowMajor {
            array: self,
            position: 0,
        }
    }

    //Return an iterator over the elements of `Array2` in column-major order
    pub fn iter_col_major(&self) -> Array2ColMajor<T> {
        Array2ColMajor {
            array: self,
            position: 0,
        }
    }

    /*
    Returns a reference to the element at the given coordinates

    # Arguments
        * `col`: the column index.
        * `row`: the row index.
    */
    pub fn get(&self, col: usize, row: usize) -> Option<&T> { 
        //Checks if referenced location is within the bounds of the array (vector)
        if col < self.width && row < self.height {
            Some(&self.data[row * self.width + col])
        }
        //Error: Referenced Index is out of Bounds
        else {
            None
        }
    }
}

//Iterator for `Array2` in row-major order
pub struct Array2RowMajor<'a, T: Clone> {
    array: &'a Array2<T>,
    position: usize,
}

impl<'a, T: Clone> Iterator for Array2RowMajor<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        //Checks if the position is within the bounds of the array (vector)
        if self.position < self.array.data.len() {
            let item = &self.array.data[self.position];
            self.position += 1;
            Some(item)
        }
        //Error: Referenced Index is out of Bounds
        else {
            None
        }
    }
}


//Initialization of the iterator for `Array2` in column-major order
pub struct Array2ColMajor<'a, T: Clone> {
    array: &'a Array2<T>,
    position: usize,
}

//Iterator for `Array2` in column-major order
impl<'a, T: Clone> Iterator for Array2ColMajor<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        //Checks if the position is within the bounds of the array (vector)
        //If it is, the item is returned and the position is incremented
        if self.position < self.array.data.len() {
            let col = self.position % self.array.width;
            let row = self.position / self.array.width;
            let index = col * self.array.height + row;
            if index < self.array.data.len() {
                let item = &self.array.data[self.position];
                self.position += 1;
                Some(item)
            }
            else {
                None
            }
        }
        //Out of Bounds Error
        else {
            None
        }
    }
}