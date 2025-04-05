use std::path::Path;

#[derive(Debug)]
pub struct FilePathInfo {
    pub input_file_name: String,
    pub output_file_path: String,
    pub extension: String,
    pub parent_dir: String,
}

pub struct Split {
    pub before: String,
    pub after: String,
}
impl FilePathInfo {
    pub fn new(
        file_path: &str,
        output_file_name: &str,
        output_file_extension: Option<String>,
    ) -> FilePathInfo {
        let extension = if file_path.contains(".") {
            file_path.split('.').last().unwrap()
        } else {
            ""
        };
        let input_file_name = Self::split_the_difference(file_path, "/").after;

        let parent_dir_before = Self::split_the_difference(file_path, "/").before;

        let parent_dir = Self::split_the_difference(&parent_dir_before, "/").before;

        let output_file_path = match output_file_extension {
            Some(ext) => format!("{}/{}.{}", parent_dir, output_file_name, ext),
            None => format!("{}/{}.{}", parent_dir, output_file_name, extension),
        };

        Self {
            input_file_name,
            output_file_path,
            extension: extension.to_string(),
            parent_dir,
        }
    }

    pub fn split_the_difference(some_string: &str, delimiter: &str) -> Split {
        if some_string.contains(delimiter) {
            let (before, after) = some_string.rsplit_once(delimiter).unwrap();
            println!("Before: {}", before);
            println!("After: {}", after);
            return Split {
                before: before.to_string(),
                after: after.to_string(),
            };
        } else {
            println!("❌ String does not contain delimiter: {}", delimiter);
            return Split {
                before: "nothing to split".to_string(),
                after: "x".to_string(),
            };
        }
    }
}
