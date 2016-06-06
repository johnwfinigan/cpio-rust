use std::io;
use std::io::prelude::*;
use std::error::Error;
use std::fs::File;


fn main() {
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => {
                break;
            }
            Ok(_) => {

                let mut file = match File::open(input.trim()) {
                    Err(why) => panic!("couldn't open {}: {}", input.trim(), why.description()),
                    Ok(file) => file,
                };

                loop {
                    let mut buf = [0; 4096];
                    match file.read(&mut buf) {
                        Ok(0) => {
                            break;
                        }
                        Ok(n) => n,
                        Err(why) => panic!(why),
                    };


                    match io::stdout().write_all(&buf) {
                        Ok(_) => (),
                        Err(why) => panic!(why),
                    };

                }
            }
            Err(why) => println!("error reading stdin: {}", why),
        }
    }
}
