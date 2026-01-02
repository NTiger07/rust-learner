// Primitive Data Types
// int, float, bool, char

// Compound Data Types
//arrays, tuples, slices, strings

fn main() {
    let x: i32 = -80;
    let y: u64 = 80;

    let is_snowing: bool = true;
    let letter: char = 'a';

    let numbers: [i32; 5] = [1,2,3,4,5]; // homogenous
    let fruits: [&str; 3] = ["Apple", "Orange", "Mango"];


    let human: (&str, i32, bool)= ("Alice", 30, false);

    //Slices: contiguous
    let number_slice:&[i32;5] = &[1,2,3,4,5];

    // Strings vs String Slices (&str)
    // Strings [growable, mutable, owned string type]
    let mut stone_cold: String = String::from("Hell, ");
    stone_cold.push_str("Shekpeee");

    // &str (String Slice)
    let string : String  = String::from("Hello, World");
    let slice: &str = &string[1..5];

    println!("Slice Value: {}", slice);

}
