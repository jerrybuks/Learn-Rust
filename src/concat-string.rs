fn main() {
    let mut string1 = String::from("Hello");
    let mut string2 = String::from(" World");

    concatenate_strings(&mut string1, &mut string2);
}

fn concatenate_strings (string1: &mut String,string2: &mut String) {
    let result = string1;

    result.push_str(string2);
    println!("{}", result);
    
}

