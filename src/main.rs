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
        println!("Controls: (q)uit | (i)nfo | (s)earch | (d)isplay lines");
        let contents: String = ripper::open_file(&user).unwrap();

        //User input
        if let Err(key_error) = ripper::get_user_input() {
            eprintln!("Error: {key_error}");
        } else {
            let key_pressed = ripper::get_user_input().unwrap();
            if key_pressed == String::from("d") {
                let mut line_from: String = String::new();
                let mut line_to: String = String::new();
                io::stdin()
                    .read_line(&mut line_from)
                    .expect("Failed to read line!");
                io::stdin()
                    .read_line(&mut line_to)
                    .expect("Failed to read line!");

                if let Err(parse_e) = line_from.parse::<u8>() {
                    println!("Failed to parse input: {parse_e}");
                } else {
                    if let Err(parse_e) = line_to.parse::<u8>() {
                        println!("Failed to parse input: {parse_e}");
                    } else {
                        let page: String = ripper::display_contents(
                            &contents,
                            line_from.parse::<u8>().unwrap(),
                            line_to.parse::<u8>().unwrap()
                        );

                        //I'm still using Nah as an error code, should probably go and fix that later. Maybe, maybe not, i dunno.
                        if page != String::from("Nah") {
                            println!("{page}");
                        } else {
                            eprint!("Nah");
                        }
                    }
                }
            }
        }

        //Just testing the get_lines function here.
        let file_lines: u16 = ripper::get_lines(&contents);
        println!("Lines: {file_lines}");
    }
}
