use rand::Rng;
use std::io::{self, Write, BufRead, BufReader};
use std::fs::{OpenOptions, File};
use std::collections::HashMap;

fn create_password(name: String) {
    let mut rng = rand::thread_rng();

    let charset = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
        abcdefghijklmnopqrstuvwxyz\
        0123456789\
        !@#$%^&*()_+-={}[]|:;<>,.?/";

    let password: String = (0..8)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset[idx] as char
        })
        .collect();

    let mut dict = HashMap::new();

    dict.insert(name, password);

    let mut content = String::new();
    for (key, value) in &dict {
        content.push_str(&format!("{}:{}\n", key, value));
    }

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("passwords.txt")
        .expect("Failed to open file");

    file.write_all(content.as_bytes()).expect("Failed to write to file!");
}

fn view_password() -> io::Result<()> {
    let file = File::open("passwords.txt").expect("Failed to open file!"); // Opening the file
    let reader = BufReader::new(file); // Reading from the file

    for line in reader.lines() {
        let line = line?;
        println!("{}", line);
    }

    Ok(())
}

fn update_password(label: String) {
    let file = File::open("passwords.txt").expect("Failed to open file!");
    let reader = BufReader::new(file);

    let mut data: HashMap<String, String> = HashMap::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split(':').collect();
        data.insert(parts[0].to_string(), parts[1].to_string());
    }

    match data.get(&label) {
        Some(_) => {
            let mut new_password = String::new();
            print!("Enter the new password: ");
            io::stdout().flush().expect("Flush failed!");
            io::stdin()
                .read_line(&mut new_password)
                .unwrap();
            data.insert(label, new_password.trim().to_string());

            let mut content = String::new();
            for (key, value) in &data {
                content.push_str(&format!("{}:{}\n", key, value));
            }

            let mut file = OpenOptions::new()
                .write(true)
                .open("passwords.txt")
                .expect("Failed to open file");

            file.write_all(content.as_bytes()).expect("Failed to write to file!");
            println!("Password updated successfully!");
        },
        None => println!("Not found!"),
    }
}

fn main() {
    let mut input = String::new();

    println!("\n1. Create password\n2. View password\n3. Update password\n4. Delete password");
    print!("\nEnter your choice: ");
    io::stdout().flush().expect("Flush failed!");
    
    // Reads input string
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input!");

    // Converts the string to a single char
    let choice = input.trim().chars().next().unwrap();

    match choice {
        '1' => { 
            let mut label = String::new();
            print!("Enter a label for the password: ");
            io::stdout().flush().expect("Flush failed!");
            io::stdin()
                .read_line(&mut label)
                .expect("Failed to read label!");
            create_password(label.trim().to_string());
        }
        '2' => view_password().expect("Failed to view passwords"),
        '3' => {
            let mut label = String::new();
            print!("Enter a label of the password to change: ");
            io::stdout().flush().expect("Flush failed!");
            io::stdin()
                .read_line(&mut label)
                .unwrap();

            update_password(label.trim().to_string());
        }
        '4' => println!("Password deleted!"),
        _ => println!("Invalid choice"),
    }
}
