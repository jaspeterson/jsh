pub fn tokenize(s: &String) -> Vec<String> {
    let mut v: Vec<String> = s.split(' ').map(|st|String::from(st)).collect();
    trim_spaces(&mut v);
    v
}

fn trim_spaces(v: &mut Vec<String>) {
    let mut pos = 0;
    while pos < v.len() {
        if v[pos] == "" {
            v.remove(pos);
        } else {
            pos += 1;
        }
    }
}