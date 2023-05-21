fn main() {
    let user1 = create_user(String::from("🥵"), String::from("itsme@mar.io"));

    let mut user2 = create_user(String::from("Super 🥵"), user1.email[..].to_string());

    user2.sign_in_count = 42;
    // user2.active = false;

    let users: [User; 2] = [user1, user2];

    for user in &users {
        if user.active {
            println!(
                "{} - {} - {}",
                user.username, user.email, user.sign_in_count
            );
        };
    }

    let rect = Rect {
        height: dbg!(10 * 3),
        width: 12,
    };

    println!("");
    println!("{:?}", rect);
    println!("area : {} - area (method) : {}", area(&rect), rect.area());

    rect.width();

    let other_rect = Rect {
        height: 15,
        width: 10,
    };

    println!("rect can hold other_rect : {}", rect.can_hold(&other_rect));
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn create_user(username: String, email: String) -> User {
    User {
        email,
        username,
        sign_in_count: 1,
        active: true,
    }
}

#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

fn area(r: &Rect) -> u32 {
    r.width * r.height
}

impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn width(&self) -> () {
        if self.width > 0 {
            println!("width is {}", self.width);
        } else {
            println!("width invalid");
        }
    }
    fn can_hold(&self, other: &Rect) -> bool {
        self.width > other.width && self.height > other.height
    }
}
