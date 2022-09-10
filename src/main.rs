use path_helper::read_paths;

fn main() {
    match read_paths() {
        Err(e) => eprintln!("{:?}", e),
        Ok(cmd) => println!("{}", cmd)
    }
}
