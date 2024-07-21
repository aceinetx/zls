use std::fs;

struct ZLSSettings {
    hide_dotfiles: bool
}

fn check_paths(paths: &Vec<String>) -> Result<bool, String> {
    for path in paths {
        let md = fs::metadata(path);
        let dash_flag = get_file_name_from_path(path).starts_with('-');
        if md.is_err() && !dash_flag {
            let errmsg = std::format!("Invalid path: {}", path);
            return Err(errmsg);
        }
    }
    Ok(true)
}

fn type_of<T>(_: T) -> &'static str {
    std::any::type_name::<T>()
}

fn get_file_name_from_path(path: &String) -> String{
   let mut new_path_reversed: String = Default::default();
   for char in path.chars().rev() {
       if ['/', '\\'].contains(&char) {
           break
       }
       new_path_reversed.push(char);
   }
   new_path_reversed.chars().rev().collect::<String>()
}

fn print_file_info(path: &String, settings: &ZLSSettings){
    let md = fs::metadata(path);
    let file_name = get_file_name_from_path(path);
    if file_name.starts_with('.') && settings.hide_dotfiles || file_name.starts_with('-') {
        return
    }
    let metadata = match md {
        Ok(_md) => _md,
        Err(_error) => {
            //println!("\x1b[38;5;203mzls: failed to get metadata (faulty path: {}): {:?} (maybe a broken symlink?)", path, error);
            println!("\x1b[38;5;47m\x1b[1m{}\x1b[0m Unknown (maybe a broken symlink?)", file_name);
            //std::process::exit(1);
            return
        }
    };
    let permissions = metadata.permissions();
    let mut permissions_str: String = Default::default();
    if permissions.readonly() {
        permissions_str.push_str("readonly protect  ");
    } else {
        permissions_str.push_str("write+read protect");
    }

    let mut file_size_str: String;
    let file_len = metadata.len();
    file_size_str = format!("{} B", file_len);
    if file_len > 8192 {
        file_size_str = format!("{} KiB", file_len/8192);
    }

    if file_len > 1049000 {
        file_size_str = format!("{} MiB", file_len/1049000);
    }

    if file_len > 8590000000 {
        file_size_str = format!("{} GiB", file_len/8590000000);
    }

    if file_len > 1100000000000 {
        file_size_str = format!("{} TiB", file_len/1100000000000);
    }

    if metadata.is_file() {
        println!("\x1b[38;5;47m\x1b[1m{}\x1b[0m \x1b[38;5;7m{} \x1b[38;5;57msize: {}",
            file_name, 
            permissions_str,
            file_size_str
        );
    } else {
        println!("\x1b[38;5;132m{}\x1b[0m folder", file_name);
    }
}

fn parse_args(settings: &mut ZLSSettings, args: &Vec<String>){
    for arg in args {
        if arg == "-l" {
            settings.hide_dotfiles = false;
        }
    }
}

fn main() {
    let mut args = std::env::args().collect::<Vec<String>>();
    let mut settings: ZLSSettings = ZLSSettings {hide_dotfiles: true};
    if args.len() < 2 {
        println!("\x1b[38;5;203mzls: no arguments passed");
        std::process::exit(1);
    }

    args.remove(0);
    let paths = &args;
    let paths_valid = check_paths(paths);
    if paths_valid.is_err() {
        println!("\x1b[38;5;203mzls: {} is Err(\"{}\")", type_of(&paths_valid), paths_valid.err().unwrap());
        std::process::exit(1);
    }

    parse_args(&mut settings, &args);

    for path in paths {
        let md = fs::metadata(path);
        let file_name = get_file_name_from_path(path);
        if file_name.starts_with('.') && settings.hide_dotfiles || file_name.starts_with('-') {
            continue
        }

        let metadata = match md {
            Ok(_md) => _md,
            Err(_error) => {
                //println!("\x1b[38;5;203mzls: failed to get metadata (faulty path: {}): {:?} (maybe a broken symlink?)", path, error);
                println!("\x1b[38;5;47m\x1b[1m{}\x1b[0m Unknown (maybe a broken symlink?)", file_name);
                //std::process::exit(1);
                return
            }
        };

        if metadata.is_file() {
            print_file_info(path, &settings);
        } else {
            let file_name = get_file_name_from_path(path);
            if file_name.starts_with('.') && settings.hide_dotfiles || file_name.starts_with('-') {
                continue;
            }

            let iterator = match fs::read_dir(path) {
                 Ok(x) => x,
                 Err(error) => {
                     println!("\x1b[38;5;203mzls: failed to iterate {}: {:?}", path, error);
                     std::process::exit(1);
                 }
            };
            for entry in iterator {
                let dir_entry = match entry {
                    Ok(x) => x,
                        Err(error) => {
                        println!("\x1b[38;5;203mzls: failed to unwrap while iterating through {}: {:?}", path, error);
                        std::process::exit(1);
                    }
                };
                let dir_entry_path = dir_entry.path().to_str().unwrap().to_string();
                let file_name = get_file_name_from_path(&dir_entry_path);
                if file_name.starts_with('.') && settings.hide_dotfiles || file_name.starts_with('-') {
                    continue
                }

                print_file_info(&dir_entry_path, &settings);
            }
        }
    }
}
