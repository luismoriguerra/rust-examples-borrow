use std::borrow::Cow;

fn remove_spaces<'a>(input: &'a str) -> Cow<'a, str> {
    if input.contains(' ') {
        let mut buf = String::with_capacity(input.len());

        for c in input.chars() {
            if c != ' ' {
                buf.push(c);
            }
        }

        return Cow::Owned(buf);
    }

    return Cow::Borrowed(input);
}

// We can also use the Into trait to convert the &str or String into the proper Cow variant
fn remove_spaces_2<'a>(input: &'a str) -> Cow<'a, str> {
    if input.contains(' ') {
        let mut buf = String::with_capacity(input.len());
        let v: Vec<char> = input.chars().collect();

        for c in v {
            if c != ' ' {
                buf.push(c);
            }
        }

        return buf.into();
    }

    return input.into();
}


fn remove_spaces_3<'a>(input: &'a str) -> Cow<'a, str> {
    if input.contains(' ') {
        input
        .chars()
        .filter(|&x| x != ' ')
        .collect::<std::string::String>()
        .into()
    } else {
        input.into()
    }
}

// to mutate I need to owned the string

fn main() {
    let s = remove_spaces("Herman Radtke");  // s is a Cow::Borrowed variant
    println!("Length of string is {}", s.len());
    let len = s.len(); // immutable function call using Deref
    let owned: String = s.into_owned(); // memory is allocated for a new string
}
