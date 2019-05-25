struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    // the entire instance must be marked as mutable, not
    // only a single value
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");

    
    let user2 = User {
        email: String::from("another@examples.com"),
        username: String::from("anotherusername"),
        // struct update syntax, '..' specifies that the remaining fields not 
        // explicitly set, should have the same value as the fields in the given
        // instance
        ..user1
    };

    // tuple structs, useful when giving tuples names
    struct Color(i32,i32,i32);
    struct Point(i32,i32,i32);

    let black = Color(0,0,0);
    let origin = Point(0,0,0);

}


fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
