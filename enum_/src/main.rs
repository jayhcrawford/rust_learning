struct Person {
    name: String,
    age: i32,
}

impl Person {
    fn about_age(&self) {
        print!("{}'s age is {}", self.name, self.age);
    }
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

    fn relationship(&self) -> String {
        match self {
            Relationship::Family(_p) => String::from("family member"),
            Relationship::Friend(_p) => String::from("friend"),
            Relationship::Foe(_p) => String::from("sworn enemy"),
            Relationship::Myself(_p) => String::from("myself"),
        }
    }

    fn name(&self) -> String {
        match self {
            Relationship::Family(p) => p.name.clone(),
            Relationship::Friend(p) => p.name.clone(),
            Relationship::Foe(p) => p.name.clone(),
            Relationship::Myself(p) => p.name.clone(),
        }
    }
}

fn loan_interest(relationship: Relationship) -> f32 {
    match relationship {
        Relationship::Family(_person) => 0.0,
        Relationship::Friend(_person) => 3.0,
        Relationship::Foe(_person) => 40.0,
        Relationship::Myself(_person) => -5.0,
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

    chaz.about_age();

    let bestie = Relationship::Friend(bob);
    let brother = Relationship::Family(chaz);
    let worst_enemy = Relationship::Foe(bing_ballio);
    let myself = Relationship::Myself(me);

    println!();
    bestie.talk_about();
    brother.talk_about();
    worst_enemy.talk_about();
    myself.talk_about();

    println!(
        "I would loan interest to my {} {} at {}%!",
        bestie.relationship(),
        bestie.name(),
        loan_interest(bestie)
    );
    println!(
        "I would loan interest to my {} {} at {}%!",
        worst_enemy.relationship(),
        worst_enemy.name(),
        loan_interest(worst_enemy)
    );
}
