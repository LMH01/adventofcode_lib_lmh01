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
/// let vec = read_file("test_file.txt").unwrap();
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

/// Transforms a string vector
///
/// Each first element of a string is used to form a new vector entry.\
/// For this function to work properly you should only use it with vectors where each element has
/// the same length. Otherwise it can lead to undefined behaviour.
///
/// # Arguments
///
/// * 'vec' - The vector that should be transformed
///
/// # Examples
/// 
/// Lets say that vec_old contains the following elements:\
/// Cat\
/// Dog\
/// Car\
///
/// The new vector 'vec' will contain the following elements:\
/// CDC\
/// aoa\
/// tgr\
/// ```
/// use adventofcode_lmh01_lib::transform_vec;
///
/// let vec_old = Vec::new();
/// let vec = transform_vec(vec_old);
/// ``` 
///
pub fn transform_vec(vec: Vec<String>) -> Vec<String> {
    let mut output: Vec<String> = vec![String::new(); vec.get(0).unwrap_or(&String::new()).len()];
    for line in &vec {
        for (index, character) in line.chars().enumerate() {
            output
                .get_mut(index)
                .unwrap_or(&mut String::new())
                .push(character);
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use crate::{read_file, transform_vec};

    #[test]
    fn read_file_() {
        let content = read_file("test_file.txt");
        assert_eq!(content.unwrap_or(Vec::new()).len(), 10);
        let content = read_file("does_not_exist.txt");
        assert!(content.is_err());
    }

    #[test]
    fn transform_vec_() {
        let content = transform_vec(read_file("test_file.txt").unwrap());
        assert_eq!(content.len(), 2);
    }
}
