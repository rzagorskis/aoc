use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// pub fn read_lines_from_input_file(file_name: Option<&str>) -> io::Result<Lines<BufReader<File>>> {
//     let this_file_path = Path::new(file!());

//     let input_file_relative = &format!(
//         "{}{}",
//         this_file_path.parent().unwrap().display(),
//         file_name.unwrap_or("/input.txt")
//     );
//     let input_file_path = Path::new(input_file_relative);

//     return read_lines(input_file_path);
// }