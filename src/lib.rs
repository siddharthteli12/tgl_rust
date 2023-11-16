use std::cmp::Ordering;

#[derive(Debug)]
pub struct Person {
    pub uid: u64,
    pub name: String,
    pub age: u16,
    pub height: u16,
}

impl Person {
    pub fn build(uid: u64, name: String, age: u16, height: u16) -> Self {
        Self {
            uid,
            name,
            age,
            height,
        }
    }
}

// Default trait impl for Person
/// # Examples
/// ```
/// use tgl_rust::Person;
/// # fn main() {
/// let person = Person::default();
/// assert_eq!(person, Person {uid: 0,name: String::new(), age: 0, height: 0});
/// }
/// ```
impl Default for Person {
    fn default() -> Self {
        Self {
            uid: 0,
            name: String::new(),
            age: 0,
            height: 0,
        }
    }
}

// Clone trait impl for Person
/// # Examples
/// ```
/// use tgl_rust::Person;
/// # fn main() {
/// let person = Person::default();
/// let person2 = person.clone();
/// assert_eq!(person, person2);
/// let mut person3 = Person { uid: 0, name: String::from("test"), age: 10, height: 150};
/// person3.clone_from(&person2);
/// assert_eq!(person2, person3);
/// }
/// ```
impl Clone for Person {
    fn clone(&self) -> Self {
        Self {
            uid: 0,
            name: self.name.clone(),
            age: self.age.clone(),
            height: self.height.clone(),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.name = source.name.clone();
        self.age = source.age.clone();
        self.height = source.height.clone();
    }
}

// PartialEq trait impl for Person
/// # Examples
/// ```
/// use tgl_rust::Person;
/// # fn main() {
/// let person = Person::default();
/// let person2 = Person::build(0, "Sid".to_string(), 10, 120);
/// assert_eq!(person, person2);
/// let person3 = Person::build(100, "Sid".to_string(), 10, 120);
/// assert_ne!(person2, person3);
/// }
/// ```
impl PartialEq for Person {
    fn eq(&self, other: &Self) -> bool {
        self.uid == other.uid
    }
    fn ne(&self, other: &Self) -> bool {
        self.uid != other.uid
    }
}

// Eq trait impl for Person
// Now, Person can act as keys in HashMap, which as to impl Eq trait.
impl Eq for Person {}

// PartialOrd trait impl for Person
// This trait returns option & is impl for types where sometimes comparsion might make no sense
// resulting in returning none.
// PartialEq trait impl for Person
/// # Examples
/// ```
/// use tgl_rust::Person;
/// # fn main() {
/// let person = Person::default();
/// let person2 = Person::build(10, "Sid".to_string(), 10, 120);
/// assert!(person2 > person);
/// }
/// ```
impl PartialOrd for Person {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.uid.cmp(&other.uid))
    }
}

// Ord trait impl for Person
// This trait provides methods like max, min & clamp, which is used to restrict value to certain interval.
// Eq, ParitalOrd should be impl for Ord trait.
/// # Examples
/// ```
/// use tgl_rust::Person;
/// # fn main() {
/// let person = Person::default();
/// let person2 = Person::build(10, "Sid".to_string(), 10, 120);
/// assert_eq!(person.clone().max(person2.clone()), person2.clone());
/// assert_eq!(person.clone().min(person2.clone()), person.clone());
/// }
/// ```
impl Ord for Person {
    fn cmp(&self, other: &Self) -> Ordering {
        self.uid.cmp(&other.uid)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitivePerson<'a> {
    name: &'a str,
    age: i32,
    height: u16,
}

impl<'a> Default for PrimitivePerson<'a> {
    fn default() -> Self {
        Self {
            name: "Something",
            age: 0,
            height: 0,
        }
    }
}

// Copy trait impl for PrimitivePerson
/// Copy trait is a marker trait, which means it doesn't have any methods.
/// Also, Copy is a subtrait of Clone, which means Copy can be impl on a type only if it impls Clone.
/// # Examples
/// ```
/// use tgl_rust::PrimitivePerson;
/// # fn main() {
/// let person = PrimitivePerson::default();
/// let person2 = person;
/// assert_eq!(person, person2);
/// }
/// ```
impl<'a> Copy for PrimitivePerson<'a> {}
