mod translation;

use std::io::{self, Write};
use translation::translate;

fn main() -> anyhow::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let project = prompt("What is your project about?", &stdin, &mut stdout)?;

    let synonyms = thesaurus::synonym(&project).map_or_else(|_| Vec::new(), |d| d.synonyms);

    for word in synonyms.into_iter().chain(std::iter::once(project)) {
        let in_esperanto = translate(&word, "en", "eo")?;
        println!("{}", in_esperanto);
    }

    Ok(())
}

fn prompt(prompt: &str, stdin: &io::Stdin, stdout: &mut io::Stdout) -> anyhow::Result<String> {
    write!(stdout, "{} ", prompt)?;
    stdout.flush()?;

    let mut buf = String::new();
    stdin.read_line(&mut buf)?;

    Ok(buf.trim().to_string())
}
