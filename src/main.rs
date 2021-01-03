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
}
