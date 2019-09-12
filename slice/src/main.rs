fn main() {
    let my_string = String::from("This is a test string");

    let word = first_word(&my_string[..]);

    println!("My word is: {}", word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
