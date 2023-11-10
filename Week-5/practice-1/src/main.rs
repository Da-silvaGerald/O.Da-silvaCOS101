// Rust Program To Calculate the area of a
// Triangle For A Given Base And Height

use std::io;

fn main() 
{
	let mut input1 = String::new();
	let mut input2 = String::new();


    println!("Enter Base: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let base:f32 = input1.trim().parse().expect("Not a Valid number");


   println!("Enter the height:	");
   io::stdin().read_line(&mut input2).expect("Not a vaklid string");
   let height:f32 = input1.trim().parse().expect("Not a valid number");

   if base > 0.0 {
   	    let area:f32 = (base * height) / 2.0;
   	    println!("Area of a triangle: {}",area);
   }
}
