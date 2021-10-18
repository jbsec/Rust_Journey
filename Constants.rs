fn main() {
    println!("Hello, world!");
    //constants
    const USER_LIMIT:i32 = 100; //declare int const
    const PI:f32 = 3.14; //declare float const
    println!("user limit is {}",USER_LIMIT); // dispaly value of constant
    println!("pi value is {}",PI); //display value of the constant
    // shadowing vars and consts
    let salary = 100.00;
    let salary = 1.50;
    // read first salary
    println!("The value of salary is: {}",salary);
    // throws warning for unused var
    let uname = "James";
    let uname = uname.len();
    println!("name changed to integer : {}",uname);
    // below will show an error
    /*
    const NAME:&str = "Jamesy";
    const NAME:usize = NAME.len();
    // error name already defined
    println!("name changed to : {}",NAME);
    */
}