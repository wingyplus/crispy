use copypasta::{ClipboardContext, ClipboardProvider};

fn main() -> Result<(), String> {
    let mut ctx = ClipboardContext::new().unwrap();
    match ctx.get_contents() {
        Ok(content) => {
            print!("{}", content);
            Ok(())
        }
        Err(_) => Err(String::from("Cannot get content from clipboard")),
    }
}
