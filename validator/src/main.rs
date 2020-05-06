use beatmap::Beatmap;
use std::{
    env,
    fs::File,
    io::{self, BufReader},
};

fn main() {
    for filename in env::args().skip(1) {
        println!("Validating `{}`...", filename);
        let file = match File::open(&filename) {
            Ok(f) => BufReader::new(f),
            Err(e) => {
                eprintln!("Error reading `{}`: {}", filename, e);
                continue;
            }
        };
        if let Err(e) = Beatmap::read(file) {
            eprintln!("`{}` is invalid: {}", filename, e);
        } else {
            println!("`{}` is valid", filename);
        }
        println!();
    }
    println!("Press any key to exit...");
    io::stdin().read_line(&mut String::new()).unwrap();
}
