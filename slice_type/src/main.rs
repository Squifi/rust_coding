fn main() {

    let mut s = String::from("squifi is learning rust");
    let word = first_word(&s); 
    println!("Sadly only contains the length of the first word being: {}", word);

    s.clear(); // this empties the String, making it equal to ""
    println!("The original String is just gone: {}",s);

    let s1 = String::from("squifi is getting along a little better");
    let word = first_word_with_slice(&s1);
    println!("Hope this works as expected: {}", word);

}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    // enumerate wraps the bytes element as a tuple of (index, byte)
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // Hence we can return the index here
            return i;
        }
    }

    // Otherwise return the length of the String
    s.len()
}

// string slice is written as &str
fn first_word_with_slice(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i]
        }
    }

    // Otherwise return the entire string as a slice, 
    // [..] omits the start and end index
    &s[..]
}
