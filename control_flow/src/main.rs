fn main() {
    
    let number = 3;
    if number < 5{
        println!("true");
    } else{
        println!("true");
    }

    //else if
    if number % 4 == 0{
        println!("Number is divisible by 4");
    } else if number % 3 == 0{
        println!("Number is divisible by 3");
    }

    //if in a let statement
    let condition = true;
    let number = if condition {5} else { 6 };
    println!("The value of number is: {}", number);

    //loops
    /**
    loop{
        println!("again!");
    }**/

    //returing values from loops
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}:", result);



    //conditional loops 
    let mut condition = 10;

    while condition != 0 {
        println!("{}!", condition);
        condition -= 1;
    }

    //while vs for
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("The value of is: {}", a[index]);
        index += 1;
    }

    //for
    let b = [60, 70, 80, 90, 100];
    for element in b.iter(){
        println!("The value is: {}", element);
    }

    //countdown with for
    for num in (1..4).rev() {
        println!("{}!", num)
    }


}
