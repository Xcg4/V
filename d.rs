fn main() {

	let mut text = String::new();
	let inp = std::io::stdin();

	println!("enter anme:");
	inp.read_line(&mut text).expect("Failed to read line");
	
	println!("prompt:{}", text);
	
	let mut text = String::new();
	let inp = std::io::stdin();

	println!("enter anme:");
	inp.read_line(&mut text).expect("Failed to read line");

}