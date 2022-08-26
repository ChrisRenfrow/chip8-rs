use std::{env, fs, io::Error};

struct Config<'a> {
    file_path: &'a str,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        eprintln!("Need file path!");
    }

    run(Config {
        file_path: &args[1],
    });
}

fn run(config: Config) -> Result<Vec<&str>, Error> {
    read_file(&config.file_path)
}

fn read_file(path: &str) -> Result<Vec<&str>, Error> {
    let file_content = fs::read_to_string(path)?;

    Ok(file_content.split(' ').collect::<Vec<String>>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        todo!()
    }
}
