fn main() {
    let s = "hello".to_owned();
    let r = reverse(s.clone());

    println!("Original: {}", s);
    println!("Reversed: {}", r);
}

fn reverse(s: String) -> String {
    s.chars().rev().collect()
}
