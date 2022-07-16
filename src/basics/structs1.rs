// Tradtional struct
struct User{
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}


pub fn fun(){
    // 1.
    let mut user1 = User{
        username: String::from("someone"),
        email: String::from("someone@mail.com"),
        sign_in_count: 1,
        active: true,
    };

    user1.email = String::from("dead@mail.com");
    user1.sign_in_count += 1;
    println!("{:?}", user1.sign_in_count);

    let user2 = create_user(String::from("ramesh"), String::from("ramesh@gmail.com"));
    println!("{}", user2.email);

} 

fn create_user(username: String, email: String) -> User{
    User{
        username,
        email,
        sign_in_count: 1,
        active: true,
    }
}