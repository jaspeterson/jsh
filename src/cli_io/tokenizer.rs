// Tokenizer object for parsing a line of input
pub struct Tokenizer<'a> {
    pub tokens: Vec<&'a str>,
}

impl Tokenizer<'_> {
    // Constructor method that takes a line of input deliminated by spaces and separates it into a vector of tokens
    pub fn tokenize<'a>(s: &'a String) -> Tokenizer<'a> {
        let mut v: Vec<&str> = s.split(' ').collect();
        trim_spaces(&mut v);
        Tokenizer {
            tokens: v,
        }
    }
}

fn trim_spaces(v: &mut Vec<&str>) {
    let mut pos = 0;
    while pos < v.len() {
        if v[pos] == "" {
            v.remove(pos);
        } else {
            pos += 1;
        }
    }
}