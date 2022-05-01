fn main() {

    //A string slice is a reference to a part of a String
    let s = String::from("Hello World");
    let len = s.len();
    let hello = &s[0..5];
    let slice = &s[3..len];

}

fn first_word(s: &String) -> &str{

    let bytes = s.as_bytes();
    for(i, &item) in bytes.iter().enumerate(){
        if item == b' ' {return &s[0..i];}
    }
    &s[..]
}