fn main() {




    //Struct
    struct Book{
        title: String,
        author: String,
        pages: u32,
        availability: bool,
    }

    #[derive(Debug)]
    struct User{
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    let mut user1 = User{
        active: true,
        username: "favour".to_string(),
        email: "test@gmail.com".to_string(),
        sign_in_count: 10,
    };

    user1.email = String::from("example@mail.com");

    println!("{:?}", user1);

    let user2 = User{
        email: "user2@gmail.com".to_string(),
        ..user1
    };

    //Tuple Structs
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black: Color  = Color(0,0,0);
    let white: Color  = Color(255,255,255);



}