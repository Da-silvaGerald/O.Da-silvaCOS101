fn main() {

	let b:(i32,bool,f64) = (30,true,4.9);
    println!("Hello, world!");
    print(b);
}

fn print(x:(i32,bool,f64)) {

	println!("Inside print method");
	// Assigns a tuple to distinct variables
	let (age,is_male,cgpa) = x;
	println!("Age is {}, Is he male? {}, cgpa is {}",age,is_male,cgpa);
}