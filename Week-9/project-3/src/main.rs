// Rust program to join three datasets of corrupt officials 
use std::io::Write;

fn main() 
{
	let commissioner_name = vec!["Aigbogun Alamba Daudu","Murtala Afeez Bendu","Okorocha Calistus Ogbona"
	,"Adewale Jimoh Akanbi","Osazuwa Faith Etieye"];
    let ministry = vec!["Internal affairs","Justice","Defense","Power & Steel","Petroleum"];
    let geopolitical_zone = vec!["South West","North East","South South","South West","South East"];

    let mut file = std::fs::File::create("Commissioners.txt").expect("Create failed");
    file.write_all("\t\t\t\t\t\t Officials Database".as_bytes()).expect("Failed");
    file.write_all("\nCommissioner Name      Ministry      Geo-Political Zone ".as_bytes()).expect("Failed");
    
    for i in 0..commissioner_name.len(){
    	file.write_all(commissioner_name[i].as_bytes()).expect("Failed");

    	file.write_all(ministry[i].as_bytes()).expect("Failed");

    	file.write_all(geopolitical_zone[i].as_bytes()).expect("Failed");
    }
    println!("\nFile created successfully");
}           