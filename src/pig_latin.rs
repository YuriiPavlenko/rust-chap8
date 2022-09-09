pub fn to_pig_latin(text: &str) {
    if text.trim() == "" {
        println!("Can't piglatin an empty string!");
        return;
    }
    let words = String::from(text);
    let words = words.split_whitespace();
    let mut new_words = String::new();
    for word in words {
        if word.starts_with('a')
            || word.starts_with('e')
            || word.starts_with('u')
            || word.starts_with('o')
            || word.starts_with('i')
        {
            let mut new_word = String::from(word);
            new_word.push_str("hay");
            new_words.push_str(&new_word);
            new_words.push_str(" ");
        } else {
            let (left, right) = word.split_at(1);
            let mut new_word = String::from(right);
            new_word.push_str(left);
            new_word.push_str("ay");
            new_words.push_str(&new_word);
            new_words.push_str(" ");
        }
    }
    println!("{}", new_words);
}
