fn encrypt(target: String) -> String {
    let mut result = "".to_string();

    for char in target.chars() {
        for value in char.to_string().as_bytes() {
            let remainder = value % 4;
            let divider = (value - remainder.clone()) / 4;
            result.push_str(format!("{}${}/", divider, remainder).as_str());
        }
    }

    remove_last_char(result)
}

fn remove_last_char(target: String) -> String {
    let b = target.chars().count() - 1;
    let (a, _) = target.split_at(b);
    a.to_string()
}

fn decrypt(target: String) -> String {
    let mut result = String::new();
    for byte_rep in target.split('/') {
        let (divider, remainder) = byte_rep.split_once('$').expect("Error:");
        let byte =
            &[divider.parse::<u8>().expect("msg") * 4 + remainder.parse::<u8>().expect("msg")];
        result.push_str(std::str::from_utf8(byte).expect("msg"));
    }
    result
}

fn main() {
    println!("{}", decrypt(encrypt("target".to_string())))
}
