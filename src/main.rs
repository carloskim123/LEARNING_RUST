// Primitive data types
// int, float, bool, char

//?? Integer
// Rust has signed (+ and -) and unsigned integer (only+) types of different sizes
// i8, i16, i32, i64, i128: Signed integers
// u8, u16, u32, u64, u128: Signed integers


fn main() {
    // let x: i32 = -42;
    // let y: u64 = 100;
    // println!("Signed integer: {}", x);
    // println!("Unsigned integer: {}", y);

    //?? Floats [Floating Point Types]
    // f32, f64
    let pi: f64 = 3.14;
    println!("Value of pi: {}", pi);

    //?? Boolean Values : true | false
    let is_snowing: bool = true;
    println!("Is it snowing? {}", is_snowing);

    // ?? Character type : char
    let letter: char = 'a';
    println!("The first letter of the alphabet is: {}", letter);

    //?? Compount Data Types
    // arrays, tuples, slices, and strings (slice strings)

    // arrays
    let numbers:[i32; 5] = [1,2,3,4,5];
    println!("Array elements: {:?}", numbers);
    // let mix = [1,2,"apple", true];
    let fruits: [&str; 3] = ["apple", "mango", "orange"];
    println!("Fruits array: {:?}", fruits); 
    println!("Fruits array 1st element: {}", fruits[0]); 
    println!("Fruits array 2nd element: {}", fruits[1]); 
    println!("Fruits array 3rd element: {}", fruits[2]); 

    // tuples
    let human: (String, i32, bool)= ("Alice".to_string(), 30, false);
    println!("Human tuple: {:?}", human);
    let my_mix_tuple = ("Kratos",23, true,[1,2,3,4,5]);
    println!("My mix tuple: {:?} ", my_mix_tuple);

    // slices
    let number_slices = &[1,2,3,4,6];
    println!("Num. Slices: {:?} ",number_slices);
    
    let animal_slices = &[1,2,3,4,6];
    println!("Num. Slices: {:?} ",number_slices);

}   
