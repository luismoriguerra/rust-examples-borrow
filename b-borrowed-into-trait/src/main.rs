struct Person {
    name: String,
}

impl Person {
    // This trait will can automatically convert a &str into a String. 
    //If we already have a String, then no conversion happens.
    fn new<S: Into<String>>(name: S) -> Person {
        Person { name: name.into() }
    }
}

fn main() {
    let person = Person::new("Herman");
    let person = Person::new("Herman".to_string());
}
 