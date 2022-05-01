struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
struct AlwaysEqual;

fn main(){
    let user = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let mut other_user = User{
        email: String::from("someotheremail@example.com"),
        username: String::from("someotherusername123"),
        active: true,
        sign_in_count: 1,
    };

    other_user.email = String::from("anotheremail@example.com");

    //creating istances from other instances with struct update syntax
    let user2 = User{
        email: String::from("another@example.com"),
        ..user
    }; //using struct update syntax to set a new email value
    //for a user instance but use the rest of the values from user1    


    //using tuple structs without named fields to craete different types
    let origin = Point(0, 0, 0);
    let x = origin.0;
    //println!("{}", x);

    //unit like structs without any fields
    let subject = AlwaysEqual;


    //ownership of struct data
    //It is possible for structs to store references to data
    //owned by something else. Doing so requires the use of 
    //lifetimes -> Lifetimes ensure that the data referenced
    //by a struct is valid for as long as the struct is.

}

/**
 * function that uses field init shorthand
 * */
fn build_user(email: String, username: String) -> User{
    User{
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}