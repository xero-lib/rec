use std::io;

pub fn read_line() -> String {
    let mut s = String::new();
    let _ = io::stdin().read_line(&mut s).unwrap();
    s.trim().into()
}
