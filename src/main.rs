use std::io;

fn main() {
    let mut user = String::new();
    io::stdin().read_line(&mut user).expect("Failed to read user input!");
    user = user.trim().to_string();
    
    //If Rust lets an error be equal to the open file function, handle it as one.
    if let Err(e) = ripper::open_file(&user) {
        eprintln!("Error: {e}")
    }

    // Most of the code will be going in here.
    else {
        let contents: String = ripper::open_file(&user).unwrap();
        let page: String = ripper::display_contents(&contents, 1, 4);

        //I'm still using Nah as an error code, should probably go and fix that later. Maybe, maybe not, i dunno.
        if page != String::from("Nah") {
            println!("{page}");
            println!("Controls: (q)uit | (i)nfo | (s)earch");
        }
        else {
            eprint!("Nah");
        }

        //Just testing the get_lines function here.
        let file_lines: u16 = ripper::get_lines(&contents);
        println!("Lines: {file_lines}");

        //Testing user input
        if let Err(key_error) = ripper::get_user_input() {
            eprintln!("Error: {key_error}");
        }
        else {
            let key_pressed = ripper::get_user_input().unwrap();
            println!("{key_pressed}");
        }
        
    }
}