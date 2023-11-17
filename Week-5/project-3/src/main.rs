// Rust program to take an order

use std::io;

 fn main(){
     let input = io::stdin();
     let mut total:i32 = 0;

     println!("\t\tMenu\t\tPrice");
     println!("P = Poundo Yam/Edikainko Soup\t  - N3'200");
     println!("F = Fried Rice and Chicken\t  - N3,000");
     println!("A = Amala and Ewedu soup\t  - N2,500");
     println!("E = Eba & Egusi \t          - N2,000");
     println!("W = White rice and stew \t  - N2,500");

     println!("Enter the letter of the food you would like to order (q to quit)");
     loop {
     	let mut food = String::new();
     	input.read_line(&mut food).expect("Not a valid string");
     	let food = food.trim();

     	if food == "P" {
     		total += 3200;
     	} else if food == "F" {
     		total += 3000;
     	} else if food == "A" {
     		total += 2500;
     	} else if food == "E" {
     		total += 2000; 
     	} else if food == "W" {
     		total += 2500; 
     	} else if food == "q" {
     		break;
     	} else {
     		println!("Sorry We down serve that here!");
     		continue;
     	}
     }
    println!("Your old balance is {}",total);
    if total > 10_000 {
    	let new_balance = total - ((5 / 100) * total);  	
    	println!("Your new balance is {}",new_balance);
    } else {
        println!("Total: N{}",total);	
    }
}
         