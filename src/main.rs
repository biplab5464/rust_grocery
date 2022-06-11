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
    let mut total : i32 = 0;

    let arr = [("apple", 35),("banana",45),("mango",34)];
    for ele in arr{
        let mut bill = shop::Bill::new_bill(ele.0.to_string(),ele.1,0);
        //bill.print_bill();
        let mut result = bill.get_bill();
        total = total + result.1;
    }

    println!("Total : {}",total);

    //let mut bill = shop::Bill::new_bill(fruit,num,3);

    //bill.print_bill();
    
}
