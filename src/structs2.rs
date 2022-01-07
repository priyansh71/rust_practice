// Person struct 
struct Person{
    first_name : String,
    last_name : String,
}

impl Person {
    // constructor
    pub fn new(first : &str, last : &str) -> Person{
        Person{
            first_name : first.to_string(),
            last_name : last.to_string(),
        }
    }

    // get full name
    fn full_name(&self) -> String{
        format!("{} {}", self.first_name, self.last_name)
    }

    // set last name
    fn got_married_to(&mut self, spouse: &Person){
        self.last_name = spouse.last_name.clone();
    }

    //name to tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn fun(){

    let paul = Person::new("Paul", "Jimenez");
    let mut mary = Person::new("Mary", "Jane");
    println!("Husband's name : {}.", paul.full_name());
    println!("Woman's native name : {}.", mary.full_name());
    // POV: got married
    mary.got_married_to(&paul);
    println!("New name : {}.", mary.full_name());
    println!("{:?}", paul.to_tuple());  

}