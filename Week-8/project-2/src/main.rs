use std::io;

fn main() {
    let mut developers: Vec<(String, u32, String)> = Vec::new();

    // Gather developer information from the user
    loop {
        println!("Enter developer name (or 'done' to finish):");
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Failed to read input");
        let name = name.trim();

        if name.to_lowercase() == "done" {
            break;
        }

        println!("Enter years of experience:");
        let mut experience = String::new();
        io::stdin().read_line(&mut experience).expect("Failed to read input");
        let experience: u32 = experience.trim().parse().expect("Invalid input");

        println!("Enter preferred language of Programmingz:");
        let mut programming_language = String::new();
        io::stdin()
            .read_line(&mut programming_language)
            .expect("Failed to read input");
        let programming_language = programming_language.trim().to_string();

        developers.push((name.to_string(), experience, programming_language));
    }

    /* Find the developer with the highest experience */
    let mut max_experience = 0;
    let mut max_name = String::new();
    let mut max_language = String::new();
    for &(ref name, experience, ref programming_language) in &developers {
        if experience > max_experience {
            max_experience = experience;
            max_name = name.clone();
            max_language = programming_language.clone();
        }
    }

    // Display the result
    if max_experience > 0 {
        println!(
            "The developer with the highest experience is {}", max_name);
        println!("Language: {} ", max_language);
        println!("Years of experience: {}", max_experience);
    } else {
       
        println!("No developers found.");
}
}