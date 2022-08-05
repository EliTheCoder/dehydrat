use std::{fs, io::{Read, self}};
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

	println!("{}", title);
	println!("by EliTheCoder\n");

	let mut nasty_files: Vec<String> = Vec::new();

	println!("Scanning {} for RATs", fname.file_name().unwrap().to_str().unwrap());

	let mut pb = ProgressBar::new(archive.len().try_into().unwrap());

	for i in 0..archive.len() {
		let mut file = archive.by_index(i).unwrap();
		let mut bytes = Vec::new();
		file.read_to_end(&mut bytes).unwrap();

		// check if bytes contains string 110432
		for i in bytes.windows(6) {
			if i == b"110432" {

				// push the file name to nasty_files
				nasty_files.push(file.enclosed_name().unwrap().to_str().unwrap().to_owned());
				break;

			}
			pb.inc();
		}

	}
	pb.finish_println("\n");
	println!("{} file(s) found containing RATs", nasty_files.len());
	println!("{}", nasty_files.join("\n"));
	println!();
	println!("Press enter to exit");

	io::stdin().read_line(&mut String::new()).unwrap();

}
