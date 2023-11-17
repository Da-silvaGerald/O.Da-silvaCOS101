// Rust Program for multiplication of 1 to n
// Nested loop.

use std::io;

fn main() {
	let mut input1 = String::new();


	println!("What number would you like to stop at? ");
	io::stdin().read_line(&mut input1).expect("Not a valid string");
    let number:i64  = input1.trim().parse().expect("Not valid number");

    let mut x = 0;
    println!("The {} times table ",number);
	loop{
		x+=1;
		let product = x * number;
		println!("x= {} x {}= {}",x,number,product);

		if x==number{
			break;
		}
        }
        
	
}
