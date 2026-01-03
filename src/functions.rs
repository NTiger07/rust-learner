struct BankAccount{
    owner: String,
    balance: f64,
}

impl BankAccount {
    fn withdraw(&mut self, amount:f64){
        println!("Withdrawing {} from account owned by {}", amount, self.owner);
        self.balance -= amount;
    }

    fn check_balance(&self){
        println!("Account owned by {} has a balance of ${}", self.owner, self.balance);
        // return self.balance;
    }
}

fn main() {
    hello_world();
    tell_height(65);
    human_id("Favour", 20, 160.1234);
    let _X = {
        let price: u32 = 5;
        let qty: u32 = 10;
        price * qty
    };
    println!("Result is {}", _X);
    let y = add(4, 6);
    println!("{}", y);
    immutability();
    let mut account = BankAccount{
        owner: "Favour".to_string(),
        balance: 1000.00
    };

    //Immutable borrow
    account.check_balance();

    //Mutable borrow
    account.withdraw(300.00);

    account.check_balance();

}

fn hello_world(){
    println!("Hello, Rust");
}

fn tell_height(height: u32){
    println!("My height is {}", height)
}

fn human_id(name: &str, age:u32, height:f32){
    println!("My name is {}, I am {} years old and my height is {:.2}cm", name, age, height)
}

fn add(a: i32, b:i32) -> i32{
    a + b
}

fn immutability(){
    let mut _x:i32 = 5;
    let _r:&mut i32 = &mut _x;

    *_r += 1;
    *_r -= 3;

    println!("Value of _r: {}", _r);
}