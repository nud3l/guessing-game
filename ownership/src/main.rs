fn main() {
    strings();
    moving();
    cloning();

    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was
  // moved, so nothing happens. s1 goes out of scope and is dropped.    

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("hello"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}


fn strings() {
    let mut s = String::from("hello");

    s.push_str(", world!");

    println!("{}", s);
}

fn moving() {
    // assign 5 to x and make a copy of x and assign it to y
    // both x and y are stored on the stack
    let x = 5;
    let y = x;

    // s1 stores pointer, length, capacity on the stack
    // pointer stores the string on the heap
    // s2 stores pointer, length, capacity on the stack
    // pointer of s1 and s2 to the same heap location
    let s1 = String::from("hello");
    let s2 = s1;
    
    // below won't compile since s1 is invalidated when
    // assigning s2 to the heap location of the string
    // created with s1
    // Concept is called "move" in Rust
    // It invalidates s1 and safely stores the pointer in s2
    // println!("{}", s1);
}

fn cloning() {
    let s1 = String::from("hello");
    // explicitly clone stack and heap data
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}

