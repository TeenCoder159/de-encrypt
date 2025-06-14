use rand::Rng;

fn encrypt(target: String) -> (String, Vec<u8>) {
    let mut result = "".to_string();
    let mut i = 0;
    let key = generate_key(target.clone());
    for char in target.chars() {
        for value in char.to_string().as_bytes() {
            let remainder = value % key[i];
            let divider = (value - remainder.clone()) / 4;
            result.push_str(format!("{}${}/", divider, remainder).as_str());
            i += 1;
        }
    }

    (remove_last_char(result), key)
}

fn remove_last_char(target: String) -> String {
    let b = target.chars().count() - 1;
    let (a, _) = target.split_at(b);
    a.to_string()
}

// fn decrypt(target: String, key: Vec<u8>) -> String {
//     let mut result = Vec::new();
//     let mut i = 0;
//     // for byte_rep in target.split('/') {
//     //     let (divider, remainder) = byte_rep.split_once('$').expect("Error:");
//     //     let byte = &[
//     //         divider.parse::<u8>().expect("msg") * key[i] + remainder.parse::<u8>().expect("msg")
//     //     ];
//     //     result.push(std::str::from_utf8(byte).expect("msg"));
//     //     i += 1;
//     // }
//     for byte_rep in target.split('/') {
//         if byte_rep.is_empty() {
//             continue;
//         }
//         let (divider, remainder) = byte_rep.split_once('$').expect("Error in format");
//         let byte = divider.parse::<u8>().expect("Invalid divider") * 4
//             + remainder.parse::<u8>().expect("Invalid remainder");
//         result.push(byte);
//         i += 1;
//     }
//     String::from_utf8(result).unwrap_or_else(|e| format!("UTF-8 error: {}", e))
// }

fn decrypt(target: String, key: Vec<u8>) -> String {
    let mut result = Vec::new(); // Store bytes instead of a String initially
    let mut i = 0;
    for byte_rep in target.split('/') {
        if byte_rep.is_empty() {
            continue;
        }
        let (divider, remainder) = byte_rep.split_once('$').expect("Error in format");
        let byte = divider.parse::<u8>().expect("Invalid divider") * key[i]
            + remainder.parse::<u8>().expect("Invalid remainder");
        result.push(byte);
        i += 1;
    }
    // Convert the collected bytes to a String at the end
    String::from_utf8(result).unwrap_or_else(|e| format!("UTF-8 error: {}", e))
}

fn generate_key(input: String) -> Vec<u8> {
    let mut res = Vec::new();
    for _ in input.as_bytes() {
        res.push(rand::rng().random_range(1..10) as u8);
    }

    res
}

fn main() {
    let (encrypted, key) = encrypt("target".to_string());

    println!("{}", encrypted);
    let decrypted = decrypt(encrypted, key);
    println!("{}", decrypted)
}
