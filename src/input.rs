use std::fs::File;
use std::io;

// gets user input, stages selectors for input
// @todo: make selector trait
pub struct Querier {}

impl Querier {
  fn new() -> Querier {
    Querier {}
  }

  // keeps asking for input until timeout
  // @todo: @urgent: cleanup
  // @refactor: name imply more meaning 
  // that you are waiting for query
  fn prompt<'a, F>(
    &self,
    // @refactor: trait or 'override'
    selection: &SelectionPicker,
    timeout: usize,
    get_input: F,
    // @todo: make timeout error for second result arg
  ) -> Result<String, String>
  where
    // input generating closure, 
    // normally formatted from terminal
    F: Fn() -> &'a String,
  {
    // result could timeout before matched
    let mut result = None;
    for x in 0..timeout {
      // cant own more than once, with closure must clone
      result = selection.select(get_input().clone());
    }
    result.ok_or(String::from("Timeout Error"))
  }

  // querier.prompt(menu, 10, |line| input);
}

// file grab commands for FileCommencer
enum FileGrab {
  New,
  Existing,
}

// returns created or already existing files
pub struct FileCommencer {
  // @todo: keep directory position state
}

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

// goes into a holder to represent a selection
pub struct SelectionPicker<'a> {
  // a tree of choices
  options: &'a [&'a str],
  // @suggestion: add field for displaying a label
}

impl<'a> SelectionPicker<'a> {
  fn new(options: &'a [&'a str]) -> SelectionPicker<'a> {
    SelectionPicker { options }
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
      .and_then(|v| Some(String::from(*v)))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn select_picker_option_create() {
    let input = String::from("create");
    let options = ["create", "edit"];
    let menu = SelectionPicker::new(&options);
    let selected = menu.select(input);

    match selected {
      Some(v) => assert_eq!("create", v),
      None => panic!("selected is none"),
    };
  }

  #[test]
  fn keep_prompting_input() {
    let input = String::from("create");
    let options = ["create", "edit"];
    let menu = SelectionPicker::new(&options);

    let querier = Querier::new();
    let result = querier.prompt(&menu, 10, || &input);
    assert!(result.is_ok());

    /*
      let result = terminal.prompt(menu, 10, |line| {
        io::stdin().read_line(line).expect("Failed to read line")
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
