use std::io::{self, Write, Read};
use termion::clear;

struct Contact {
	name: String,
	phone: String,
	email: String,
}

struct ContactBook {
   contacts: Vec<Contact>,
}

impl ContactBook {
	fn new() -> Self {
		Self {
			contacts: Vec::new(),
		}
	}

	// Need Checks for phone and email
	fn create_contact(&mut self) {
		let mut name = String::new();
		io::stdin()
			.read_line(&mut name)
			.expect("Failed to read line");
		name.trim();
		let mut phone = String::new();
		io::stdin()
			.read_line(&mut phone)
			.expect("Failed to read line");
		phone.trim();
		let mut email = String::new();
		io::stdin()
			.read_line(&mut email)
			.expect("Failed to read line");
		email.trim();
		let contact: Contact = Contact {
			name,
			phone,
			email,
		};
		self.contacts.push(contact);
	}

	fn display_contacts(&mut self) {
		for contact in &self.contacts {
			println!("--------------------------");
			println!("name: {}", contact.name);
			println!("phone: {}", contact.phone);
			println!("email: {}", contact.email);
			println!("--------------------------");
			println!("")
		}
	}

	// fn delete_contact();
	// fn edit_contact();
}

// fn clear_screen() {
// 	print!("{}{}", clear::All, termion::cursor::Goto(1, 1));
// 	io::stdout().flush().unwrap();
// }

fn main() {
	let mut contact_book = ContactBook::new();

	loop {
		// clear_screen();
		println!("Choose an option:");
		println!("c: Create a contact");
		println!("p: Print contacts");
		println!("q: Quit");

		let mut buffer = [0, 2];
		
		match io::stdin().read(&mut buffer) {
			Ok(n) if n == 1 => {
				let input = buffer[0] as char;
				match input {
					'c' => contact_book.create_contact(),
					'p' => contact_book.display_contacts(),
					'q' => break,
					_ => println!("Not one of the options"),
				}
			}
			Ok(_) => {
				println!("Input must be one character");
			}
			Err(error) => {
				eprintln!("Failed to read input: {}", error);
			}
		}
	}
	// Creating two contacts
	//contact_book.create_contact("Mike Edubas".to_string(), "+3578347332".to_string(), "mikeedubas@hotmail.com".to_string());
	//contact_book.create_contact("Anastasia Romanova".to_string(), "+3578347456".to_string(), "anastasia.romanova@hotmail.com".to_string());
	//contact_book.display_contacts();
}
