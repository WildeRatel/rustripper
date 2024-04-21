use std::io;

fn main() {
    let mut user = String::new();
    io::stdin()
        .read_line(&mut user)
        .expect("Failed to read user input!");
    user = user.trim().to_string();

    //If Rust lets an error be equal to the open file function, handle it as one.
    if let Err(e) = ripper::open_file(&user) {
        eprintln!("Error: {e}")
    }
    // Most of the code will be going in here.
    else {
        loop {
            ripper::term_clear();
            println!("Controls: (q)uit | (i)nfo | (s)earch | (d)isplay lines");
            let contents: String = ripper::open_file(&user).unwrap();

            //User input
            if let Err(key_error) = ripper::get_user_input() {
                eprintln!("Error: {key_error}");
            } else {
                let key_pressed = ripper::get_user_input().unwrap();

                if key_pressed == String::from("d") {
                    ripper::display(&contents);
                }
                if key_pressed == String::from("q") {
                    println!("Quit!");
                    break;
                }
            }
            /*
            Just testing the get_lines function here.
            let file_lines: u16 = ripper::get_lines(&contents);
            println!("Lines: {file_lines}");
            */
        }
    }
}
