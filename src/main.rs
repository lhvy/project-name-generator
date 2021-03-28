mod translation;

use std::io::{self, Write};
use translation::translate;

fn main() -> anyhow::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    let input = prompt("What is your project about?", &stdin, &mut stdout)?;

    let names = input.split_whitespace().map(|s| s.to_string()).collect();
    let names = sanitize_project_names(names).collect();

    let names = get_project_names_from_synonyms(names);
    let names = sanitize_project_names(names);

    let mut translated_names = Vec::new();

    for word in names {
        let in_esperanto = translate(&word, "en", "eo")?;
        translated_names.push(any_ascii::any_ascii(&in_esperanto));
    }

    let names = sanitize_project_names(translated_names);

    for name in names {
        println!("{}", name);
    }

    Ok(())
}

fn get_project_names_from_synonyms(mut names: Vec<String>) -> Vec<String> {
    let mut names_with_synonyms: Vec<_> = names
        .iter()
        .flat_map(|keyword| thesaurus::synonym(keyword).map_or_else(|_| Vec::new(), |d| d.synonyms))
        .collect();

    names_with_synonyms.append(&mut names);

    names_with_synonyms
}

fn sanitize_project_names(mut names: Vec<String>) -> impl Iterator<Item = String> {
    names.sort_unstable();
    names.dedup();
    names.into_iter().map(|name| name.to_lowercase())
}

fn prompt(prompt: &str, stdin: &io::Stdin, stdout: &mut io::Stdout) -> anyhow::Result<String> {
    write!(stdout, "{} ", prompt)?;
    stdout.flush()?;

    let mut buf = String::new();
    stdin.read_line(&mut buf)?;

    Ok(buf.trim().to_string())
}
