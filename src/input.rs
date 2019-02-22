use std::fs::File;
use std::io;

pub struct Ostrich {
  querier: Querier,
  directory_peaker: DirectoryPeaker,
}

// body of the application
impl Ostrich {
  pub fn new() {
    let ostrich = Ostrich {
      querier: Querier::new(),
      directory_peaker: DirectoryPeaker::new(),
    };

    ostrich.file_menu();
  }

  // prompt the file menu
  fn file_menu(&self) {
    self
      .querier
      .prompt(&self.directory_peaker.menu, || self.querier.stdin_line());
  }
}

struct DirectoryPeaker {
  menu: SelectionPicker,
  file_namer: StaticWriter,
}

// before working with files, this is the control
// to select exisiting or create new file
impl<'a> DirectoryPeaker {
  pub fn new() -> DirectoryPeaker {
    DirectoryPeaker {
      menu: DirectoryPeaker::menu(),
      file_namer: DirectoryPeaker::file_namer(),
    }
  }

  // create or edit file menu
  fn menu() -> SelectionPicker {
    SelectionPicker::new(Box::new(["create", "edit"]), "Write 'create' or 'edit'")
  }

  // used both by new and edit file
  // to specify which file to target
  fn file_namer() -> StaticWriter {
    StaticWriter::new("Write filename")
  }
}

struct Querier {}

// gets user input, stages selectors for input
impl Querier {
  fn new() -> Querier {
    Querier {}
  }

  // reads one line from stdin
  // used as closure protocol for input
  pub fn stdin_line(&self) -> String {
    let mut input = String::new();
    io::stdin()
      .read_line(&mut input)
      .expect("Failed to read line");
    // trimming newline char at end
    input.trim().to_string()
  }

  // keeps asking for input until selected returns something
  pub fn prompt<'a, T, F>(&self, selection: &T, get_input: F) -> Result<String, String>
  where
    // promptable input selector
    T: Promptable,
    // input generating closure,
    // can be specified or use querier.stdin_line()
    F: Fn() -> String,
  {
    let mut result = None;
    while result.is_none() {
      // display then ask for input
      selection.describe();
      result = selection.select(get_input());
    }
    result.ok_or(String::from("Prompt Error"))
  }
}

// file grab commands for FileCommencer
enum FileGrab {
  New,
  Existing,
}

pub struct FileCommencer {
  // @todo: keep directory position state
}

// returns created or already existing files
impl FileCommencer {
  fn new() -> FileCommencer {
    FileCommencer {}
  }

  fn create(&self, name: String) -> Result<File, std::io::Error> {
    File::create(name)
  }

  fn get(&self, name: String) -> Result<File, std::io::Error> {
    File::open(name)
  }

  fn grab(&self, name: String, grab: FileGrab) -> Result<File, std::io::Error> {
    // returns moved file object
    match grab {
      New => self.create(name),
      Existing => self.get(name),
    }
  }
}

// prompted to screen and wait for user input,
// used with querier.prompt()
pub trait Promptable {
  // how to select output from option
  fn select(&self, option: String) -> Option<String>;
  // display description to be visible in a prompt
  fn describe(&self);
}

/*
* ------------------------------
*  ⬇️ Promptable selection inputs
* ------------------------------
*/

pub struct SelectionPicker {
  // a tree of choices
  options: Box<[&'static str]>,
  // label describing selection
  description: &'static str,
}

// match input with arbitrary length options
impl SelectionPicker {
  fn new(options: Box<[&'static str]>, description: &'static str) -> SelectionPicker {
    SelectionPicker {
      options,
      description,
    }
  }
}

impl Promptable for SelectionPicker {
  fn describe(&self) {
    println!("{}", self.description);
  }

  // tries to match with one of option field
  fn select(&self, option: String) -> Option<String> {
    // iterate try to find argument
    self
      .options
      .iter()
      // find matching on option self field
      .find(|v| **v == option)
      .and_then(|v| Some(v.to_string()))
  }
}

pub struct StaticWriter {
  // label describing selection
  description: &'static str,
}

// only description label and larger than 0 char input
impl StaticWriter {
  fn new(description: &'static str) -> StaticWriter {
    StaticWriter { description }
  }
}

impl Promptable for StaticWriter {
  fn describe(&self) {
    println!("{}", self.description);
  }

  // just checks if option input is not empty then
  // passes it back as accepted
  fn select(&self, option: String) -> Option<String> {
    if option.chars().count() > 0 {
      Some(option)
    } else {
      None
    }
  }
}

/*
* ------------------------------
*  ⬆️
* ------------------------------
*/

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn select_picker_selection_create() {
    let input = String::from("create");
    let menu = SelectionPicker::new(
      Box::new(["create", "edit"]),
      "Do you want to create or edit file?",
    );

    let selected = menu.select(input);
    match selected {
      Some(v) => assert_eq!("create", v),
      None => panic!("selected is none"),
    };
  }

  #[test]
  fn static_writer_input_filename() {
    let input = String::from("hello.txt");
    let text_box = StaticWriter::new("Please enter filename");
    let selected = text_box.select(input);

    match selected {
      Some(v) => assert_eq!("hello.txt", v),
      None => panic!("selected is none"),
    };
  }

  #[test]
  fn querier_keep_prompting_input() {
    let input = String::from("create");
    let menu = SelectionPicker::new(
      Box::new(["create", "edit"]),
      "Do you want to create or edit file?",
    );

    let querier = Querier::new();
    let result = querier.prompt(&menu, || querier.stdin_line());
    assert!(result.is_ok());
  }

  #[test]
  fn create_new_file() {
    let file_name = String::from("hello.txt");
    let file_commencer = FileCommencer::new();
    // @help: see how to remove file creation when testing
    let file = file_commencer.grab(file_name, FileGrab::New);
    assert!(file.is_ok());
  }
}
