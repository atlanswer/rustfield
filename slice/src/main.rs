fn main() {
    let mut s = String::from("Hello world!");

    let hello = &s[0..5];
    let world = &s[6..11];

    let first_world = first_word(&s);

    s.clear();

    // println!("The first world is {}", first_world);

    let a = [1, 2, 3, 4, 5];
    let slice_a = &a[1..3];
    assert_eq!(slice_a, &[2, 3]);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
