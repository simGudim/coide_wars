use std::collections::HashMap;


pub fn duplicate_encode(word:&str)->String {
    let mut dictionary: HashMap<char, usize> = HashMap::new();
    let mut result: String = String::from("");
    for i in word.chars() {
        dictionary.insert(i, word.matches(i).count());
    }
    for i in word.to_lowercase().chars() {
        if let Some(letter) = dictionary.get(&i) {
            if *letter > 1usize {
                result.push_str(")");
            } else {
                result.push_str("(");
            }
        };
    }
    println!("{:?}", dictionary);
    result
}