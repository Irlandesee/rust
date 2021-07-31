fn main() {
    //variables and mutability

    let mut x = 5; //senza il mut da un errore in compilazione
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);

    //constants

    const MAX_POINTS: u32 = 100000;
    println!("Max points: {}", MAX_POINTS);

    // shadow variables

    let y = 5;
    let y = y + 1;
    let y = y * 2;
    println!("The value of y is: {}", y);

    //esempio
    let spaces = "   ";
    let spaces = spaces.len();
    println!("The value of spaces is: {}", spaces);


}
