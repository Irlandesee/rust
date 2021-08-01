# OwnerShip
In rust memory is managed through a system of ownership with a set of rulesthat the compiler checks at compile time. None of the ownership features slow down  the program while it is running.

# The stack and the heap

In a systems programming language like rust, whether a value is on the stack or the heap has more effect on how the language behaves.  Both stack and heap are parts of memory available to the code at runtime, but they are structured in different ways.
# Stack
The stack stores the values in the order it gets them and removes the values in the opposite order (LIFO). All data stored on the stack must have a known, fixed size. Data with an unknown size at compile time or a size that might change must be stored on the heap instead. 

# Heap
When data is put on the heap, a certain amount of space is requested. The memory allocator finds an empty spot in the heap that is big enough, marks it as being in use, and returs a pointer. This process is called allocating on the heap (allocating). Because a pointer is a known fixed size, it can be stored on the stack.

When the code calls a function, the values passed into the function (including potentially poiunters to data on the heap) and the function's local variables get pushed onto the stack; when the function is over, those values get popped off the stack.

# Ownership rules
 * Each value in rust has a variable that is called its owner.
 * There can only be one owner at a time.
 * When the owner goes out of scope, the value will be dropped.

# Variable scope
A scope is the range within a program for which an item is valid. 
example: let s = "hello";
-> The variable s is valid from the point at which it is declared until the end of the current scope.

# The String type
A second String type, which value cannot be known ahead of time. This type is allocated on the heap and as such is able to store an amount of text that is uknown to the programmer at compile time.

example: using the from function
-> let s = String::from("hello")
Note: the "::" operator allows to namespace this particular from function from under the String type rather than using some sort of nale like string_from.

This kind of String can be muted
example:

let mut s = String::from("hello");
s.push_str(", world"); //push_str() appends a literal to a String
println!("{}", s);

# Memory and Allocation

With the String type, in order to supporet a mutable, growable piece of text, it is necessary to allocate  an amount of memory on the heap, unknown at compile time, to hold the contents:
-> The memory must be requested from the memory allocator at runtime.
-> There needs to be a way of returning the memory once it has finished its purpose.

First part done by: String::from -> its implementation requests the memory it needs.

The second part: In languages with a garbage collector, the gc keeps track and cleans up memory that isn't used anymore. Without the gc, it's the programmers responsibility
to indentify when memory is no longer being used and call code to explicity return it.

Rust takes a different path: the memory is automatically returned once the variable that owns it goes out of scope: Rusts calls a special function, drop. Drop is called automatically at the closing curly bracket.
-> In c++ this pattern of deallocating memory is called Resource Acquisition Is Initialization (RAII).

# Clone
function clone: deeply copies not just the "stack" data of a String but even the "heap" data.

-> example:

let s1 = String::from("hello");
let s2 = s1.clone();
println!("s1 = {}, s2 = {}");


# copy
Rust hash a spcieal annotation called the copy trait that the the programmer can place on types like integers that are stored on the stack. If a type implements the Copy trait, an older variable is till usable after assignment.  Rust won't let the user annotate a type with the copy trait if the type, or any of its parts, has implemented the Drop trait. If the type needs something special to happen when the value goes out of scope and the Copy annotation is added to the type, a compile-time error is generated. 
Any group op simple scalar values can implement copy, and nothing that requires allocation or is some form of resource can implement Copy.

Types that implement copy:
* All Integers types , such as u32
* The boolean type
* All the floating point types, such as f64
* The character type, char
* tuples, if the only contain types that also implement copy, (i32, i32)

# Ownership and functions

Passing a variable to a funciton will move or copy, just as assignment does.

# Return values and scope

Returning values can also transfer ownership: The ownership of a value follows the same pattern every time: assigning a value to another variable moves it. When a variable that includes data on the heap goes out of scope, the value will be cleaned up by drop unless the data has been moved to be owned by another variable.














