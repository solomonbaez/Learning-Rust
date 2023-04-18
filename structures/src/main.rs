// define a user profile structure
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    // define user 1
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    // define user 2 using the build function
    let user2 = build_user(String::from("anotherusername123"),
                            String::from("anothersomeone@example.com"));
    // define user 3 by inheriting user 2's structure
    let user3 = User {
        username: String::from("athirdusername123"),
        email: String::from("athirdsomeone@example.com"),
        ..user2
    };
    // report and drop user data to prevent lifetime errors
    println!("Active Users are: {}, {}, {}", user1.username, user2.username, user3.username);
}

// function to build a new user
fn build_user(email: String, username: String) -> User{
    // return the constructed user profile
    // field init shorthand used
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}