fn main(){
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // the & creates a refrences that refers to the value of s1
                                     // It does not own it -> the vlaue it points to will not be
                                     // dropped when the reference goes out of scope.

    println!("The lenght of '{}' is {}.", s1, len);
    
    //mutable references -> there can only be onw mutable refrences to a piece of data in scope.
    change(&mut s); //This restriction allows for mutation but in a controlled fashion.

}

fn calculate_length(s: &String) -> usize{// s is reference to a String: These parameters (references)
                                         // are borrowed (borrowing).
    s.len()
}

fn change(some_string: &mut String){
    some_string.push_str(", world");
}
