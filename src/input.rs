// @suggestion: options into a more generic 'structure'
pub struct SelectionPicker<'a> {
  options: &'a [&'a str],
}

impl<'a> SelectionPicker<'a> {

  // @todo: this instancation goes into a box in control of templating input data
  fn new(options: &'a [&'a str]) -> SelectionPicker<'a> {
    SelectionPicker { options }
  }

  // tries to match with one of option field
  fn select<'b>(&self, input: Vec<String>) -> Option<String> {
    // first argument is query
    // @refactor: grabber of query from input to check if it is some.
    let option = &input[1];
    // find matching on option seld field
    match self.options.iter().find(|x| **x == option) {
      // 'unwrap' with non referencing string object
      Some(v) => Some(String::from(*v)),
      None => None
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn select_menu_option_create() {
    let options = ["create", "new", "edit"];
    let menu = SelectionPicker::new(&options);
    let input = vec![String::from("texidor"), String::from("create")];
    let selected = menu.select(input);

    assert!(!selected.is_none());
    match selected {
      Some(v) => assert_eq!("create", v),
      None => println!("Error"),
    };
  }
}
