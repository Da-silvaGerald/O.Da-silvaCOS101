use std::io::Write;


fn main() {
    let student_name = vec!["Oluchi Mordi","Adams Aliyu","Shania Bolade","Adekunke Gold","Blanca Edemoh"];
    let matric_number = vec!["ACC10211111","ECO1011001","CSC10328828","EE11020202","MEE10202001"];
    let department = vec!["Accounting","Economics","Computer Science","Electrical Enginnering","Mechanical Engineering"];

    let mut file = std::fs::File::create("SIMS.txt").expect("Create failed");
    file.write_all("\t\t\t PAU SIMS".as_bytes()).expect("Failed");
    file.write_all("\nStudent Name      Matric Number    Department \n".as_bytes()).expect("Failed");
    
    for i in 0..student_name.len(){
        file.write_all(student_name[i].as_bytes()).expect("Failed");

        file.write_all(matric_number[i].as_bytes()).expect("Failed");

        file.write_all(department[i].as_bytes()).expect("Failed");

    }

    println!("\nFile created successfully");
}
