use std::fs::File;

enum FileGrab {
  New,
  Existing
}

// returns created or already existing files
pub struct FileCommencer {
  // @todo: keep directory position state
}

impl FileCommencer {
  fn new() -> FileCommencer {
    FileCommencer { }
  }

  fn create(&self, name: String) -> Result<File, std::io::Error> {
    File::create(name)
  }

  fn get(&self, name: String) -> Result<File, std::io::Error> {
    File::open(name)
  }

  fn grab (&self, name: String, grab: FileGrab) -> Result<File, std::io::Error> {
    // returns moved file object
    match grab {
      New => self.create(name),
      Existing => self.get(name)
    }
  }
}

// goes into a holder to represent a selection
pub struct SelectionPicker<'a> {
  // a tree of choices
  options: &'a [&'a str],
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
  fn select_menu_option_create() {
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
  fn create_new_file() {
    let file_name = String::from("hello.txt");
    let file_commencer = FileCommencer::new();
    let file = file_commencer.grab(file_name, FileGrab::New);
    assert!(file.is_ok());
  }
}
