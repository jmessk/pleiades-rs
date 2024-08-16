use std::{borrow::Cow, default, sync::Arc};

fn main() {
    let start = std::time::Instant::now();
    let s = "asdfasdf".to_string();
    println!("1: {:?}", start.elapsed());

    let start = std::time::Instant::now();
    let _ = s.clone();
    println!("2: {:?}", start.elapsed());

    let s = Arc::new("asdfasdf".to_string());
    let start = std::time::Instant::now();
    let _ = s.clone();
    println!("3: {:?}", start.elapsed());

    let s = Cow::Borrowed("asdfasdf");
    let start = std::time::Instant::now();
    let _ = s.clone();
    println!("4: {:?}", start.elapsed());

    let s: Cow<str> = Cow::Owned("asdfasdf".to_string());
    let start = std::time::Instant::now();
    let _ = s.clone();
    println!("5: {:?}", start.elapsed());

    let s = Cow::Borrowed("asdfasdf");
    let start = std::time::Instant::now();
    let s = Arc::new(s);
    println!("6: {:?}", start.elapsed());

    let start = std::time::Instant::now();
    let _ = s.clone();
    println!("7: {:?}", start.elapsed());

    let s: Cow<str> = Cow::Owned("asdfasdf".to_string());
    let s = Arc::new(s);
    let start = std::time::Instant::now();
    let _ = s.clone();
    println!("8: {:?}", start.elapsed());

    let s = "asdfasdf";
    let start = std::time::Instant::now();
    let _ = s.clone();
    println!("9: {:?}", start.elapsed());
}
