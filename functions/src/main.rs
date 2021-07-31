fn main() {
    hello_world(); 
    fn_with_parameter(10);
    fn_with_two_parameters(20, 30);

    //
    let y = {
        let x = 3;
        x + 1 //Questa è un espressione, senza il ;. 
            // viene cosi ritornato il valore 4 a y
    };

}  

fn hello_world(){
    println!("Hello World!");
}

//function with parameters
fn fn_with_parameter(x: i32){
    println!("The value of x is: {}", x);
}

fn fn_with_two_parameters(x: i32, y: i32){
    println!("The value of x is {}", x);
    println!("The value of y is {}", y);
}

//functions with return values
fn five() -> i32{ //ritorna 5 al chiamante
    5
}

fn plus_one(x: i32) -> i32{
    x + 1
}


