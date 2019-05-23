fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("hello");
    change(&mut s);

    {
        let r1 = &mut s;
        println!("I am the mutable reference r1:{}",r1);
    }
    let r2 = &mut s;
    println!("I am the mutable reference r2:{}",r2);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(a_string: &mut String) {
    a_string.push_str(", world!");
}
