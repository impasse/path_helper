use clap::ArgGroup;
use clap::Parser;

use path_helper::mode::Mode;
use path_helper::read_paths;

#[derive(Parser, Debug)]
#[clap(author, about, long_about = None)]
#[clap(group(
  ArgGroup::new("mode")
      .required(true)
      .args(&["s", "c"]),
))]
pub struct Args {
  /// csh
   #[clap(short, action)]
   c: bool,

   /// bash
   #[clap(short, action)]
   s: bool,
}

fn main() {
  let args = Args::parse();
  let mode = if args.c {
    Mode::CSH
  } else {
    Mode::BASH
  };
  match read_paths(&mode) {
      Err(e) => {
        eprintln!("{:?}", e);
        std::process::exit(1)
      },
      Ok(cmd) => println!("{}", cmd),
  }
}
