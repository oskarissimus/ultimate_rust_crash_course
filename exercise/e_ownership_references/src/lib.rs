pub fn inspect(word: &String) {
    let grammatical_number = if word.ends_with("s") {
        "plural"
    } else {
        "singular"
    };
    println!("{} is {}", word, grammatical_number)
}

pub fn change(word: &mut String) {
    if !word.ends_with("s") {
        word.push_str("s")
    }
}

pub fn eat(word: String) -> bool {
    word.contains("a") && word.starts_with("b")
}

pub fn bedazzle(word: &mut String) {
    (*word) = "sparkly".to_string()
}
