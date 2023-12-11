// Rust program that saves the types of drinks
use std::io::Write;

fn main() {

	let mut lager_file = std::fs::File::create("Lager.txt").expect("Create Failed");
	lager_file.write_all("33 Export
		            Desperados
		            Goldberg\n
		            Gulder\n
		            Heineken\n
		            Star\n"
		.as_bytes()).expect("Write Failed");

	let mut stout_file = std::fs::File::create("Stout.txt").expect("Create Failed");
	stout_file.write_all("Legend
		            Turbo King
		            Williams"
		.as_bytes()).expect("Write Failed");

	let mut non_alchoholic_file = std::fs::File::create("Non-Alchoholic.txt").expect("Create Failed");
	non_alchoholic_file.write_all("Maltina
		            Amstel Malta
		            Malta Gold
		            Fayrouz"
		.as_bytes()).expect("Write Failed");
	
	
    println!("\nData Written to file.");
}
