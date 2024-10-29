use crate::types::*;
use std::io;

pub(crate) mod types;

#[macro_export]
macro_rules! fail {
      ($($arg:tt)*) => {
          {
              eprint!($($arg)*);
              ::std::process::exit(1);
          }
      };
}

fn validate(item: ValidationItem) {
    let _ = item;
    todo!("validate the test case and use fail!() to fail the test case");
}

fn main() {
    let mut str_in = String::new();
    if let Err(err) = io::stdin().read_line(&mut str_in) {
        fail!("Failed to read input: {err}")
    }
    match serde_json::from_str(&str_in) {
        Ok(item) => validate(item),
        Err(err) => {
            fail!("Failed to deserialize input: {err}")
        }
    }
}
