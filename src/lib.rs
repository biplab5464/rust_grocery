pub mod shop{

    pub struct Bill{
        result : (i32,i32,i32),
        fruit : String,
        num : i32,
        discount : i32,
    }

    impl Bill{

        pub fn new_bill(fruit : String , num : i32 , discount :i32) -> Bill {
            Bill{
                fruit,
                num,
                discount,
                result:(0,0,0),
            }
        }


        fn get_price(&mut self){
            match self.fruit.trim(){
                "banana" =>{   
                    if self.num > 100 { self.result.0 = self.num * 3; }
                    else if self.num > 30 { self.result.0 = self.num * 4; }
                    else  { self.result.0 = self.num * 5 }
                },
                "apple" =>{
                    if self.num > 100 { self.result.0 = self.num * 16;}
                    else if self.num > 30 {self.result.0 = self.num * 18;}
                    else {self.result.0 = self.num * 20;}
                },
                "mango" => {
                    if self.num > 100 { self.result.0 = self.num * 9;}
                    else if self.num > 30 { self.result.0 = self.num * 10;}
                    else {self.result.0 = self.num * 12; }
                },
                _ => {
                    self.result.0 = -1;
                    println!("Fruit does not exist");
                },
            }
            
        }
        
        fn price_with_gst(&mut self){
            self.result.1 = (self.result.0 * 12)/100 + self.result.0
        }

        fn discount_price(&mut self){
            if self.discount > 0 && self.discount <  101{
                self.result.2 = self.result.1 - (self.result.1 * self.discount) / 100
            }else if self.discount == 0{
                self.result.2 = self.result.1
            }else{
                panic!("discount can't be less then 0 or bigger then 100")
            }
        }

        pub fn print_bill(&mut self){
            self.get_price();
            self.price_with_gst();
			println!("--------------------------------------------------------------");
            println!("bill {} \nbill after gst {}",self.result.0,self.result.1);
			println!("--------------------------------------------------------------");
        }

		pub fn print_bill_dis(&mut self){
			self.get_price();
			self.price_with_gst();
			self.discount_price();
			println!("--------------------------------------------------------------");
			println!("Bill  \t\t\t: {} \nBill after gst \t\t: {} \nbill after {}% discount \t: {}",self.result.0,self.result.1,self.discount, self.result.2);
			println!("--------------------------------------------------------------");
		}

        pub fn get_bill(&mut self) -> (i32,i32){
            self.get_price();
			self.price_with_gst();

            (self.result.0,self.result.1)
        }

        pub fn get_bill_dis(&mut self) -> (i32,i32,i32){
            self.get_price();
            self.price_with_gst();
            self.discount_price();

            self.result

        }
    }
}