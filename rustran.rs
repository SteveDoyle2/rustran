// This is a comment, and is ignored by the compiler.
// You can test this code by clicking the "Run" button over there ->
// or if you prefer to use your keyboard, you can use the "Ctrl + Enter"
// shortcut.

// This code is editable, feel free to hack it!
// You can always return to the original code by clicking the "Reset" button ->

// This is the main function.

use std::io::Read;

fn main() {
    // Statements here are executed when the compiled binary is called.

    // Print text to the console.
    println!("Hello World!");

   let mut file = std::fs::File::open("rod.bdf").unwrap();
   let mut contents = String::new();
   file.read_to_string(&mut contents).unwrap();
   print!("{}", contents);
}
