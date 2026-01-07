//Approach 1
// enum Option<T>{ // Define the generic Option type
//     Some(T), // Represents a value
//     None, //Represents no value
// }

// //Approach 2
// enum Result<T, E>{ // Define the generic Result type
//     Ok(T), // Represents a value
//     Err(E), //Represents an error
// }

fn main() {
    // Approach 1
    fn divide_option(numerator: f64, denominator: f64) -> Option<f64> {
        if denominator == 0.0 {
            None
        } else {
            Some(numerator / denominator)
        }
    }

    match divide_option(10.0, 2.0) {
        Some(x) => println!("Result: {}", x),
        None => println!("Cannot divide by Zero!"),
    }

    // Approach 2
    fn divide_result(numerator: f64, denominator: f64) -> Result<f64, String> {
        if denominator == 0.0 {
            Err("Denominator cannot be zero!".to_string())
        } else {
            Ok(numerator / denominator)
        }
    }

    match divide_result(10.0, 0.0){
        Ok(x) => println!("Result: {}", x),
        Err(e) => println!("Error: {}", e)
    }
}
