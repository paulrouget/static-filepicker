use rfd::{FileDialog};
use clap::{App, Arg};

fn main() {
  let matches = App::new("filepicker")
    .arg(
      Arg::with_name("action")
        .help("Specifies the action to perform")
        .possible_values(&["pick_folder", "pick_file", "save_file"])
        .required(true),
    )
    .arg(
      Arg::with_name("directory")
        .help("Specifies the directory")
        .takes_value(true)
        .required(true),
    )
    .arg(
      Arg::with_name("title")
        .help("Specifies the title")
        .takes_value(true)
        .required(true),
    )
    .arg(
      Arg::with_name("filename")
        .help("Specifies the filename")
        .takes_value(true),
    )
    .get_matches();

  let action = matches.value_of("action").unwrap();
  let directory = matches.value_of("directory").unwrap();
  let title = matches.value_of("title").unwrap();
  let filename = matches.value_of("filename");

  match action {
    "pick_folder" => pick_folder(directory, title),
    "pick_file" => pick_file(directory, title),
    "save_file" => {
      if let Some(filename) = filename {
        save_file(directory, title, filename)
      } else {
        eprintln!("Filename is required for action save_file");
        std::process::exit(1);
      }
    }
    _ => {
      eprintln!("Invalid action specified");
      std::process::exit(1);
    }
  }
}

fn pick_folder(directory: &str, title: &str) {
  match FileDialog::new().set_directory(directory).set_title(title).pick_folder() {
    Some(path) => println!("{}", path.display()),
    None => std::process::exit(1),
  };
}

fn pick_file(directory: &str, title: &str) {
  match FileDialog::new().set_directory(directory).set_title(title).pick_file() {
    Some(path) => println!("{}", path.display()),
    None => std::process::exit(1),
  }
}

fn save_file(directory: &str, title: &str, filename: &str) {
  match FileDialog::new().set_directory(directory).set_title(title).set_file_name(filename).save_file() {
    Some(path) => println!("{}", path.display()),
    None => std::process::exit(1),
  }
}
