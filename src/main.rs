use enum_dispatch::enum_dispatch;

#[enum_dispatch(People)]
trait Person {
    fn say_hello(&self);
}
//implementor struct 1
struct Me {
    name: &'static str,
}

impl Person for Me {
    fn say_hello(&self) {
        println!("Hello, it's me.")
    }
}

#[warn(dead_code)]
struct Grandma {
    age: usize
}

impl Person for Grandma {
    fn say_hello(&self) {
        println!("G'day!")
    }
}

#[enum_dispatch]
enum People {
    Grandma(Grandma),
    Me(Me)
}
//I thought enum_dispatch generated this impl for us automatically, so comment out
//impl Person for People {
//    fn say_hello(&self) {
//        match self {
//           People::Grandma(grandma) => grandma.say_hello(),
//            People::Me(me) => me.say_hello()
//        }
//    }
//}

struct PeopleZoo<P: Person> {
    people: Vec<P>,
}

impl<P: Person> PeopleZoo<P> {
    fn add_person(&mut self, person: P) {
        self.people.push(person);
    }

    fn last_person(&self) -> Option<&P> {
        self.people.last()
    }
}

fn main() {
    let mut zoo: PeopleZoo<People> = PeopleZoo { people: vec![] };
    zoo.add_person(People::Me(Me { name: "Bennett" }));

    if let Some(People::Me(me)) = zoo.last_person() {
        println!("My name is {}.", me.name)
    }
}