use tokio::process::Command;
use std::io::Write;
use crate::utls::parser::shortname_to_qualified;
use serde::*;
use uuid::Uuid;
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Default)]
struct LinguistOutput {
    lines : i32,
    sloc : i32,
    #[serde(rename = "buildid")]
    result_type : String,
    mime_type : String,
    language : String,
    large : bool,
    generated : bool,
    vendored : bool,
}

pub async fn get_language_from(content : &str) -> serenity::Result<String> {
    let mut dir = std::env::temp_dir();
    dir.push(format!("{}", Uuid::new_v4()));
    let mut file = std::fs::File::create(&dir)?;
    let _ = file.write_all(content.as_bytes());
    let _ = file.flush();


    let child = Command::new("github-linguist")
        .arg("--json")
        .arg(&dir)
        .spawn()?;

    let output = child.wait_with_output().await?;
    let _ = std::fs::remove_file(&dir)?;

    let stdout = String::from(String::from_utf8_lossy(&output.stdout));
    let linguist : HashMap<String, LinguistOutput>  = serde_json::from_str(&stdout)?;
    let name = String::from(dir.to_string_lossy());
    let linguist_out = linguist.get(&name).unwrap();
    Ok(String::from(shortname_to_qualified(&linguist_out.language.to_lowercase())))
}