fn main() {
    let _s = "hello";
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", first_word(&s));
    println!("{}", second_word(&s));
    show_message(&mut s);
    s.push_str("Goodbye!");
    show_message(&mut s);
    s.push_str("second ");
    println!("{}", second_word(&s));
}

fn show_message(s: &mut String) {
    println!("{}", s);
    s.clear();
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    s
}

fn second_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    let mut is_first_word = true;
    let mut second_word_index = 0;
    for (i, &item) in bytes.iter().enumerate() {
        if item != b' ' {
            continue;
        }

        if is_first_word {
            second_word_index = i + 1;
            is_first_word = false;
        } else {
            return &s[second_word_index..i];
        }
    }

    ""
}
