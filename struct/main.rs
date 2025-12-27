struct User {
    name: String,
    is_cool: bool,
    id: i32,
}

impl Clone for User {
    fn clone(&self) -> Self {
        User {
            name: self.name.clone(),
            is_cool: self.is_cool,
            id: self.id,
        }
    }
}

fn main() {
    let user = User {
        name: String::from("Jay"),
        is_cool: false,
        id: 1234,
    };

    let mut user_clone: User = user.clone();

    user_clone.name = String::from("Jay's Clone");

    let mut user_spread: User = User {
        is_cool: true,
        ..user
    };

    user_spread.name = String::from("Jay McSpread");

    println!("{}", user_clone.name);
    println!("{}", user_spread.name);
}
