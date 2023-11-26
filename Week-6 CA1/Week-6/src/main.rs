// Rust Program for Otuene Family health centre

use std::io;

fn main() {
    
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    let mut input4 = String::new();
    let mut input5 = String::new();
    let mut input6 = String::new();
    let mut input7 = String::new();
    let mut input8 = String::new();
    let mut input9 = String::new();

loop{
    println!("Welcome to Otuene Family health Centre.
    	      These are the diseases treated here: Alzheimer              - N1,200,000
    	                                           Arrhythima             - N550,000
    	                                           Chronic kidney disease - N1,500,000
    	                                           Diabetes               - N800,000
    	                                           Arthritis              - N450,000");
                                                   
    println!("Enter your name: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");

    println!("Enter your date of birth (Only the year): ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let dob:i32 = input2.trim().parse().expect("Not a valid number");

    println!("Enter your E-mail address: ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");

    println!("Enter your phone number: ");
    io::stdin().read_line(&mut input4).expect("Not a valid string");
    let number:i64 = input4.trim().parse().expect("Not a valid number");

    println!("Enter the number of siblings you have: ");
    io::stdin().read_line(&mut input5).expect("Not a valid string");
    let siblings:i32 = input5.trim().parse().expect("Not a valid number");

    println!("Enter the number of children you have: ");
    io::stdin().read_line(&mut input6).expect("Not a valid string");
    let children:i32 = input6.trim().parse().expect("Not a valid number");

    println!("Enter your medical Diagnosis: ");
    io::stdin().read_line(&mut input7).expect("Not a valid string");

    println!("Enter your Village of residence ");
    io::stdin().read_line(&mut input8).expect("Not a valid string");

    println!("Enter your patient number: ");
    io::stdin().read_line(&mut input9).expect("Not a valid string");
    let patient_no:i32 = input9.trim().parse().expect("Not a valid number");
 
    println!("Your name is: {}
    	       Your D.O.B: {}
    	       Your E-mail address is: {}
    	       Your Phone number is: {}
    	       The number of sibling you have is: {}
    	       The number of childen you have is: {}
    	       Your diagnosis is: {}
    	       Your Village of resisdence is: {} 
    	       Your patient number is: {}",input1,dob,input3,number,siblings,children,input7,input8,input9);

    let age = 2023 - dob;
    println!("Your age is: {}",age);

    let mut discounted_price = 0
    let amount_b = 550_000;
    let amount_c = 1__500_000;
    let amount_d = 800_000;
    let amount_e = 450_00;
    let amount_a = 1_200_000;
    if input7 == "Alzheimer" && age > 50 && children > 4 && input8 == "Akpabom"
    {
    	println!("Your amount is: {}",amount_a);
    }
    else if input7 == "Arrhythima" && age == 30 && siblings > 4 && input8 == "Ngbauji"
    {
    	println!("Your amount is: {}",amount_b);
    }
    else if input7 == "Chronic Kidney Disease" && age > 40 && siblings > 3 && children > 3 && input8 == "Atabrikang"
    {
      	println!("Your amount is: {}",amount_c);
    }
    else if input7 == "Diabetes" && age > 28 && age < 45 && children >= 2 && children <=4 && input8 == "Okorobilom"
    {    	
    	println!("Your amount is: {}",amount_d);
    }    
    else if input7 == "Arthritis" && age > 58 && siblings > 5 && children > 5 && input8 == "Emeremen"
    {
       	println!("Your amount is: {}",amount_e);
    }
    else if input7 == "Alzheimher"	
    {
    	println!("Your price is 1,200,000");
    }
    else if input7 == "Arrhythima"
    {
    	println!("Your price is 550,000");
    }
    else if input7 == "Chronic Kidney Disease"
    {
    	println!("Your price is 1,500,000");
    }
    else if input7 == "Diabetes"
    {
    	println!("Your price is 800,000");
    }
    else if input7 == "Arthritis"
    {
    	println!("Your price is 450,000");
    }
    else{
        println!("We don't treat that here");
    }
    if patient_no > 100
    {
    	break;
    }
    } 
}