use std::io;
use std::io::prelude::*;
use std::error::Error;
use std::fs::File;
use std::path::Path;


fn main() {
  loop {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
	Ok(0) => { break; },
        Ok(_) => {

            let path = Path::new(&input);
            let display = path.display();
            let mut file = match File::open(&path) {
                Err(why) => panic!("couldn't open {}: {}", display, why.description()),
                Ok(file) => file,
            };

            loop {
                let mut buf = [0; 4096];
                match file.read(&mut buf) {
	 	    Ok(0) => { break; },
                    Ok(n) => n,
                    Err(why) => panic!(why),
                };


                    match io::stdout().write_all(&buf) {
                        Ok(_) => (),
                   	Err(why) => panic!(why),
                    };

                

            }
        },
        Err(error) => println!("error: {}", error),
    }
 }
}
