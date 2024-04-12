// converting from String to &str is cheap and implicit.
// borrow without taking ownership
// immutable reference (ref)
// no copying of data
fn three_vowels(word: &str) -> bool {
    let mut vowel_count = 0;
    for c in word.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                vowel_count += 1;
                if vowel_count >= 3 {
                    return true;
                }
            }
            _ => vowel_count = 0,
        }
    }

    false
}

fn main() {
    let ferris = "Ferris".to_string();
    let curious = "Curious".to_string();

    println!("{}: {}", ferris, three_vowels(&ferris));
    println!("{}: {}", curious, three_vowels(&curious));

    //  This works fine, but the following two lines would fail:
    println!("Ferris: {}", three_vowels("Ferris"));
    println!("Curious: {}", three_vowels("Curious"));

    let sentence_string = "Once upon a time, there was a friendly curious crab named Ferris";
    for words in sentence_string.split_whitespace() {
        if three_vowels(words) {
            println!("{} has 3 vowels", words);
        }
    }
}
