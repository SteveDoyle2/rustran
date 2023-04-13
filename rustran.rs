// Rust Tutorial 101
// mut: flags a variable as mutable
//----------------------------------------------
// This is a comment, and is ignored by the compiler.
// You can test this code by clicking the "Run" button over there ->
// or if you prefer to use your keyboard, you can use the "Ctrl + Enter"
// shortcut.

// This code is editable, feel free to hack it!
// You can always return to the original code by clicking the "Reset" button ->

// This is the main function.

//use std::io::Read;
use std::fs::File;
use std::io::{ self, BufRead, BufReader };


fn read_lines(filename: String) -> io::Lines<BufReader<File>> {
    // Open the file in read-only mode.
   //let mut file = std::fs::File::open(filename).unwrap();

   //let mut contents = String::new();
   //file.read_to_string(&mut contents).unwrap();
   //print!("{}", contents);
   
   //------------------------
   let file = File::open(filename).unwrap();
   // TODO: read includes
   // TODO: split into system, executive, case, bulk lines


   // Read the file line by line, and return an iterator of the lines of the file.
   return io::BufReader::new(file).lines();
}


fn main() {
    // Statements here are executed when the compiled binary is called.
    let lines = read_lines("./rod.bdf".to_string());
    println!("back to code");
    let mut i = 0;
    for line in lines {
        println!("{}: {}", i, line.unwrap());
        i += 1;
    }
    
    // Print text to the console.
    println!("Hello World!");
}
