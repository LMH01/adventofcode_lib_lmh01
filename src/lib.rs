use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
    path::Path,
};

/// Reads the input file into a vector containing a new entry per line and returns it.
/// 
/// If the file could not be opened an error is returned.
/// The end of line character is not writtten in the entry.
///
/// # Arguments
///
/// * `file_name` - A string slice containing the file name
///
/// # Examples
///
/// ```
/// use adventofcode_lmh01_lib::read_file;
///
/// let vec = match read_file("test_file.txt") {
///     Ok(ok) => ok,
///     Err(err) => {
///         println!("Unable to read file: {}", err);
///         std::process::exit(0);
///     }
/// };
///
/// ```
pub fn read_file(file_name: &str) -> Result<Vec<String>, Error> {
    let mut content: Vec<String> = Vec::new();
    let path = Path::new(file_name);
    let file = File::open(path);
    let file = match file {
        Ok(file) => file,
        Err(error) => return Err(error),
    };
    let mut buffered_reader = BufReader::new(file);
    let mut current_line: String = String::new();
    while buffered_reader.read_line(&mut current_line).unwrap_or(0) != 0 {
        content.push(current_line.trim().to_string());
        current_line = String::new();
    }
    Ok(content)
}

#[cfg(test)]
mod tests {
    use crate::read_file;

    #[test]
    fn read_file_() {
        let content = read_file("test_file.txt");
        assert_eq!(content.unwrap_or(Vec::new()).len(), 10);
        let content = read_file("does_not_exist.txt");
        assert!(content.is_err());
    }
}
