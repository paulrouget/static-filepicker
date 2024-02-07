use rfd::{FileDialog};
use clap::{App, Arg};

fn main() {
  // Define command-line interface using clap
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

  // Extract values of the provided arguments
  let action = matches.value_of("action").unwrap();
  let directory = matches.value_of("directory").unwrap();
  let title = matches.value_of("title").unwrap();
  let filename = matches.value_of("filename");

  // Perform actions based on user input
  match action {
    "pick_folder" => pick_folder(directory, title),
    "pick_file" => pick_file(directory, title),
    "save_file" => {
      if let Some(filename) = filename {
        save_file(directory, title, filename)
      } else {
        eprintln!("Filename is required for action save_file");
      }
    }
    _ => eprintln!("Invalid action specified"),
  }
  std::process::exit(1);
}

fn pick_folder(directory: &str, title: &str) {
  let res = FileDialog::new()
    .set_directory(directory)
    .set_title(title)
    .pick_folder();
  println!("{}", res.unwrap().display());
  std::process::exit(0);
}

fn pick_file(directory: &str, title: &str) {
  let res = FileDialog::new()
    .set_directory(directory)
    .set_title(title)
    .pick_file();
  println!("{}", res.unwrap().display());
  std::process::exit(0);
}

fn save_file(directory: &str, title: &str, filename: &str) {
  let res = FileDialog::new()
    .set_directory(directory)
    .set_title(title)
    .set_file_name(filename)
    .save_file();
  println!("{}", res.unwrap().display());
  std::process::exit(0);
}
