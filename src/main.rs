// Basic Rust Program to print "Hello World" with Rust Mascot 

use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let message = String::from("Hello, welcome to Rust Development ^_* !!!
        This main.rs file will be updated in upcoming sessions with SOL DAP.");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(&message, width, &mut writer).unwrap();
}

