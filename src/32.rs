use std::fs::File;
use std::io::{self, BufRead};
fn main() -> io::Result<()> {
    let file = File::open("data.txt")?;
    use serde::Deserialize;

    #[derive(Deserialize)]
    struct Person {
        name: String,
        age: i32,
    }

    let persons = match serde_json::from_reader(&file) {
        Ok(persons) => persons,
        _ => panic!("Failed to parse data"),
    };

    // Sort the list of people by age in descending order
    let mut sorted_people = persons.clone();
    sorted_people.sort_by_key(|person| person.age.desc());

    for (index, &person) in sorted_people.iter().enumerate() {
        println!("{:?}", person);
    }

    Ok(())
}
