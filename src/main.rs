use rustyline::Editor;
use rustyline::error::ReadlineError;

fn main() {
    let mut reader = Editor::<()>::new()?;
    loop {
        let line = reader.readline(">> ");
        match line {
            Ok(contents) => {
                println!("TODO: Make this");
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL+C!");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL+D!");
                break;
            }
            Err(err) => {
                println!("Unexpected error: {:?}", err);
                break;
            }
        }
    }
}
