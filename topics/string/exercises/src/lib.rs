pub fn hello() -> String {
    "Hello Rust".to_string()
}

pub fn greet(name: &str) -> String {
    let text: String = "Hello ".to_string();
    text + name
}

pub fn append(mut s: String) -> String {
    s += "!";
    s
}
