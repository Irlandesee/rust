# REFERENCES AND BORROWING

& -> ampersands are references: They allow to refer to some value without taking ownership
of it. 

#Mutable restrictions
There can only be one mutable reference to a particular piece of data in a particular piece
of scope. Mutation is allowed but controlled. 
For this reason, Rust can prevent data races at compile time.
Data race: similar to a race condition and happens when these 3 behaviors occur:
1. 2 or more pointers access the same data at the same time.
2. At least 1 of the pointers is being used to write to the data.
3. There is no mechanism being used to synchronize access to the data.
Data races can cause undefined behavior and are difficult to diagnose.

Rust won't compile with data races.

# Dangling References
Dangling Pointer: a pointer that references a location in memory that may have been given to 
someone/something else, by freeing some memory while preserving a pointer to that memory.
The Rust compiler guarantees that there will never be dangling references.

# Rules of References
1. At any given time, you can have either one mutable reference or any number of immutable
   references.
2. References must always be valid.

