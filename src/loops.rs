fn main(){
    
    // loop{
    //     println!("Hello world")
    // }


    // let result = loop{
    //     counter += 1;
    //     if counter == 10 {
    //         break counter * 2;
    //     }
    // };

    // println!("The result is {}", result)


    let mut counter = 0;

    'counting_up: loop {
        println!("Count = {counter}");
        let mut remaining: i32 = 10;

        loop {
            println!("Remainig: {remaining}");

            if remaining == 9 {
                break;
            }
            if counter == 2 {
                break 'counting_up;
            } 
            remaining -= 1;
        }
        counter += 1;
    }

    // While loop
    let mut number = 3;
    while number != 0 {
        println!("{number}");
        number -= 1;
    }

    println!("HEY!!!");

    // Looping through a collection

    let a = [1,2,3,4,5,6];

    for element in a {
        println!("{element}")
    }


}