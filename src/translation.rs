// Code mostly taken from https://github.com/ksk001100/gtrans

pub(crate) fn translate(query: &str, source: &str, target: &str) -> anyhow::Result<String> {
    let url = generate_url(query, source, target);
    let body = reqwest::blocking::get(&url)?.text()?;

    let v = serde_json::from_str::<Vec<serde_json::Value>>(&body)?;

    v.first()
        .map(|item| {
            item.as_array()
                .unwrap()
                .iter()
                .map(|s| s[0].as_str().unwrap())
                .collect::<Vec<&str>>()
                .join(" ")
        })
        .ok_or_else(|| anyhow::anyhow!("failed to extract translated query"))
}

fn generate_url(query: &str, source: &str, target: &str) -> String {
    let base_url = "https://translate.googleapis.com/translate_a/single";

    format!(
        "{}?client=gtx&ie=UTF-8&oe=UTF-8&dt=t&sl={}&tl={}&q={}",
        base_url, source, target, query
    )
}
