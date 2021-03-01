use goblin::Object;
use walkdir::{WalkDir};
use std::collections::HashMap;
use std::fs;


fn main() {
    let mut raw_filenames = HashMap::new();
    let mut filt_filenames = HashMap::new();
    for entry in WalkDir::new(".")
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| !e.file_type().is_dir()) {
        let f_name = String::from(entry.file_name().to_string_lossy());
        let counter = raw_filenames.entry(f_name.clone()).or_insert(0);
        *counter += 1;
        if (*counter == 1) && (f_name.ends_with(".dll") || f_name.ends_with(".pyd") || f_name.ends_with(".exe")){
            println!("{}", f_name);
			filt_filenames.entry(f_name.clone()).or_insert(0);
			let file = fs::read(&entry.path().to_owned()).unwrap();
			let win_pe = match Object::parse(&file).unwrap() {
				Object::PE(pe) => pe,
				_ => {
					println!("Unhandle bin type");
					return;
				}
			};
			let import_data = match win_pe.import_data {
				None => vec![],
				Some(v) => v.import_data,
			};
			for cur_import in &import_data {
				let counter = raw_filenames.entry(cur_import.name.to_owned()).or_insert(0);
				*counter += 1;
				if *counter == 1 {
					println!("{:?}",cur_import.name);
					filt_filenames.entry(cur_import.name.to_owned()).or_insert(0);
				}
				}
			}
		}

}
