struct Point(i32, i32, i32);
struct Vector(i32, i32, i32);

impl Vector {
    fn get_magnitude(&self) -> f64 {
        let sum_of_elems: i32 = (self.0 * self.0) + (self.1 * self.1) + (self.2 * self.2);
        let conv = sum_of_elems as f64;
        conv.sqrt()
    }
}

#[derive(Debug)]
struct User {
    name: String,
    is_cool: bool,
    id: i32,
}

impl User {
    fn make_cool(&mut self) {
        self.is_cool = true;
    }

    fn new(name: &str, id: i32) -> Self {
        Self {
            name: String::from(name),
            is_cool: false,
            id,
        }
    }
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

    // THIS PERFORMS A MOVE
    let mut user_spread: User = User {
        is_cool: true,
        ..user
    };

    user_spread.name = String::from("Jay McSpread");

    // THIS WILL PANIC (after the move)
    // println!("{}", user.name);

    println!("The results of cloning or spreading: ");
    println!("{}", user_clone.name);
    println!("{}", user_spread.name);
    println!();
    println!("Debug a struct: {user_clone:?}");
    println!("");
    println!("Make Jay's Clone cool with make_cool(): ");
    user_clone.make_cool();
    println!("Debug a struct with pretty print: {user_clone:#?}");
    println!();

    println!("Make a new struct using a new() associated function:");
    let bob: User = User::new("Bob", 5678);
    println!("Show us Bob: {bob:#?}");
    println!();

    play_with_point();
}

fn construct_vector(pt_a: &Point, pt_b: &Point) -> Vector {
    Vector(pt_b.0 - pt_a.0, pt_b.1 - pt_a.1, pt_b.2 - pt_a.2)
}

fn play_with_point() {
    let origin: Point = Point(0, 0, 0);
    let point_p: Point = Point(10, 10, 10);

    let point_po = construct_vector(&origin, &point_p);

    println!("point PO:");
    println!("x: {}, y: {}, z: {}", point_po.0, point_po.1, point_po.2);
    println!("magnitude: ≈{:.2}", point_po.get_magnitude());
}
