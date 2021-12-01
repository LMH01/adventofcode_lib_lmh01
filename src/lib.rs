use std::{path::Path, fs::File, io::{BufRead, BufReader}};

/// Reads the input file into a vector containing a new entry per line
///
/// The end of line character is not writtten in the entry
///
/// # Arguments
///
/// * `file_name` - A string slice containing the file name
///
/// # Panics
///
/// Panics when the file does not exist
///
/// # Examples
///
/// ```
/// use adventofcode_lmh01_lib::read_file;
/// let content = read_file("test_file.txt");
/// ```
pub fn read_file(file_name: &str) -> Vec<String> {
    let mut content: Vec<String> = Vec::new();
    let path = Path::new(file_name);
    let file = File::open(path);
    let file = match file {
        Ok(file) => file,
        Err(error) => panic!("Problem while opening file: {}", error),
    };
    let mut buffered_reader = BufReader::new(file);
    let mut current_line: String = String::new();
    while buffered_reader.read_line(&mut current_line).unwrap_or(0) != 0 {
        content.push(current_line.trim().to_string());
        current_line = String::new();
    }
    return content;
}

#[cfg(test)]
mod tests {
    use crate::read_file;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn read_file_() {
        let content = read_file("test_file.txt");
        assert_eq!(content.len(), 10);
    }

    #[test]
    #[should_panic]
    fn read_file_panic() {
        let _content = read_file("does_not_exist.txt");
    }
}
