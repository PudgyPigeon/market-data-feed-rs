use std::env;

fn main() {
    let args = env::args().skip(1);
    println!("Starting the program with args: {:?}", args);

    let mut _reorder = false;
    let mut _input_path = None;

    for arg in args {
        match arg.as_str() {
            "-r" => _reorder = true,
            path if !path.starts_with("-") => _input_path = Some(path.to_string()),
            _ => eprintln!("Warning! Unknown argument '{}'", arg),
        }
    }
    println!("Done with everything!")
}
