extern crate reqwest;
extern crate tempfile;
extern crate zip;

use itertools::Itertools;
use std::{collections::HashMap, env, io::Read};
use url::Url;

fn main() {
    let url = Url::parse(&env::args().nth(1).expect("Must provide a URL")).expect("Invalid URL");

    let mut tmpfile = tempfile::tempfile().unwrap();
    reqwest::blocking::get(url)
        .expect("Error fetching file")
        .copy_to(&mut tmpfile)
        .unwrap();

    let mut archive = zip::ZipArchive::new(tmpfile).expect("Error reading archive");

    let target_list = [
        "func_148254_d",
        "awt/Robot",
        "squareup/okhttp",
        "launcher_accounts.json",
        ".minecraft/versions",
        ".minecraft\\versions",
        ".minecraft/mods",
        ".minecraft\\mods",
        "Local Storage",
        "leveldb",
        "APPDATA",
        "Google\\Chrome",
        "Login Data",
        "user.home",
        "checkip.amazonaws",
        "discord.com/api",
        "discordapp.com/api",
        "dropboxusercontent",
        "drive.google",
    ];

    let mut nasty_files: HashMap<String, Vec<String>> = HashMap::new();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes).unwrap();

        for word in target_list {
            for i in bytes.windows(word.len()) {
                if i == word.as_bytes() {
                    // push the file name to nasty_files
                    let entry = nasty_files
                        .entry(file.enclosed_name().unwrap().to_str().unwrap().to_owned())
                        .or_insert(Vec::new());
                    entry.push(word.to_string());
                    break;
                }
            }
        }
    }
    for (file, words) in nasty_files.iter().sorted() {
        println!("{}\n{:?}", file, words);
    }
}
