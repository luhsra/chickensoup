use std::{fs, io};
use std::io::Write;

fn main() {
    print!("Enter the recipe directory: ");
    let _ = io::stdout().flush();

    let mut source = String::new();
    io::stdin().read_line(&mut source).unwrap();

    let entries = fs::read_dir(&source[..source.len()-1]).unwrap();
    for entry in entries {
        let path = entry.unwrap().path();
        let cont = fs::read_to_string(&path).unwrap();
        println!("{}\n{}", path.display(), cont);
    }
}
