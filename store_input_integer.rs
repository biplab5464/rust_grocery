use std::io;

fn main() {

    let mut fruit = String::new();
    let mut num_str = String::new();
    let mut value = String::new();
    let mut result : i32;
    // let mut total : i32 = 0;

   // loop{
        println!("Enter the fruit name");
        io::stdin().read_line(&mut fruit).unwrap();

        //let fruit = "banana";

        println!("Enter the number of fruits");
        io::stdin().read_line(&mut num_str).unwrap();
        let num : i32 = num_str.trim().parse().unwrap();

        match fruit.trim(){
            "banana" =>{   
                if num > 100 { result = num * 3; }
                else if num > 30 { result = num * 4; }
                else  { result = num * 5 }
            },
            "apple" =>{
                if num > 100 { result = num * 16;}
                else if num > 30 {result = num * 18;}
                else {result = num * 20;}
            },
            "mango" => {
                if num > 100 { result = num * 9;}
                else if num > 30 { result = num * 10;}
                else {result = num * 12; }
            },
            _ => {
                result = -1;
                println!("Fruit does not exist");
            },
        }

        // println!("do you want to Continue? (yes|no)");
        // io::stdin().read_line(&mut value).unwrap();
        // if value.trim() == "no" {  break; }

        // total = total + result;
    //}
    println!("Cost of the is : {}", result);
    
}
