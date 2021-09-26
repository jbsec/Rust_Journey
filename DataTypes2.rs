fn main() {
    let company_string = "JamesBa"; // string type
    let rating_float = 4.5; // float type
    let is_growing_boolean = true; // bool type
    let icon_char = '♥'; // unicode char type
    
    println!("Company Name Is:{}", company_string);
    println!("Company rating /5 is {}", rating_float);
    println!("Company is growing: {}", is_growing_boolean);
    println!("Company Icon is:{}", icon_char);
    second();
    archint();
}

fn second() {
    println!("adding second.");
    println!("can add secondary function to a primary main function just dandy, as you can see this. ");
}

fn archint() {
// try not to reuse any vars LOL.
    let result = 10; // i32 by default
    let age:u32 = 20;
    let sum:i32 = 5-15;
    let mark:isize = 10;
    let count:usize = 30;
    println!("result value is {}", result);
    println!("sum is {} and age is {}",sum,age);
    println!("mark is {} and count is {}",mark,count);
    // had to make sure to call cause this
    // function was "technically dead"
}

