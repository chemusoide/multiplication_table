# Create multiplication table

##📄 Description:
Enter a number, and the programe create a multiplication table for that number:	use std::io;

### Use this library to obtain string from keyboard
	use std::str::FromStr;

### Use library here to read the line:
	io::stdin().read_line(&mut number).ok().expect("Error to read keyboard");
	
### Transform string to i32
	let number: i32 = i32::from_str(&number.trim()).unwrap();

### To use the index of the for to search the array     
	let index_vector: usize = x.try_into().unwrap();

## 🤖 Test
cargo test

## 🖥️ Running the program
cargo run