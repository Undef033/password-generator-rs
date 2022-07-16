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

fn get_inp<T>(default: T) -> Result<T, std::io::Error>
where
    T: std::str::FromStr + std::any::Any + std::convert::From<bool>,
{
    let mut line = String::new();
    std::io::stdin().read_line(&mut line)?;

    if (&default as &dyn std::any::Any)
        .downcast_ref::<bool>()
        .is_some()
    {
        return Ok(match line.trim().to_lowercase().as_str() {
            "y" | "1" => true.into(),
            "n" | "0" => false.into(),
            _ => default,
        });
    }

    Ok(line.trim().parse::<T>().unwrap_or(default))
}

fn main() -> Result<(), std::io::Error> {
    println!("[16] Password length: ");
    let length = get_inp::<u8>(16)?;

    println!("[y] Special characters [Y/n]: ");
    let special_chars = get_inp::<bool>(true)?;

    println!("[y] Uppercase [Y/n]: ");
    let uppercase = get_inp::<bool>(true)?;

    println!("{}", generate_pwd(length, uppercase, special_chars));

    Ok(())
}
