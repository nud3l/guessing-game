fn strings() {
    let mut s = String::from("hello");

    s.push_str(", world!");

    println!("{}", s);
}

fn heap() {
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
    println!("{}", s1);
}

fn main() {
    strings();
    heap();
}
