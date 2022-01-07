pub fn fun(){
    // Primitive : Arrays
    let arr1 = [1,2,3];
    let arr2 = arr1;

    println!("{:?}", (arr1,arr2));

    // Non-primitives : Vectors
    let arr3 :Vec<i32> = vec![1,2,3,4];
    let arr4 = &arr3; // create a reference to point

    println!("{:?}", (&arr3,arr4));

}