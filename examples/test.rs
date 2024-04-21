struct Person {
    name: &'static str,
    age: u32,
}

fn main() {
    let alice = Person {
        name: file!(),
        age: 30,
    };

    println!("Name: {}, Age: {}", alice.name, alice.name.len());
}
