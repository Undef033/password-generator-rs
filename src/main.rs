use rand::prelude::*;

fn generate_pwd(len: u8, uppercase: bool, special_chars: bool) -> String {
    let mut chars = "abcdefghijklmnopqrstuvwxyz".to_string();
    let mut pwd = "".to_string();
    let mut rng = rand::thread_rng();

    if uppercase {
        chars += &chars.to_uppercase()
    }

    if special_chars {
        chars += "!#$%*+-/?@^_"
    }

    for _ in 0..len {
        let n = rng.gen_range(0..chars.len());
        pwd += chars.get(n..n + 1).unwrap();
    }

    pwd
}

fn get_inp<T: std::str::FromStr>(default: T) -> Result<T, std::io::Error> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line)?;

    Ok(line.trim().parse::<T>().unwrap_or(default))
}

fn get_inp_b(default: bool) -> Result<bool, std::io::Error> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line)?;

    Ok(match line.trim().to_lowercase().as_str() {
        "y" | "1" => true,
        "n" | "0" => false,
        _ => default,
    })
}

fn main() -> Result<(), std::io::Error> {
    println!("[16] Password length: ");
    let length = get_inp::<u8>(16)?;

    println!("[y] Special characters [Y/n]: ");
    let special_chars = get_inp_b(true)?;

    println!("[y] Uppercase [Y/n]: ");
    let uppercase = get_inp_b(true)?;

    println!("{}", generate_pwd(length, uppercase, special_chars));

    Ok(())
}
