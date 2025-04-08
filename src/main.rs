use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let dir = if args.len() > 1 {
        &args[1]
    } else {
        "."
    };
  
   if let Ok(entries) = fs::read_dir(dir) {
    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_dir() {
                println!("Dir: {:?}", path);
            } else {
                println!("File: {:?}", path);
            }
        }
    }

   } else {
    println!("Unable to read directory")
   }

}
