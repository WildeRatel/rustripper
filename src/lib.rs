use std::io::Error;
use std::fs;
use crossterm::event::{poll, read, Event, KeyCode};
use std::time::Duration;

//Opens a file into a String when possible, otherwise it returns an error.
pub fn open_file(file_path: &String) -> Result<String, Error> {
    let contents = fs::read_to_string(file_path)?;

    Ok(contents)
}

//Displays lines in a file from a point to a point. Will error with nah if the lines are out of index.
pub fn display_contents(contents: &String, scroll_lines_from: u8, scroll_lines_to: u8) -> String {
    let mut content_vec: Vec<String> = Vec::new();
    for i in contents.split("\n") {
        content_vec.push(i.into());
    }
    if (content_vec.len() > (scroll_lines_to - scroll_lines_from) as usize) && (content_vec.len() >= scroll_lines_to as usize) {
        let mut page: String = String::new();

        for i in (scroll_lines_from - 1)..scroll_lines_to {
            page.push_str(&content_vec[i as usize]);
            page.push('\n');
        }

        page
    } else {
        String::from("Nah")
    }
}

//Gets the amount of lines in a file.
pub fn get_lines(contents: &String) -> u16 {
    let mut line_count: u16 = 0;

    contents.split('\n').for_each(|_| {
        line_count += 1;
    });
    line_count
}

//Function to get user input. Uses crossterm.
pub fn get_user_input() -> Result<String, Error> {
    loop {
        if poll(Duration::from_millis(100))? {
            if let Event::Key(key_event) = read()? {
                match key_event.code {
                    KeyCode::Char('q') => { 
                        return Ok(String::from("q"));
                    },
                    KeyCode::Char('i') => {
                        return Ok(String::from("i"));
                    },
                    KeyCode::Char('s') => {
                        return Ok(String::from("s"));
                    },
                    _ => println!("Other key pressed!")
                }
            }
        }
    } 
}