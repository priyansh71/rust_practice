// Borrow checker checks to see if the lifetime of all references are valid or not

pub fn fun(){
    let r : &i32; // lifetime begins
    {
        let x = 5; // x lifetime starts
        r = &x;         // x does not live longer than this scope, lifetime ends
    }
    // println!("r: {}", r); // r is a dangling reference, lifetime ends here

    let x = 5; 
    let y = &x;
    // println!("y: {}", y); // y is a reference to x, lifetime ends here

    let string1 = String::from("abcd"); 
    let string2: String = String::from("xyz"); 
    // let result = longest_invalid(string1.as_str(), string2.as_str()); 
    // let result = longest_valid(string1.as_str(), string2.as_str());
    // println!("The longest string is {}", result);

    {
        let string3 = String::from("efg");
        let result = longest_valid(string1.as_str(), string3.as_str());
        // println!("The longest string is {}", result); 
        // PASSES:  this works because lifetime of result would be the the same as that of string3
        // and string3's lifetime is valid in this scope
    }

    let mut result : &str;
    {
        let string4 = String::from("efg");
        result = longest_valid(string1.as_str(), &string4.as_str());
    }
    // ERROR: this does not work because lifetime of result would be the the same as that of string4
    // and string4's lifetime is not valid in this scope
    // hence string4 has not lived long enough to be borrowed here
    // println!("The longest string is {}", result); 

    {
        let string5 = String::from("efg");
        result = same(string1.as_str(), &string5.as_str());
    }
    println!("The longest string is {}", result); 
    // PASSES: this works because lifetime of result would be the the same as that of string1
    // string1's lifetime is valid in this scope

    let new_result = return_string(string1.as_str(), &string2.as_str());
    println!("Result is {}", new_result);


}

// ERROR: return type contains a borrowed reference, but we do not know if it is valid as it can be either of x or y
// x and y are params in general and hence this function does not know anything about their lifetimes
fn longest_invalid(x: &str, y: &str) -> &str { 
    // if x.len() > y.len() {
    //     x
    // } else {
    //     y
    // }

}

// PASSES: Hence, we need to use a lifetime annotation to specify the lifetime of the return type
// we will annotate by relating the lifetime of the return type with the lifetime of the borrowed references
// this syntax tells the compiler that the lifetime of the return type is the minimum of the lifetimes of the borrowed references

fn longest_valid<'a>(x: &'a str, y: &'a str) -> &'a str { 
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// PASSES 
fn same<'a>(x: &'a str, y: & str) -> &'a str { 
   x
}

// If we want to return a reference from a function then it cannot be something that has been defined 
// inside the scope of the function because that value would be dropped as soon as the function ends.
// Hence we always have to return a reference that is related to the parameters of the function 
// along with lifetime annotations to specify the lifetime of the return type wherever it's called.
// Otherwise we should return an owner which can transfrt ownership of the value itself.

fn return_str<'a>(x : &'a str, y: &'a str) -> &'a str  { 
    let t: String = String::from("hello");
    // t.as_str()
    x
    // ERROR : cannot return reference to a local variable because its owner is defined inside the scope
 }
 
fn return_string<'a>(x : &'a str, y: &'a str) -> String { 
    let t: String = String::from("hello");
    t
 }
 