struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
fn main() {
    {
        let mut user1 = User {
            email: String::from("someone@example.com"),
            username: String::from("someusername123"),
            active: true,
            sign_in_count: 1,
        };
        user1.email = String::from("anotheremail@example.com");
    }

    {
        let user1 = build_user(String::from("foo@example.com"), String::from("foo"));
    }

    {
        let user1 = User {
            email: String::from("someone@example.com"),
            username: String::from("someusername123"),
            active: true,
            sign_in_count: 1,
        };

        // let user2 = User {
        //     email: String::from("another@example.com"),
        //     username: String::from("anotherusername567"),
        //     active: user1.active,
        //     sign_in_count: user1.sign_in_count,
        // };
        let user2 = User {
            email: String::from("another@example.com"),
            username: String::from("anotherusername567"),
            ..user1
        };
    }

    {
        struct Color(i32, i32, i32);
        struct Point(i32, i32, i32);

        let black = Color(0, 0, 0);
        let origin = Point(0, 0, 0);
    }

    {
        // TODO: unit-like struct
    }

    {
        // error[E0106]: missing lifetime specifier
        // struct User {
        //     username: &str,
        //     email: &str,
        //     sign_in_count: u64,
        //     active: bool,
        // }
        // let user1 = User {
        //     email: "someone@example.com",
        //     username: "someusername123",
        //     active: true,
        //     sign_in_count: 1,
        // };
    }
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
