// A struct, structure, is a custom data type that lets you package together and name multiple related values that make up a meaningful group.

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
// If a field is never read because `#[warn(dead_code)]` is on by default

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    user1.active = false; // Note that to mutate a field the entire instance must be mutable; Rust doesn't allow us to mark only certain fields as mutable.
    /*
    fn build_user(email: String, username: String) -> User {
        User {
            active: true,
            username: username,
            email: email,
            sign_in_count: 1,
        }
    }
    fn build_user(email: String, username: String) -> User {
        User {
            active: true,
            username,
            email,
            sign_in_count: 1,
        }
    }

    These are the same

    let user2 = User {
            active: user1.active,
            username: user1.username,
            email: String::from("another@example.com"),
            sign_in_count: user1.sign_in_count,
        }
        let user2 = User {
            email: String::from("another@example.com"),
            ..user1
        }
    And these also
    */



    // Note that the struct update syntax ( i.e. the second case ) uses = like an assignment; this is because it moves the ownership of the data. In this example, we can no longer use user1 after creating user2 because the String in the username field of user1 was moved into user2.
    // In the context of Rust's ownership system, when you use the struct update syntax, the fields that are moved cannot be used after the move. However, fields that implement the Copy trait can still be used.



    // Tuple Structs

    // Tuple structs are useful when you want to give the whole tuple a name and make the tuple a different type from other tuples.

    struct Color(i32, i32, i32);
    let black = Color(0, 0, 0);
}