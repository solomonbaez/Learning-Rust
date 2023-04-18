fn main() {
    let input = String::from("hello world");

    let output = word_len(&input);

    println!("'{}' contains the word: {}", input, output);
}
// define fn to take a string slice, works on literals and Strings
// api is more general and useful
fn word_len(s: &str) -> &str {
    let bytes = s.as_bytes();

    // item requires a reference from enum tuple
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // return the first letter if a word is found
            return &s[..i];
        }
    }
    // return the string if no space is found
    &s[..]
}

