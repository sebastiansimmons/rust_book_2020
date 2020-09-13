fn translate(s: &String) -> String {
    let split = s.split(" ");
    let mut cop_spanish = String::new();
    for word in split {
        let mut new_word = String::from(word);
        let c = new_word.remove(0);
        new_word.push(c);
        new_word.push_str("ay");
        println!("{}", new_word);
        cop_spanish.push_str(&new_word);
        cop_spanish.push_str(" ");
    }
    cop_spanish
}

fn main() {
    let latin = String::from("You, sir, are a simp!");
    let piglish = translate(&latin);
    println!("{}", piglish);
}
