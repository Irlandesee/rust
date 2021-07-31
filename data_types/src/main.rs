fn main() {
    //scalar types:  Integers, Float, booleans and chars
    //Compound Types

    //tuple type
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);


    //The array type
    let a = [1, 2, 3, 4, 5];
    let b: [i32; 5] = [6, 7, 8, 9, 10];
    let _c = [3; 5];
    //accessing Array elements

    let _first = a[0];
    let _second = b[1];


}
