use std::io;
use std::str::FromStr;

fn main() {
    println!("Give me the number to create the multiplication table: ");

    let mut number = String::new();

    io::stdin().read_line(&mut number).ok().expect("Error to read keyboard");

    let number: i32 = i32::from_str(&number.trim()).unwrap();
    
    println!("Your numer is {}", number);

    println!("The multiplication table is:");

    let mut table: Vec<i32> = Vec::new();

    for x in 0..11 {
        let mut multiplication = x*number;
        
        let index_vector: usize = x.try_into().unwrap();

        table.push(multiplication);

        multiplication = table[index_vector];

        println!("{} x {} = {}", number, x, multiplication);
    }
}
