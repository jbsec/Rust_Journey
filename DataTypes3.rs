// Okay so I'm straight up gonna finish all the 
// data types from Rust into this page, this will
// be a massive file so just a heads up to the 
// 2 people that might read my code.

fn main() {
    /*
    let age:u8 = 255;
    // 0 to 255 is only allowed for u8
    let weight:u8 = 256; //overflow val 0
    let height:u8 = 257; //overflow val 1
    let score:u8 = 258; //overflow val 2
    
    println!("age is {}", age);
    println!("weight is {}", weight);
    println!("height is {}", height);
    println!("score is {}", score);
    */
    
    // the above code runs but with errors 
    // for literals out of range.
    // I'm commenting them out with block
    // comments. 
    //floaty()  // acc doesn't matter if ; is above.
    
    //ATC()
    num_sep();
    // if listing multiple functions seperate
    // function calls with ;
    boo_stuff();
    // ; not required for single use but gonna
    // use it now as good measure. 
    spec_char();
    
    }

/*    
}

fn floaty() {
    let result = 10.00; //f64 default? 
    let interest:f32 = 8.35;
    let cost:f64 = 15000.600; //double precision
    
    println!("result value is: {}",result);
    println!("interest is: {}",interest);
    println!("cost is {}",cost);
}
*/

/*
fn ATC() {
    let interest:f32 = 8; //integer assinged to float variable
    println!("interest is {}", interest);
}
// ... supposed to give an error wow.
*/

fn num_sep() {
// change function names from things like
// NumSep to snake case such as num_sep
    let float_with_separator = 11_000.555_001;
    println!("float value {}",float_with_separator);
    
    let int_with_separator = 50_000;
    println!("in value {}",int_with_separator);
    
    }

fn boo_stuff() {
    let isfun:bool = true;
    println!("Is Rust Programming Fun ? {}",isfun);
}    

fn spec_char() {
    let special_character = '@'; //default?
    let alphabet:char = 'A';
    let emoji:char = '😁';
    
    println!("special character is {}",special_character);
    println!("aplhabet is {}", alphabet);
    println!("emoji is {}",emoji);
}    
    
    
    
    
    
    
    
    
    
    
    
    
    