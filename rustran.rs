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

    // goal is to create some empty lists of strings
    // that we will later append to
    let mut system_lines = vec!["Paris", "New York"];
    //let mut system_lines: Vec<String> = vec![String::new(); 126];
    //let mut system_lines: [&str; 0] = [];
    //let mut executive_lines: [&str; 0] = [];
    //let mut case_lines: [&str; 0] = [];
    //let mut bulk_lines: [&str; 120] = [];
    let mut bulk_lines: Vec<String> = vec![];  // the fork?


    let cend: String = String::from("CEND");
    
    system_lines.push("CAT");
    let mut i = 0;
    let mut ideck = 0;
    for line in lines {
        let line0 = line.unwrap();
        //line.what_is_this;  // get type of line (String)
        
        // TODO: how do I do an if compare two strings???
        //if line.compare("CEND") { no
        //if line.cmp("CEND") { // no
        //if line.cmp(cend) {  // no
        //   ideck += 1;
        //}
        //if line == "CEND" {
        //   println!("**CEND {} {}: {}", ideck, i, line0);
        //}
        //if line.eq("CEND") {
        if ideck != 0 {
           ideck *= -1;
        }
        if i < 9 {
           println!("**{} {}: {}", ideck, i, line0);
        } else {
           // unwrap can only be done once.  It's readline()?
           //println!("{} {}: {}", ideck, i, line.unwrap());
           //println!("{}b {}: {}", ideck, i, line.unwrap());
           //bulk_lines.push(line.to_string());
           bulk_lines.push(line0);
        }
        i += 1;
    }
    
    for bline in bulk_lines {
        // TODO: filter the comments
        //let bline = line.as_bytes();
        //if bline.eq("$") {
        //    println!("$$$");
        //}
        //if bline(0).eq("$") {
        //    println!("comment: {}", bline);
        //}
        println!("bulk: {}", bline);
    }

    // Print text to the console.
    println!("Hello World!");
}
