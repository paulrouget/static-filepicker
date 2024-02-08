use rfd::{FileDialog};
use clap::{App, Arg};
use homedir::get_my_home;
use std::path::Path;

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
  let mut directory: String = matches.value_of("directory").unwrap().into();
  let title = matches.value_of("title").unwrap();
  let filename = matches.value_of("filename");

  if directory.starts_with("~") {
    let subdir = directory.strip_prefix("~").unwrap();
    let home = get_my_home().unwrap().unwrap().into_os_string().into_string().unwrap();
    directory = format!("{home}{subdir}");
  }

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

fn pick_folder(directory: String, title: &str) {
  match FileDialog::new().set_directory(directory).set_title(title).pick_folder() {
    Some(path) => print_result_folder(&path),
    None => std::process::exit(1),
  };
}

fn pick_file(directory: String, title: &str) {
  match FileDialog::new().set_directory(directory).set_title(title).pick_file() {
    Some(path) => print_result_file(&path),
    None => std::process::exit(1),
  }
}

fn save_file(directory: String, title: &str, filename: &str) {
  match FileDialog::new().set_directory(directory).set_title(title).set_file_name(filename).save_file() {
    Some(path) => print_result_file(&path),
    None => std::process::exit(1),
  }
}

fn print_result_file(path: &Path) {
  let dir = path.parent().unwrap();
  let filename = path.file_name().unwrap().to_str().unwrap();
  println!("{}", path.display());
  println!("{}", dir.display());
  println!("{}", filename);
}

fn print_result_folder(dir: &Path) {
  println!("{}", dir.display());
}
