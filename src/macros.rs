#[macro_export]
macro_rules! some_nempty {
    ($x:expr) => {
        {
            // look, it's either this or s.is_empty().not().then(s), okay? chill
            let s: String = $x;
            if s.is_empty() {
                None
            } else {
                Some(s)
            }
        }
    };
}
