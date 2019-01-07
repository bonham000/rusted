
#[derive(Debug)]
struct Person {
    name: String,
    age: i32,
    occupation: String,
}

fn main() {
    println!("Starting");

    let mark = Person {
        age: 32,
        name: String::from("Mark"),
        occupation: String::from("CEO")
    };
    
    let mut mark_clone = Person {
        age: 32,
        name: String::from("Mark"),
        occupation: String::from("CEO")
    };

    let steve = Person {
        age: 45,
        name: String::from("Steve"),
        occupation: String::from("QA"),
    };

    println!("{:?}", mark);
    println!("{:?}", steve);

    let same: bool = people_eq(&mark, &steve);
    println!("Are mark and steve the same? {}", same);

    let same: bool = people_eq(&mark, &mark_clone);
    println!("Are mark and mark the same? {}", same);

    mark_clone.name = String::from("Joe");

    let same: bool = people_eq(&mark, &mark_clone);
    println!("Are mark and mark the same? {}", same);
}

fn people_eq(person_one: &Person, person_two: &Person) -> bool {
    if person_one.age != person_two.age {
        return false;
    } else if person_one.name != person_two.name {
        return false;
    } else if person_one.occupation != person_two.occupation {
        return false;
    }

    true
}