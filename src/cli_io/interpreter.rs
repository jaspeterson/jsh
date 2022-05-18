// Interpreter object for parsing and interpreting a line of input
pub struct Interpreter<'a> {
    tokens: Vec<&'a str>,
}

impl Interpreter<'_> {
    // Constructor method that takes a line of input deliminated by spaces and separates it into a vector of tokens
    pub fn tokenize<'a>(&self, s: &'a String) -> Interpreter<'a> {
        let mut v: Vec<&str> = s.as_str().split(' ').collect();
        trim_spaces(&mut v);
        Interpreter {
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