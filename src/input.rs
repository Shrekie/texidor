use std::fs::File;
use std::io;

struct DirectoryPeaker {
  menu: SelectionPicker,
  name_file: StaticWriter
}

// before editing files, this is the control 
// to select exisiting or create new file
impl<'a> DirectoryPeaker {
  pub fn new() -> DirectoryPeaker {
    DirectoryPeaker {
      menu: DirectoryPeaker::menu(),
      name_file: DirectoryPeaker::nameFile()
    }
  }

  // create or edit file menu
  fn menu() -> SelectionPicker {
    SelectionPicker::new(
      Box::new(["create", "edit"]),
      "Write 'create' or 'edit'",
    )
  }

  // used both by new and edit file
  // to specify which file to target
  fn nameFile() -> StaticWriter {
    StaticWriter::new(
      "Write filename"
    )
  }
}

struct Querier {}

// gets user input, stages selectors for input
impl Querier {
  fn new() -> Querier {
    Querier {}
  }

  // displays description and parses input from user
  fn input_format<T: Promptable>(&self, input: String, selection: &T) -> Option<String> {
    selection.describe();
    selection.select(input)
  }

  // keeps asking for input until selected
  // @refactor: name imply more meaning
  // that you are waiting for query
  pub fn prompt<'a, T, F>(&self, selection: &T, get_input: F) -> Result<String, String>
  where
    // promptable selector
    T: Promptable,
    // input generating closure,
    // normally formatted from terminal
    F: Fn() -> &'a String,
  {
    let mut result = None;
    while result.is_none() {
      // cant own more than once, with closure 'must' clone
      result = self.input_format(get_input().clone(), selection);
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

pub trait Promptable {
  // how to select output from option
  fn select(&self, option: String) -> Option<String>;
  // display description to be visible in a prompt
  fn describe(&self);
}

/*
* ------------------------------------------------------
* Promptable selection input component
* ------------------------------------------------------
* able to be prompted to screen and wait for user input,
* used with querier.prompt()
* 
*
*/

pub struct SelectionPicker {
  // a tree of choices
  options: Box<[&'static str]>,
  // label describing selection
  description: &'static str,
}

// goes into a holder to represent a selection
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
      // 'unwrap' to move string object
      .and_then(|v| Some(v.to_string()))
  }
}

pub struct StaticWriter {
  // label describing selection
  description: &'static str,
}

// only description label and > 0 text input
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
    // no characters is none
    if option.chars().count() > 0 {
      Some(option)
    } else {
      None
    }
  }
}

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
      Box::new(["create","edit"]),
      "Do you want to create or edit file?",
    );

    let querier = Querier::new();
    let result = querier.prompt(&menu, || &input);
    assert!(result.is_ok());
    
    /*
    let querier = Querier::new();
    let result = querier.prompt(&menu, || &io::stdin().read_line(line)
      .expect("Failed to read line"));
    });
    */
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
