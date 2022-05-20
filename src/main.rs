//use std::io;
use store::shop;

fn main() {

    // let mut fruit = String::new();
    // let mut num_str = String::new();

    // println!("Enter the fruit name :");
    // io::stdin().read_line(&mut fruit).unwrap();

    // println!("Enter the number of fruits :");
    // io::stdin().read_line(&mut num_str).unwrap();
    // let num : i32 = num_str.trim().parse().unwrap();

    let arr = [("apple", 35),("banana",45),("mango",34)];
    for ele in arr{
        println!("{} - {}",ele.0,ele.1);
        let bill = shop::Bill::new_bill(ele.0.to_string(),35)

    }

    //let mut bill = shop::Bill::new_bill(fruit,num,3);

    //bill.print_bill();
    
}
