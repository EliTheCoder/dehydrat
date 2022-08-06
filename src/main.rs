use itertools::Itertools;
use std::{collections::HashMap,	fs,	io::{Read, self}};
use pbr::ProgressBar;

fn main() {
	let args = std::env::args().collect::<Vec<_>>();
	if args.len() != 2 {
		println!("Usage: {} <file>", args[0]);
		std::process::exit(1);
	}

	let fname = std::path::Path::new(&args[1]);
	let file = fs::File::open(&fname).unwrap();

	let mut archive = zip::ZipArchive::new(file).unwrap();

	let title = "    ___     _               _   __    _   _____ 
   /   \\___| |__  _   _  __| | /__\\  /_\\ /__   \\
  / /\\ / _ \\ '_ \\| | | |/ _` |/ \\// //_\\\\  / /\\/
 / /_//  __/ | | | |_| | (_| / _  \\/  _  \\/ /   
/___,' \\___|_| |_|\\__, |\\__,_\\/ \\_/\\_/ \\_/\\/    
                  |___/                         ";

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

	println!("{}", title);
	println!("by EliTheCoder\n");

	let mut nasty_files: HashMap<String, Vec<String>> = HashMap::new();

	println!("Scanning {} for RATs", fname.file_name().unwrap().to_str().unwrap());

	let mut pb = ProgressBar::new(archive.len().try_into().unwrap());

	for i in 0..archive.len() {
		let mut file = archive.by_index(i).unwrap();
		let mut bytes = Vec::new();
		file.read_to_end(&mut bytes).unwrap();

		for word in target_list {
			for i in bytes.windows(word.len()) {
				if i == word.as_bytes() {
					// push the file name to nasty_files
					let entry = nasty_files.entry(file.enclosed_name().unwrap().to_str().unwrap().to_owned()).or_insert(Vec::new());
					entry.push(word.to_string());
					break;
				}
			}
		}
	}
	pb.finish_println("\n");
	println!("{} file(s) found containing suspicious items", nasty_files.len());
	for (file, words) in nasty_files.iter().sorted() {
		println!("{} {:?}", file, words);
	}
	println!();
	println!("Press enter to exit");

	io::stdin().read_line(&mut String::new()).unwrap();

}
