use std::fs::File;
use std::io::ErrorKind;

pub fn fun(){
    // unrecoverable, unhandleable error
    // panic!("This is an error.");

    // recoverable, handleable error

    // enum Result<T, E> { // like option enum, returns success or error
    //     Ok(T),          // success with some generic type T
    //     Err(E),         // error with some generic type E
    // }

    // this f has a return type of Result<File,Error> enum
    let f = File::open("hello.txt");

    match f {
        Ok(file) => file,
        Err(error) => match error.kind() { 
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other => panic!("Problem opening the file: {:?}", other),
        },
    };
          
}