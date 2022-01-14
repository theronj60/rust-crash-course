// struct Person {
//     name: String,
//     age: u32,
// }

// impl std::fmt::Display for Person {
//     fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
//         write!(fmt, "{} ({} years old)", self.name, self.age)
//     }
// }

// fn main() {
//     let alice = Person {
//         name: String::from("Alice"),
//         age: 30,
//     };
//     println!("Person: {}", alice);
// }

// fn main() {
//     let val: String = String::from("Hello, World!");
//     printer(&val);
//     printer(&val);
// }

// fn printer(val: &str) {
//     println!("The value is: {}", val);
// }

// Printing Numbers 1.7 pg: 15

fn main() {
    let mut i = 1;

    loop {
        println!("i == {}", i);
        if i >= 10 {
            break;
        } else {
            i += 1;
        }
    }
}
