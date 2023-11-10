// rust program to determine the roots of a quadratic equation

  use std::io;

fn main() {
	let mut input1 = String::new();
	let mut input2 = String::new();
	let mut input3 = String::new();

    println!("Enter A: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:f32 = input1.trim().parse().expect("Not a valid number");

    println!("Enter B: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:f32 = input2.trim().parse().expect("Not a valid number");

    println!("Enter C: ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let c:f32 = input3.trim().parse().expect("Not a valid number");

    let d = (b * b) - (4.0 * a * c);

    println!("The discriminant is: {}",d);

    if d < 0.0
    {
    	println!("There are no real roots");
    }
    else if d == 0.0
    {
    	println!("There is exactly one real root");
    }
    else if d > 0.0	
    {
    	println!("There are two distinct roots");
    }
 
}
