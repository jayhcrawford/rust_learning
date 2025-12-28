struct Person {
    name: String,
    age: i32,
}
enum Relationship {
    Myself(Person),
    Friend(Person),
    Family(Person),
    Foe(Person),
}

impl Relationship {
    fn talk_about(&self) {
        match self {
            Relationship::Myself(person) => println!(
                "{} is my name, and I could talk about myself all day!",
                person.name
            ),
            Relationship::Friend(person) => println!("{} is my friend!", person.name),
            Relationship::Family(person) => println!("{} is my family member!", person.name),
            Relationship::Foe(person) => {
                println!(
                    "{} is my sworn enemy!! I hate {}!",
                    person.name, person.name
                )
            }
        }
    }
}

fn main() {
    let bob: Person = Person {
        name: String::from("Bob"),
        age: 30,
    };
    let chaz: Person = Person {
        name: String::from("Chaz"),
        age: 24,
    };
    let bing_ballio: Person = Person {
        name: String::from("Bing Ballio"),
        age: 99,
    };
    let me: Person = Person {
        name: String::from("Jay"),
        age: 32,
    };

    print!("{}'s age is {}", bob.name, bob.age);

    let bestie = Relationship::Friend(bob);
    let brother = Relationship::Family(chaz);
    let worst_enemy = Relationship::Foe(bing_ballio);
    let myself = Relationship::Myself(me);

    println!();
    bestie.talk_about();
    brother.talk_about();
    worst_enemy.talk_about();
    myself.talk_about();
}
