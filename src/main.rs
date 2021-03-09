mod translation;

use std::io::{self, Write};
use translation::translate;

fn main() -> anyhow::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let project = prompt("What is your project about?", &stdin, &mut stdout)?;

    let project_in_esperanto = translate(&project, "en", "eo")?;
    println!("{}", project_in_esperanto);

    Ok(())
}

fn prompt(prompt: &str, stdin: &io::Stdin, stdout: &mut io::Stdout) -> anyhow::Result<String> {
    write!(stdout, "{} ", prompt)?;
    stdout.flush()?;

    let mut buf = String::new();
    stdin.read_line(&mut buf)?;

    Ok(buf.trim().to_string())
}
