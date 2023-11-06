use std::{collections::HashMap, io};

fn main() {
    loop {
        let mut text = String::new();

        io::stdin()
            .read_line(&mut text)
            .expect("Failed to read the line");

        let letters: Vec<char> = text.trim().chars().collect();

        let mut letters_map: HashMap<char, i32> = HashMap::new();

        for letter in letters {
            if letters_map.contains_key(&letter) {
                let value = letters_map.get(&letter).unwrap();
                letters_map.insert(letter, value + 1);
            } else {
                letters_map.insert(letter, 1);
            }
        }

        let found = letters_map.values().any(|&value| value > 1);

        if found {
            println!("Не все символы встречаются только 1 раз");
        } else {
            println!("Все символы встречаются только 1 раз");
        }
    }
}
