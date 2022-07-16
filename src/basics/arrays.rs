use std::mem;

pub fn fun(){
    // define types and length
    let mut numbers: [i32;5] = [1,2,3,4,5];

    //print the array
    println!("{:?}", numbers);
    
    //only possible if it is mutatable
    numbers[2] = 20;

    //get a single value
    println!("{}",numbers[0]);

    //length
    println!("{}",numbers.len());

    //arrays are stack allocated, memory of arrays
    println!("This array occupies {} bytes.", mem::size_of_val(&numbers));

    //get slice of an array
    let slice: &[i32] = &numbers[0..4];
    
    println!("Slice : {:?}",slice);
    println!("Numbers : {:?}",numbers);

}