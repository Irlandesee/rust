fn main() {

    //creating a new vector
    let v: Vec<i32> = Vec::new();
    //type annotation is needed -> Because there arent't any
    //values being inserted into the vector at the moment,
    //Rust doesn't know what kind of elements it is intended
    //to store.
    let v_initialized = vec![1,2,3,4,5,6,7,8,9,10];

    //updating a vector
    let mut vec = Vec::new();
    vec.push(10);
    vec.push(20);

    //dropping a vector drops its elements -> a vector is 
    //freed when it goes out of scope

    //reading elemtents of vectros
    let second: &i32 = &vec[1];
    println!("The second element is {}", second);

    match v.get(1){
        Some(second) => println!("The second element is {}", second),
        None => println!("There is no second element")
    }

    //attempting to add an element to a vector while holding
    //a reference to an item -> error
    let mut another_vector = vec![10,20,30,40,50];
    let first = &another_vector[0];
    another_vector.push(60); //ERROR
    //Because vectors put the values next to each other in memory,
    //adding a new element onto the end of the vector might require
    //allocating new memory and copying the old elements to the new
    //space, if there isnt't enough room to put all the elements
    //next to each other where the vector is currently stored. In 
    //that case, the reference to the fist element would be 
    //pointing to deallocated memory, borrowing rules prevent
    //programs from ending up in that situation.
    
    //iterating over the values in a vector

    for i in &v{ println!("{}", i);} //printing each element 
    //iterating over mutable references to elements in a vector
    for i in &mut another_vector{*i += 50;}
    //changing the value that the mutable reference refers to, using
    //the dereference operator

    //using an enum to store multiple values
    enum SpreadsheetCell{
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("hello")),
        SpreadsheetCell::Float(10.12),
    ];

    //Rust needs to know what types will be in the vector at 
    //compile time so it knows exactly how much memory on the heap 
    //will be needed to store each element.
    //If rust allowed a vector to hold any type, there would be 
    //a chance that one or more of the types would cause errors
    //with the operations performed on the elements of the vector
    //Using an enum plus a match expression means that rust will ensure
    //at compile time that every possible case in handled

}

