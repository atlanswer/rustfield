fn main() {
    let s = String::from("Hello");
    let mut s2 = String::from(",");
    let _s4 = &s2;
    let _s5 = &s2;
    let s3 = &mut s2;
    add_space(s3);
    add_world(&mut s2);
    println!("{}{}", s, s2);
}

fn add_space(s: &mut String) {
    s.push_str(" ");
}

fn add_world(s: &mut String) {
    s.push_str("world!");
}
