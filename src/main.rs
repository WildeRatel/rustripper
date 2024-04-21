use std::io;

fn main() {
    let mut user = String::new();
    io::stdin().read_line(&mut user).expect("Failed to read user input!");
    user = user.trim().to_string();

    if let Err(e) = ripper::open_file(&user) {
        eprintln!("Error: {e}")
    }
    else {
        let contents: String = ripper::open_file(&user).unwrap();
        let page: String = ripper::display_contents(&contents, 1, 4);

        if page != String::from("Nah") {
            println!("{page}");
        }
        else {
            eprint!("Nah");
        }

        let file_lines: u16 = ripper::get_lines(&contents);
        println!("Lines: {file_lines}");
    }
}