#[derive(Debug, PartialEq, Eq)]
pub struct Person {
    pub name: String,
    pub age: u16,
    pub height: u16,
}

// Default trait impl for Person
/// # Examples
/// ```
/// use tgl_rust::Person;
/// # fn main() {
/// let person = Person::default();
/// assert_eq!(person, Person {name: String::new(), age: 0, height: 0});
/// }
/// ```
impl Default for Person {
    fn default() -> Self {
        Self {
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
/// let mut person3 = Person { name: String::from("test"), age: 10, height: 150};
/// person3.clone_from(&person2);
/// assert_eq!(person2, person3);
/// }
/// ```
impl Clone for Person {
    fn clone(&self) -> Self {
        Self {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_default() {}
}
