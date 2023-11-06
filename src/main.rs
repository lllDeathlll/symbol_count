use std::{collections::HashMap, io};

fn main() {
    // Asking user to type a string
    println!("Hi, I'm a program that checks if any letter in text repeats!");
    println!("Enter any text:");

    loop {
        // Reading text from user input
        let mut text = String::new();

        io::stdin()
            .read_line(&mut text)
            .expect("Failed to read the line");

        // Getting chars vector from string
        let letters: Vec<char> = text.trim().chars().collect();
        // Creating hashmap
        let mut letters_map: HashMap<char, i32> = HashMap::new();

        /*
        Looping through every letter
        and adding 1 for each appearance
        */
        for letter in letters {
            if letters_map.contains_key(&letter) {
                let value = letters_map.get(&letter).unwrap();
                letters_map.insert(letter, value + 1);
            } else {
                letters_map.insert(letter, 1);
            }
        }

        // Checks if any value is greater than one
        let found = letters_map.values().any(|&value| value > 1);

        // Prints result
        if found {
            println!("Не все символы встречаются только 1 раз");
        } else {
            println!("Все символы встречаются только 1 раз");
        }
    }
}
