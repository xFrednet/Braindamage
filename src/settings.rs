use std::fs::File;
use std::io::Read;

#[derive(Debug)]
pub struct Settings {
    pub src: String,
    pub dump_mem: bool
}

impl Settings {
    pub fn parse_args<T>(mut args: T) -> Option<Settings>
        where T: Iterator<Item=String>
    {
        let mut has_src = false;
        let mut settings = Self::init_default();

        loop {
            let value = args.next();
            if value.is_none() {
                break;
            }
            let value = value.unwrap().trim().to_lowercase();

            match value.as_str() {
                "-f" | "--file" => {
                    let file = args.next();
                    if file.is_none() {
                        println!("Please provide a file for the {} argument. For example: -f \"hello_world.bd\"", value);
                        break;
                    }

                    let src = load_source_file(file.unwrap());
                    if src.is_some() {
                        has_src = true;
                        settings.src = src.unwrap();
                    }
                },
                "-s" | "--src" => {
                    let src = args.next();
                    if src.is_none() {
                        println!("Please provide a string for the {} argument. For example: -s \"+[.+]\"", value);
                        break;
                    }

                    has_src = true;
                    settings.src = src.unwrap();
                },
                "-d" | "--dump" => {
                    settings.dump_mem = true;
                }
                x => println!("Unknown arg: {}", x)
            }
        }

        if has_src {
            Some(settings)
        } else {
            println!("Braindamage need source code to do something");
            None
        }
    }

    fn init_default() -> Settings {
        Settings {
            src: String::default(),
            dump_mem: false
        }
    }

    pub fn get_src(&self) -> String {
        self.src.clone()
    }
}

fn load_source_file(file_name: String) -> Option<String> {
    let file = File::open(file_name.clone());
    if file.is_err() {
        println!("Unable to load the file: \"{}\"", file_name);
        return None;
    }

    let mut file = file.unwrap();
    let mut src = String::default();

    if file.read_to_string(&mut src).is_err() {
        println!("Unable to read the file: {}", file_name);
        return None;
    }

    return Some(src);
}
