use anyhow::Result;
use const_gen::*;
use std::{collections::HashSet, env, fs, path::Path};

const DICTIONARY_URL: &str = "https://github.com/dwyl/english-words/raw/master/words_alpha.txt";

fn main() -> Result<()> {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("dictionary.rs");
    let dictionary_words = reqwest::blocking::get(DICTIONARY_URL)?.text()?;
    let dictionary: HashSet<String> = HashSet::from_iter(
        dictionary_words
            .split_whitespace()
            .filter(|s| s.len() == 5)
            .map(|s| s.to_uppercase()),
    );
    fs::write(&dest_path, const_declaration!(DICTIONARY = dictionary))?;
    println!("cargo:rerun-if-changed=build.rs");
    Ok(())
}
