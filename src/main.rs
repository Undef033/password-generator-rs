use rand::prelude::*;
use std::io::Error;

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

fn get_inp<T>(default: T) -> Result<T, Error>
where
    T: std::str::FromStr + std::convert::TryInto<T>,
{
    let mut line = String::new();
    std::io::stdin().read_line(&mut line)?;

    match line.trim().parse::<T>() {
        Ok(val) => Ok(val),
        _ => Ok(default),
    }
}

fn main() -> Result<(), Error> {
    println!("[16] Password length: ");
    let length = get_inp::<u8>(16)?;

    println!("[1] Special characters [1/0]: ");
    let special_chars = get_inp::<i8>(1)?;

    println!("[1] Uppercase [1/0]: ");
    let uppercase = get_inp::<i8>(1)?;

    println!("{} {} {}", length, special_chars, uppercase);

    println!(
        "{}",
        generate_pwd(length, uppercase != 0, special_chars != 0)
    );

    Ok(())
}
