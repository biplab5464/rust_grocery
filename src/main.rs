use std::io;
use store::shop; 
use std::collections::HashMap;

fn main() {

    let mut fruit = String::new();
    let mut num_str = String::new();
    let mut total : i32 = 0;
    let mut arr = HashMap::new(); 

    for i in 0..3{
        println!("Enter the fruit name :");
        io::stdin().read_line(&mut fruit).unwrap();

        println!("Enter the number of fruits :");
        io::stdin().read_line(&mut num_str).unwrap();
        let mut num : i32 = num_str.trim().parse();
        
        arr.insert(fruit.clone(),num);

        // println!("Do you want to enter more fruits(y/n)");

    }
    
    arr.entry(String::from("apple")).or_insert(0);
    arr.entry(String::from("banana")).or_insert(0);
    arr.entry(String::from("mango")).or_insert(0);
    

    for (key,value) in &arr{
        let mut bill = shop::Bill::new_bill(key.to_string().clone(),*value,0);
        //bill.print_bill();
        let result = bill.get_bill();
        total = total + result.0;
    }

    total = total + (total * 12) /100 ;
    println!("Total : {}",total);

    //let mut bill = shop::Bill::new_bill(fruit,num,3);

    //bill.print_bill();
    
}
