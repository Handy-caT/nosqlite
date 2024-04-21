mod r#static;

use crate::r#static::welcome;
use std::{io, io::Write};

fn clear_screen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

fn repl() -> io::Result<()> {
    let mut buffer = String::new();
    loop {
        print!("> ");
        let _ = io::stdout().flush();
        io::stdin().read_line(&mut buffer)?;

        println!("{}", buffer);
        buffer.clear();
    }
}

fn main() -> io::Result<()> {
    clear_screen();
    welcome();
    repl()?;

    Ok(())
}
