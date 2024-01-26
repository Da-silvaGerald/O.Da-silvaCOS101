// Holiday Project
use std::io;
use std::io::Write;


fn main() {
    // Storing data into vectors
    let est_date = vec![1965,1974,1970,1960,1961,1923,1906,1946];
    let company = vec!["Cadbury Nigeria Plc","Champion breweries Plc","Dangote Sugar Refinery Plc",
                       "Flour Mills Nigeria Plc","Nestle Nigeria Plc","Unilever Nigeria Plc",
                       "Honeywell Nigeria Plc","Nigerian Breweries Plc"];
    let shares = vec![15000000,25000000,18000000,32000000,8000000,37000000,34000000,30000000];
    let liabilities = vec![5500000,8000000,10000000,4000000,1500000,11000000,9000000,12000000];

    // Finding Percentage leverages and pushing it into a vector
    let mut percentage_leverages : Vec<u32> = Vec::new();
    for i in 0..shares.len() {
    let  p_lev = ((shares[i] - liabilities[i]) / liabilities[i]) * 100; 
    let new_percentage_leverages = p_lev;
    percentage_leverages.push(new_percentage_leverages);    
    }

    // Interface model
    println!("Welcome to the Springate Model for Measuring Coorporate Failure.");

    // input username
    let mut input2 = String::new();
    println!("Enter Your username ( First four letters of company name)");
    io::stdin().read_line(&mut input2).expect("Failed to read Input");

    // input password
    let mut input3 = String::new();
    println!("Enter Your Password:
    	      Please take note of the following:
              - No uppercase letters
              - No letters from [$#@]
              - For Cadbury Nigeria Plc: cdr0165
              - For Champion breweries Plc: cmp0274
              - For Dangote Sugar Refinery Plc: dnt0370
              - For Flour Mills Nigeria Plc: for0460
              - For Nestle Nigeria Plc: nsl0561
              - For Unilever Nigeria Plc: uie0623
              - For Honeywell Nigeria Plc: hny0706
              - For Nigerian Breweries Plc: ngr0846");
    io::stdin().read_line(&mut input3).expect("Failed to read Input");

    // validating username and password
    if input2.trim() == "cadb" && input3.trim() == "cdr0165"{
    for _0 in 0..1 {
         println!("\nDate of Establishment:{}
                     Company name:{}
                     Shares:{}
                     Liabilities:{}
                     Percentage liabilitiies: {}\t",est_date[0],company[0],shares[0],liabilities[0],percentage_leverages[0]);
    }
    // saving the companies with the shares greater than 20,000,000 to a file
    for _0 in 0..1{
    if shares[0] > 20_000_000
    {
        let mut file = std::fs::File::create("Above_20000000.txt").expect("Create failed");
        file.write_all(shares[0].to_string().as_bytes()).expect("Failed");
    }
    }
    // finding 5% of of companies shares less than 10,000,000
    for _0 in 0..1{
    if shares[0] < 10_000_000
    {
        let per = (percentage_leverages [0] * 5) / 100;
        println!("5% of the multiples of percentage leverages = {}",per);
    }
    }
    }
    else if input2.trim() == "cham" && input3.trim() == "cmp0274"{
       for _1 in 1..2{
         println!("\nDate of Establishment:{}
                     Company name:{}
                     Shares:{}
                     Liabilities:{}
                     Percentage liabilitiies: {}\t",est_date[1],company[1],shares[1],liabilities[1],percentage_leverages[1]);
    }
    // saving the companies with the shares greater than 20,000,000 to a file
    for _1 in 1..2{
    if shares[1] > 20_000_000
    {
        let mut file = std::fs::File::create("Above_20000000.txt").expect("Create failed");
        file.write_all(shares[1].to_string().as_bytes()).expect("Failed");
    }
    }
    // finding 5% of of companies shares less than 10,000,000
    for _1 in 1..2{
    if shares[1] < 10_000_000
    {
        let per = (percentage_leverages [1] * 5) / 100;
        println!("5% of the multiples of percentage leverages = {}",per);
    }
    }
    }
    else if input2.trim() == "dang" && input3.trim() == "dnt0370"{
        for _2 in 2..3{
         println!("\nDate of Establishment:{}
                     Company name:{}
                     Shares:{}
                     Liabilities:{}
                     Percentage liabilitiies: {}\t",est_date[2],company[2],shares[2],liabilities[2],percentage_leverages[2]);
    }
    // saving the companies with the shares greater than 20,000,000 to a file
    for _2 in 2..3{
    if shares[2] > 20_000_000
    {
        let mut file = std::fs::File::create("Above_20000000.txt").expect("Create failed");
        file.write_all(shares[2].to_string().as_bytes()).expect("Failed");
    }
    }
    // finding 5% of of companies shares less than 10,000,000
    for _2 in 2..3{
    if shares[2] < 10_000_000
    {
        let per = (percentage_leverages [2] * 5) / 100;
        println!("5% of the multiples of percentage leverages = {}",per);
    }
    }
    }
    else if input2.trim() == "flou" && input3.trim() == "for0460"{
        for _3 in 3..4{
         println!("\nDate of Establishment:{}
                     Company name:{}
                     Shares:{}
                     Liabilities:{}
                     Percentage liabilitiies: {}\t",est_date[3],company[3],shares[3],liabilities[3],percentage_leverages[3]);
    }
    // saving the companies with the shares greater than 20,000,000 to a file
    for _3 in 3..4{
    if shares[3] > 20_000_000
    {
        let mut file = std::fs::File::create("Above_20000000.txt").expect("Create failed");
        file.write_all(shares[3].to_string().as_bytes()).expect("Failed");
    }
    }
    // finding 5% of of companies shares less than 10,000,000
    for _3 in 3..4{
    if shares[3] < 10_000_000
    {
        let per = (percentage_leverages [3] * 5) / 100;
        println!("5% of the multiples of percentage leverages = {}",per);
    }
    }
    }
    else if input2.trim() == "nest" && input3.trim() == "nsl0561"{
        for _4 in 4..5{
         println!("\nDate of Establishment:{}
                     Company name:{}
                     Shares:{}
                     Liabilities:{}
                     Percentage liabilitiies: {}\t",est_date[4],company[4],shares[4],liabilities[4],percentage_leverages[4]);
    }
    // saving the companies with the shares greater than 20,000,000 to a file
    for _4 in 4..5{
    if shares[4] > 20_000_000
    {
        let mut file = std::fs::File::create("Above_20000000.txt").expect("Create failed");
        file.write_all(shares[4].to_string().as_bytes()).expect("Failed");
    }
    }
    // finding 5% of of companies shares less than 10,000,000
    for _4 in 4..5{
    if shares[4] < 10_000_000
    {
        let per = (percentage_leverages [4] * 5) / 100;
        println!("5% of the multiples of percentage leverages = {}",per);
    }
    }
    }
    else  if input2.trim() == "unil" && input3.trim() == "uie0623"{
        for _5 in 5..6{
        println!("\nDate of Establishment:{}
                     Company name:{}
                     Shares:{}
                     Liabilities:{}
                     Percentage liabilitiies: {}\t",est_date[5],company[5],shares[5],liabilities[5],percentage_leverages[5]);
    }
    // saving the companies with the shares greater than 20,000,000 to a file
    for _5 in 5..6{
    if shares[5] > 20_000_000
    {
        let mut file = std::fs::File::create("Above_20000000.txt").expect("Create failed");
        file.write_all(shares[5].to_string().as_bytes()).expect("Failed");
    }
    }
    // finding 5% of of companies shares less than 10,000,000
    for _5 in 5..6{
    if shares[5] < 10_000_000
    {
        let per = (percentage_leverages [5] * 5) / 100;
        println!("5% of the multiples of percentage leverages = {}",per);
    }
    }
    }
    else if input2.trim() == "hone" && input3.trim() == "hny0706"{
        for _6 in 6..7{
         println!("\nDate of Establishment:{}
                     Company name:{}
                     Shares:{}
                     Liabilities:{}
                     Percentage liabilitiies: {}\t",est_date[6],company[6],shares[6],liabilities[6],percentage_leverages[6]);
    }
    // saving the companies with the shares greater than 20,000,000 to a file
    for _6 in 6..7{
    if shares[6] > 20_000_000
    {
        let mut file = std::fs::File::create("Above_20000000.txt").expect("Create failed");
        file.write_all(shares[6].to_string().as_bytes()).expect("Failed");
    }
    }
    // finding 5% of of companies shares less than 10,000,000
    for _6 in 6..7{
    if shares[6] < 10_000_000
    {
        let per = (percentage_leverages [6] * 5) / 100;
        println!("5% of the multiples of percentage leverages = {}",per);
    }
    }
    }
    else if input2.trim() == "nige" && input3.trim() == "ngr0846"{
        for _7 in 7..8{
        println!("\nDate of Establishment:{}
                     Company name:{}
                     Shares:{}
                     Liabilities:{}
                     Percentage liabilitiies: {}\t",est_date[7],company[7],shares[7],liabilities[7],percentage_leverages[7]);
    }
    // saving the companies with the shares greater than 20,000,000 to a file
    for _7 in 7..8{
    if shares[7] > 20_000_000
    {
        let mut file = std::fs::File::create("Above_20000000.txt").expect("Create failed");
        file.write_all(shares[7].to_string().as_bytes()).expect("Failed");
    }
    }
    // finding 5% of of companies shares less than 10,000,000
    for _7 in 7..8{
    if shares[7] < 10_000_000
    {
        let per = (percentage_leverages [7] * 5) / 100;
        println!("5% of the multiples of percentage leverages = {}",per);
    }
    }
    }
    else{
        println!("You cannot access this data.
                  Your Username or Password is not correct.");
    }


    let mut file = std::fs::File::create("Companies.txt").expect("Create failed");
    file.write_all("\t\t\t\t\t Company's Database".as_bytes()).expect("Failed");
    file.write_all("\nEstablishment Date  Company Name                 Shares     Liabilities   Percentage leverages ".as_bytes()).expect("Failed");

     for i in 0..est_date.len(){
    
        file.write_all(est_date[i].to_string().as_bytes()).expect("Failed");
    
        file.write_all(company[i].as_bytes()).expect("Failed");
    
        file.write_all(shares[i].to_string().as_bytes()).expect("Failed");
    
        file.write_all(liabilities[i].to_string().as_bytes()).expect("Failed");

        file.write_all(percentage_leverages[i].to_string().as_bytes()).expect("Failed");
     }  
}


