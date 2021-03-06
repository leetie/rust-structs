mod area;

fn main() {
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }
    fn build_user(email: String, username: String) -> User {
        User {
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    }
    let mut user_1 = User {
        email: String::from("test@example.com"),
        username: String::from("leetie"),
        active: true,
        sign_in_count: 1,
    };

    //initialize new mutable User struct, sharing some values from user_1
    // using Struct update syntax
    let mut user_2 = User {
        email: String::from("user2@example.com"),
        username: String::from("user_2"),
        ..user_1
    };
    // use function to initialize User struct
    let user_3 = build_user(
        String::from("bezos@amazon.com"),
        String::from("bigdaddyjeffXx420"),
    );
    let user_4 = User {
        // use a &reference to user_1 here as not to take ownership.
        // String::from will return ownership of data to user_1 after
        // function.
        username: String::from(&user_1.username),
        email: String::from("user4@example.com"),
        active: true,
        sign_in_count: 1,
    };

    println!("user 1 email: {}", user_1.email);
    println!("user 2 username: {}", user_2.username);
    println!("changing user 2...");
    user_2.username = String::from("eddy");
    println!("user 2's new username: {}", user_2.username);
    println!(
        "User 3's email and username: {} {}",
        user_3.email, user_3.username
    );
    println!("user 4: {} {}", user_4.username, user_4.email);

    println!("changing user 1's username");
    user_1.username = String::from("leetus");
    println!(
        "Now user 1's username is {}, and user 4's username is {}",
        user_1.username, user_4.username
    );

    let heap_string = String::from("heap string");
    let heap_string_2 = &heap_string; // copies the existing pointer here, but heap_string retains ownership. when this reference is out of scope, the pointer is not dropped

    println!("{}", heap_string);
    // line below will not compile because heap_string_2 is borrowing the data in heap_string
    //heap_string = String::from("will not work");
    println!("{}", heap_string_2);

    // FROM BOOK https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html
    //     fn calculate_length(s: &String) -> usize { // s is a reference to a String
    //     s.len()
    // } // Here, s goes out of scope. But because it does not have ownership of what
    //   // it refers to, nothing happens.

    // tuple structs
    // these two structs are different types, despite both being tuple structs -- Coords & ArbitraryValues. Even though they are both made up of two i32 values, fn some_fn(arg: Coords) {..} could not take ArbitraryValues as an argument
    struct Coords(f64, f64);
    struct ArbitraryValues(f64, f64);
    let nh = Coords(42.8, -70.0);
    let some_data = ArbitraryValues(120.1, 42.0);
    // dot syntax is valid for tuple structs
    println!("{}", nh.0);

    let rect1 = area::Rectangle {
        width: 30,
        height: 50,
    };
    println!("The area of rect1 = {}", rect1.area());
    println!("rect1 is {:#?}", rect1);
}

// fn main_2() {
//     // Data below is related, let's use a tuple instead!
//     // let width1: u32 = 30;
//     // let height1: u32 = 50;
//     // let rect1 = (30, 50);
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(&rect1)
//     );
//     // area fn using tuple syntax. can be improved using structs for code clarity
//     // fn area(dimensions: (u32, u32)) -> u32 {
//     //     dimensions.0 * dimensions.1
//     // };

//     // area fn using struct parameter. use reference as to not take ownership
//     fn area(rectangle: &Rectangle) -> u32 {
//         rectangle.width * rectangle.height
//     }
//     struct Rectangle {
//         width: u32,
//         height: u32,
//     };
// }
