fn main() {

	let n1 = "Electrical".to_string();
	let n2 = "Electronic".to_string();
	let n3 = "Engineering".to_string();
	let n4 = n1 + &n2 +&n3; //n2 & n3 reference is passed

    // About Electrical/Electronic
    println!("\nThe {} is informed by the aspiration to 
    	train electrical/electronic engineering professionals
    	in the areas of design, building and maintenace of
    	electrical conr=trol systems",n4);

    let w1 = "Software".to_string();
    let w2 = "Engineering".to_string();
    let w3= w1 + &w2;  // w2 reference is passed
    println!();
    println!("{} is aimed at developing competent, creative, 
    	innovative, enterpreneurial and ethically-winded persons,
    	capable of creating value in the diverse fields of 
    	Software Engineering. ",w3);
}