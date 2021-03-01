use goblin::Object;
use std::collections::HashMap;
use std::fs;
use walkdir::WalkDir;

fn main() {
    let mut raw_filenames = HashMap::new();
    let mut filt_filenames = HashMap::new();
    //we must first find all dll,pyds, and executables in directory to get list of everything we already have in install folder
    for entry in WalkDir::new(".")
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir())
    {
        let f_name = String::from(entry.file_name().to_string_lossy());
        let path = String::from(entry.path().to_string_lossy());
        let counter = raw_filenames.entry(f_name.clone()).or_insert(0);
        *counter += 1;
        if (*counter == 1)
            && (f_name.ends_with(".dll") || f_name.ends_with(".pyd") || f_name.ends_with(".exe"))
            && (f_name != "python3.dll")
        {
            filt_filenames.entry(f_name.clone()).or_insert(path.clone());
        }
    }
    //Now that we have a list of present dlls can can loop over and find what imports are missing
    for path in filt_filenames.clone().values() {
        let file = fs::read(path).unwrap();
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
            let counter = raw_filenames
                .entry(cur_import.name.to_owned().clone())
                .or_insert(0);
            *counter += 1;
            if *counter == 1 {
                filt_filenames
                    .entry(cur_import.name.to_owned().clone())
                    .or_insert("Not in folder".to_string());
            }
        }
    }
    for x in filt_filenames.iter() {
        println!("{:?}", x);
    }
}
