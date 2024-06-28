// A struct, structure, is a custom data type that lets you package together and name multiple related values that make up a meaningful group.

// Rust uses PascalCase for naming types and traits ( as in the structs ), where each word starts with a capital letter, for more info: https://doc.rust-lang.org/1.0.0/style/style/naming/README.html

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
    #[derive(Debug)]
    struct Color(i32, i32, i32);
    let black = Color(0, 0, 0);

    println!("{:?}", black);

    // You can also define structs that don't have any fields! These are called "unit-like structs".
    // Unit-like structs can be useful when you need to implement a trait on some type but don't have any data that you want to store in the type itself. We'll discuss traits in Chapter 10.

    // struct AlwaysEqual;
    // let subject = AlwaysEqual;
    // But you can also add {} to these two, but it will be an empty braced struct.
    // For more: https://stackoverflow.com/questions/50162597/what-are-the-differences-between-the-multiple-ways-to-create-zero-sized-structs/50170328#50170328
    // And: https://chatgpt.com/share/a0b14892-66ab-4b93-a014-60febd61e01c

    // You can't use &str in it until you reach Chapter 10 at lifetimes.
    // If you took a whole struct as an argument and you don't implement the copy trait, its ownership will be transferred



    // The println! macro can do many kinds of formatting, and by default, the curly brackets tell println! to use formatting known as Display: output intended for direct end user consumption. The primitive types we've seen so far implement Display by default because there's only one way you'd want to show a 1 or any other primitive type to a user. But with structs, the way println! should format the output is less clear because there are more display possibilities, so Rust doesn't try to guess what we want, and structs don't have a provided implementation of Display to use with println! and the {} placeholder.

    // But you can use :? inside the {} to print our struct in a way that is useful for developers, but you want to implement the Debug trait to do this OR add `#[derive(Debug)]` just before the struct definition.
    // For output that is easier to read :#?.



    // Another way to print out a value using the Debug format is to use the dbg! macro ( it also requires a Debug trait or using or derive it using the attribute ), which takes ownership of an expression ( as opposed to println!, which takes a reference, but dbg! can also take references without any problem ), prints the file and line number of where that dbg! macro call occurs in your code along with the resultant value of that expression, and returns ownership of the value.
    // Calling the dbg! macro prints to the standard error console stream ( stderr ), as opposed to println!, which prints to the standard output console stream ( stdout ). for more info page 270.
    dbg!(&black);


    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    impl Rectangle {
        fn area(self: &Rectangle) -> u32 {
            // self: &Rectangle = self: &Self = &self
            self.width * self.height
        }
    }
    println!("{}", rect1.area());


    // Methods are similar to functions: we declare them with the fn keyword and a name, they can have parameters and a return value, and they contain some code that's run when the method is called from somewhere else. Unlike functions, methods are defined within the context of a struct ( or an enum or a trait object, which we cover in Chapter 6 and Chapter 17, respectively ), and their first parameter is always self, which represents the instance of the struct the method is being called on.

    // Within an impl block, the type Self is an alias for the type that the impl block is for. Methods must have a parameter named self of type Self for their first parameter, so Rust lets you abbreviate this with only the name self in the first parameter spot.

    // We can choose to give a method the same name as one of the struct's fields or of another function.



    // Rust doesn't have an equivalent to the -> operator ( which is used in C and C++ to call a method on a pointer to an object ); instead, Rust has a feature called automatic referencing and dereferencing. Calling methods is one of the few places in Rust that has this behavior.
    // When you call a method with object.something, Rust automatically adds in &, &mut, or * so object matches the signature of the method.
    // p1.distance(&p2); = (&p1).distance(&p2);


    // All functions defined within an impl block are called associated functions because they're associated with the type named after the impl. We can define associated functions that don't have self as their first parameter ( and thus are not methods ), because they don't need an instance of the type to work with. We've already used one function like this: the String::from function that's defined on the String type.

    // Each struct is allowed to have multiple impl blocks.
}