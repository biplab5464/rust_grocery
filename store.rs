fn main() {
    let fruit = "banan";
    let num = 20;
    let result : i32;    
    match fruit{
        "banana" => 
            if num > 100{ result = num * 3; }
            else if num > 30 { result = num * 4;} 
            else    { result = num * 5; },
        "apple" => 
            if num > 100 { result = num * 20}
            else if num > 30 { result= num * 18}
            else { result = num * 16},
        "orange" => 
            if num > 100 { result = num * 9}
            else if num > 30 { result= num * 8}
            else { result = num * 7},
        
        
        _ => {
            result = -1;
            println!("Fruits is unavailable");
        },
    }
    
    println!("bill = {}",result);
}
