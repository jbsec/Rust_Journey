/// strings in rust im lazy with uploads lately. 
// this whole thing is gonna be a cluster of junk!

fn main() {
    println!("Hello, world!");
    let company:&str="company a";
    let location:&str = "location a";
    println!("company is : {} location : {}",company,location);
        let company2:&'static str = "company b";
        let location2:&'static str = "location b";
        println!("company is : {} location is {}",company2,location2);
        // im glad that mess for  literal string works.
    let empty_string = String::new();
    println!("length is {}",empty_string.len());
    // indenting means n o t h i n g here hahaha!
    let content_string = String::from("data");
    println!("length is {}",content_string.len());
    //doinh illustrations now
    let mut z = String::new();
    z.push_str("hello dudes.");
    println!("{}",z);
    //honestly why would you do it this way? :s
    //tostring now
    let name1 = "hello,
    named person.".to_string();
    println!("{}",name1);
    //replacing, cant believe these are all strings
    let namey1 = "hello hello hello,
    hello hello!".to_string(); // str object
    let namey2 = namey1.replace("hello","howdy"); //find repalce
    println!("{}",namey2);
    // find replace is actually pretty useful honestly. 
    // as string, exytacts string slice contain th entire string.
        let example_string_one = String::from("example_string");
        print_literal(example_string_one.as_str());
    let mut co1 = "tutorial".to_string();
    co1.push('s');
    println!("{}",co1);
        //length
        let fullname2 = "tuts point";
        println!("length is {}",fullname2.len());
        let fullname2 = "tut point... \r\n";
        println!("Before trim ");
        println!("length is {}", fullname2.len());
        println!();
        println!("After trim");
        println!("length is {}",fullname2.trim().len());
    // new stuff.
    // split whitespace strings
    let msg = "hello there man.".to_string();
    let mut i = 1;
    for token in msg.split_whitespace(){
        println!("token {} {}",i,token);
        i+=1; 
    }
        //split methods?
        let fname = "james, dames, wames";
        for token in fname.split(","){
            println!("token is {}",token);
        }
            //store in vector
            println!("\n");
            let tokens:Vec<&str>= fname.split(",").collect();
            println!("first name is {}", tokens[0]);
            println!("last name is {}", tokens[1]);
            println!("compnay is {}",tokens[2]);
            // suprised that ran actually....
                let n1 = "seperate".to_string();
                    for n in n1.chars(){
                        println!("{}",n);
                    }
                    //concatenation
                    let n2 = "example".to_string();
                    let n3 = "example2".to_string();
                    let n4 = n2 + &n3; // pass ref?
                    println!("{}",n3);
                        //type casting
                        let num = 2021;
                        let num_as_str = num.to_string();
                        //con no to str
                        println!("{}",num_as_str);
                        println!("{}",num_as_str=="2021");
                    //format!  macro?
                    let a1 = "examplez".to_string();
                    let a2 = "examplezz".to_string();
                    let a3 = format!("{} {}",a1,a2);
                        println!("{}",a3);
}
        //glad function doesnt return an error :)
        fn print_literal(data:&str ){
            println!("displaying string literal {}",data);
        }
//           
    


