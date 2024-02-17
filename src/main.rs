use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::thread;
use std::time::Duration;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        return;
    }

    for arg in &args[1..] {
        let path = Path::new(&arg);
        let display = path.display();
        print!("---{}---\n\n", display);
        let mut file = match File::open(path) {
            Err(why) => panic!("couldn't open {}: {}", display, why),
            Ok(file) => file,
        };

        let mut s = String::new();

        let data = match file.read_to_string(&mut s) {
            Err(why) => panic!("couldn't read {}: {}", display, why),
            Ok(_) => s.split('\n').collect::<Vec<&str>>(),
        };

        slowprint(data, 1000)
    }
}

fn slowprint(data: Vec<&str>, mills: u64) {
    for item in data {
        println!("{}", item);
        let interval = Duration::from_millis(mills);
        thread::sleep(interval)
    }
}
