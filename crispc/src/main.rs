use copypasta::{ClipboardContext, ClipboardProvider};
use std::io;
use std::io::Read;

fn main() -> Result<(), String> {
    let mut buffer = String::new();
    match io::stdin().read_to_string(&mut buffer) {
        Ok(_) => {
            let mut ctx = ClipboardContext::new().unwrap();
            match ctx.set_contents(buffer) {
                Ok(_) => Ok(()),
                Err(_) => Err(String::from("Cannot copy text to clipboard")),
            }
        }
        Err(_) => Err(String::from("Cannot copy text from standard input")),
    }
}
