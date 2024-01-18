// DATA TYPES 
//
// Scalar types - represents a single value. Rust has four primary scalar types: int, float, bool,
// and char
// Integer types - number without a fraction type 
// let x = 2; 
// Float type - numbers with decimal points 
// let y: f32 = 3.0; // f32
// let x = 2.0; // f64
// Char - rusts' most primitive alphapbetic type 
// let c = 'z';
// let z: char = 'Z'; // with explicit type annotation
// Tuple - general way of grouping together a number of values with a variety of types into one
// compound type 
// let tup: (i32, f64, u8) = (500, 6.4, 1); 
// let (x, y, z) = tup; 
// println!("The value of y is: {y}");
//
// **accessing tuple element directly by using a period: 
// let x: (i32, f64, u8) = (500, 6.4, 1); 
//
// let five_hundred = x.0; 
// let six_point_four = x.1; 
// let one = x.2; 
//
// Array - another way to have a collection of multiple values 
// let a = [1, 2, 3, 4, 5];
//
// let a: [i32, 5] = [1, 2, 3, 4, 5];
//
// initalizing an array:
// let a = [3; 5]; 
//
// accessing array elements: 
// let a = [1, 2, 3, 4, 5];
//
// let first = a[0];
// let second = a[1]; 

fn main() {
    let x = 5;
    
    let x = x + 1; 

    {
        let x = x * 2; 
        println!("The value of X in the inner scope is: {x}"); 
    }
    println!("The value of x is: {x}"); 

    // variable automatically changes type
    // on the first line it is a string
    // on the second line it is a number 
    let spaces = "   "; 
    let spaces = spaces.len(); 

    // this will error out because we cannot mutate a variable's type 
    let mut spaces = "   "; 
    spaces = spaces.len(); 
}
