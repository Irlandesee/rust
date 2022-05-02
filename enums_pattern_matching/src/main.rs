struct IpAddr{
    kind: IpAddrKind,
    address: String,
}

//defining an enum
enum IpAddrKind{
    V4,
    V6,
} //an ip address can possibly be only one of 
//its variants.

enum IpAddrBetter{ //data can be put directly into the enum
    V4(String),
    V6(String),
}

enum AnotherIpAddr{
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message{
    Quit,
    Move{x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message{
    fn call(&self){
        //definition of method body
    }
}

enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState{
    Alabama,
    Alaska,
    //etc..
}


/**
 * Enum that can encode the concept of a value being 
 * present or absent.
 * Defined by the standard library
 * */
 /**
enum Option<T> {
    None,
    Some(T),
}
**/

/**
 * Enums allow to define a type by enumerating its possible
 * variants. 
 * */
fn main() {
    //Enum values
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddr{
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr{
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    //attach the data directly
    let homeTwo = IpAddrBetter::V4(String::from("127.0.0.1"));
    let loopbackTwo = IpAddrBetter::V6(String::from("::1"));


    let homeThree = AnotherIpAddr::V4(127, 0, 0, 1);
    let loopbackThree = AnotherIpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    //The Option Enum
    //The option enum type encodes the very common scneario
    //in which a value could be something or it could be
    //nothing

    let some_number = Some(5); 
    let some_string = Some("A string");
    let absent_number: Option<i32> = None;
    //Rust can infer these types because there is a 
    //specific value inside the Some variant
    //For absent_number, rust requires to annotate the
    //overall Option types

    let x: i8 = 5;
    let y: Option<i8> = Some(8);
    //let sum = x + y; -> error: different types!
    // Rust compiler ensures that there is a valid value
    // Convert Option<T> to a T
    // In order to use an Option<T> value, there needs
    // to be code that handles each variant

    //---- The match Control Flow Construct
    //Match allows to compare a value against a series of patterns
    //then execute code based on which pattern matches 

    //---- Patterns that bind to values
    //Useful feature of match arms is that they can bind to 
    //the parts of the values that match the pattern

    //--- Matching with Option<T>
    //Option<T> can be handled using match

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    //Catch-all patterns and the _ placeholder
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        //other => move_player(other), //matches all values not specifically listed
        _ => (), //express the unit value -> it tells rust that 
                //any value that doesn't match is not going to be used
    }

    //Concise Control Flow with if let
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The max is configured to be {}", max);
    }// Using if let means less typing, less indentation
    //less boilerplate code, at the cost of losing 
    //the exhaustive checking that match enforces

    let mut count = 0;
    let coin = Coin::Nickel;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count+=1,
    }
    //with if let
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state),
    }else{
        count += 1;
    }


}

fn route(ip_kind: IpAddrKind) {}

fn value_in_cents(coin: Coin) -> u8 {
    match coin{
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter {:?}!", state);
            25
        }
    }
} // an enum and a match expression that has the variants of the enum as 
// its patterns


fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn add_fancy_hat() {}

fn remove_fancy_hat() {}

fn move_player(num_spaces: u8){}

fn reroll() {}