use std::io::Error;
use std::fs;

pub fn open_file(file_path: &String) -> Result<String, Error> {
    let contents = fs::read_to_string(file_path)?;

    Ok(contents)
}

pub fn display_contents(contents: &String, scroll_lines_from: u8, scroll_lines_to: u8) -> String {
    let content_split = contents.split('\n');
    let lines_to_scroll: u8 = (scroll_lines_to) - (scroll_lines_from);

    let content_vec = content_split.collect::<Vec<&str>>();

    if content_vec.len() >= lines_to_scroll as usize {
        let mut content: String = String::new();
        for i in (scroll_lines_from - 1) as usize..=(scroll_lines_to - 1) as usize {
            let use_line = content_vec[i].to_string();
            content.push_str(&use_line);
            println!("{content}");
        }
        content.to_string()
    }
    else {
        String::from("Nah")
    }
}