/*
Todo:
Write a working search function.
*/

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
        let contents: String = ripper::open_file(&user).unwrap();

        loop {
            ripper::term_clear();
            println!("Controls: (q)uit | (i)nfo | (s)earch | (d)isplay lines");

            //User input
            if let Err(key_error) = ripper::get_user_input() {
                eprintln!("Error: {key_error}");
            } else {
                let key_pressed = ripper::get_user_input().unwrap();

                match key_pressed {
                    ripper::UserInputs::D => ripper::display(&contents),
                    ripper::UserInputs::Q => {
                        println!("Quit!");
                        break;
                    }
                    ripper::UserInputs::I => {
                        if let Err(e) = ripper::get_info(&user, &contents) {
                            eprintln!("Error: {e}")
                        } else {
                            ripper::get_info(&user, &contents).unwrap();
                            ripper::dummy_wait();
                            ripper::term_clear(); //Dunno why i have to clear the terminal twice, but it works so whatever.
                        }
                    }
                    ripper::UserInputs::S => {
                        let test_vec: Vec<String> = ripper::get_search(&contents);
                    }
                }
            }
        }
    }
}
