fn main() {
    let string1 = "Hello,";
    let string2 = " World!";
    let concatenated_string = concatenate_string(string1, string2);
    println!("{}", concatenated_string);
}

fn concatenate_string(s1: &str, s2: &str) -> String {
    let mut result = String::new();
    result.push_str(s1);
    result.push_str(s2);
    result
}
