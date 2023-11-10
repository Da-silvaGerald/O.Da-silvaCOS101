// Rust program to take input and age of employees

use std::io;

fn main() {
	let mut exp = String::new();
	let mut input = String::new();

	let a = 1_560_000;
	let b = 1_480_000;
	let c = 1_300_000;
	let d = 100_000;
	
	println!("Are you experienced?");
    io::stdin().read_line(&mut exp).expect("Not a valid string");
    let experience:bool = exp.trim().parse().expect("Not Valid");

    
    	println!("Enter your age: ");
        io::stdin().read_line(&mut input).expect("Not a valid string");
        let age:i32 = input.trim().parse().expect("Not a valid number"); 

         if experience == true && age >= 40
          {
         	println!("Your incentive is {}",a);
          }
          else if experience == true && age >= 30 && age < 40
          {
         	println!("Your incentive is {}",b);
          }
          else if experience == true && age < 30 && age >=18
          {
        	println!("Your incentive is {}",c);
          }
          else if experience == false && age <18
          {
          	println!("Your incentive is {}",d);
          }
          else {
          	println!("Sorry, We don't employ people less than the legal age.");
          }
    
}
