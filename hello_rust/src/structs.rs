// Used to create custom data types
// Similar to Classes

// Traditional struct
struct Color {
    red: u8,
    green: u8,
    blue: u8
}

// Tuple struct
struct Char(u8, u8, u8);

// Person struct
struct Person {
    first_name: String,
    last_name: String,
    age: u8
}

impl Person {
    // Construct Person
    fn new(first_name: &str, last_name: &str, age: u8) -> Person {
        Person {
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            age: age
        }
    }

    // Get full name
    fn full_name(&self) -> String {
        format!("{} {} {} years old", self.first_name, self.last_name, self.age)
    }

    // Set last name
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    // Name to tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}


pub fn run() {
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0
    };

    c.blue = 100;

    println!("COLOR: {} {} {}", c.red, c.green, c.blue);

    let mut npc = Char(110, 80, 80);

    println!("NPC Stats: {} {} {}", npc.0, npc.1, npc.2);

    let mut person = Person::new("Beka", "Modd", 32);
    println!("new person is {} {} and is {} years old", person.first_name, person.last_name, person.age);
    println!("same person from the class method is: {}", person.full_name());

    // We'll change persons last name
    person.set_last_name("Modebadze");
    println!("Now with changed last name: {}", person.full_name());

    // Name as tuple
    println!("Name: {:?}", person.to_tuple());
}