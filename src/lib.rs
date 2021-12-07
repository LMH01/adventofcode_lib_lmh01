use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader, Error as IOError},
    ops::{Add, Sub},
    path::Path,
    str::FromStr,
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
pub fn read_file(file_name: &str) -> Result<Vec<String>, IOError> {
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

/// Transforms the string to a vector that contains the numbers
///
/// T indicates what type the vector should be
///
/// # Arguments
///
/// * `line` - A string slice that contains the numbers that should be transformed to a vector
///
/// # Returns
///
/// * `Result<Vec<i32>, ParseIntError>` - ParseIntError is returned when the string slice contains
/// chars that can not be parsed into integer
///
/// # Examples
///
/// ```
/// use adventofcode_lmh01_lib::get_draw_numbers;
///
/// let numbers = get_draw_numbers("0, 1, 2, 3").unwrap();
/// assert_eq!(vec![0, 1, 2, 3], numbers);
///
/// let numbers = get_draw_numbers("5 , 7 , 20 , 3  3  4 ").unwrap();
/// assert_eq!(vec![5, 7, 20, 334], numbers);
/// ```
///
/// Panics because unwrap is called and an error is returned:
///
/// ```should_panic
/// use adventofcode_lmh01_lib::get_draw_numbers;
///
/// let numbers = get_draw_numbers("a5, b6, c8, d203").unwrap();
/// assert_eq!(vec![5, 6, 8, 203], numbers);
/// ```
pub fn get_draw_numbers<T: Add<Output = T> + Sub<Output = T> + Ord + FromStr>(
    line: &str,
) -> Result<Vec<T>, <T as FromStr>::Err> {
    let mut drawn_numbers = Vec::new();
    let mut current_number: String = String::new();
    for char in line.chars() {
        match char {
            ',' => match current_number.parse::<T>() {
                Ok(ok) => {
                    drawn_numbers.push(ok);
                    current_number = String::new();
                }
                Err(err) => return Err(err),
            },
            ' ' => (),
            _ => current_number.push(char),
        }
    }
    match current_number.parse::<T>() {
        Ok(ok) => drawn_numbers.push(ok),
        Err(err) => return Err(err),
    }
    Ok(drawn_numbers)
}

/// Launches the part1 and part2 functions of a day and prints information into console
///
/// # Arguments
/// * `part1` - The function that contains the code for part1
/// * `part2` - The function that contains the code for part2
/// * `day` - An integer that signals what day the functions belong to
/// * `parts` - A tuple that contains what parts should be run
pub fn run_day(
    part1: fn() -> Result<(), Box<dyn Error>>,
    part2: fn() -> Result<(), Box<dyn Error>>,
    day: i32,
    parts: (bool, bool),
) -> Result<(), Box<dyn Error>> {
    println!("Running day {:2}...", day);
    if parts.0 {
        println!("--- Part 1 ---");
        part1()?;
    }
    if parts.1 {
        println!("--- Part 2 ---");
        part2()?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{get_draw_numbers, read_file, transform_vec};

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

    #[test]
    fn get_draw_numbers_() {
        assert_eq!(
            10,
            get_draw_numbers::<i32>("0,1,2,3,4,5,6,7,8,9")
                .unwrap()
                .len()
        );
        assert_eq!(
            10,
            get_draw_numbers::<i32>("0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 ")
                .unwrap()
                .len()
        );
        assert_eq!(
            10,
            get_draw_numbers::<u8>("0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 ")
                .unwrap()
                .len()
        );
        let numbers = get_draw_numbers("1, 2, 3, 4").unwrap();
        assert_eq!(vec![1, 2, 3, 4], numbers);
    }
}
