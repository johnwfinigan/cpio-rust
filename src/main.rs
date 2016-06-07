use std::io;
use std::io::prelude::*;
use std::error::Error;
use std::fs::File;


fn main() {
    loop {
        let mut input = String::new();
        let mut stderr = io::stderr();
        match io::stdin().read_line(&mut input) {
            Ok(0) => {
                break;
            }
            Ok(_) => {

                let mut file = match File::open(input.trim()) {
                    Err(why) => panic!("couldn't open {}: {}", input.trim(), why.description()),
                    Ok(file) => file,
                };

                let mut bytesread = 0;
                loop {
                    let mut buf = [0; 4096];
                    bytesread = match file.read(&mut buf) {
                        Ok(0) => {
                            writeln!(&mut stderr, "bytes read {}", bytesread).unwrap();
                            break;
                        }
                        Ok(n) => n,
                        Err(why) => panic!(why),
                    };

                    io::stdout().write_all(&buf).unwrap();

                }
            }
            Err(why) => println!("error reading stdin: {}", why),
        }
    }
}
