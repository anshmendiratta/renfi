use anyhow::Result;
use std::{
    fs::{read_dir, DirEntry},
    path::{Path, PathBuf},
    process::Command,
};

pub fn get_possible_file_names(file_name: &str) -> Result<Vec<String>> {
    let assignments_directory = Path::new("/Users/anshmendiratta/Desktop/assignments/");
    let file_iter = read_dir(assignments_directory).unwrap();
    let folders: Vec<DirEntry> = file_iter
        .filter_map(|f| {
            let file = f.unwrap();
            let file_metadata = file.metadata().unwrap();
            if file_metadata.is_dir() {
                return Some(file);
            }

            None
        })
        .collect();

    let possible_names = {
        let mut result: Vec<String> = Vec::new();
        for name_prefix in folders {
            let name_prefix_path = name_prefix.path().into_os_string().into_string();
            assert!(name_prefix_path.is_ok());
            let name_prefix_path = name_prefix_path.unwrap();

            let last_slash = name_prefix_path
                .match_indices('/')
                .collect::<Vec<(usize, &str)>>()
                .last()
                .unwrap()
                .0;

            result.push(format!(
                "{}_{}",
                &name_prefix_path[last_slash + 1..],
                file_name
            ));
        }

        result
    };

    Ok(possible_names)
}

pub fn rename_file_in_dir(
    directory_of_file: PathBuf,
    file_to_rename: String,
    new_file_name: String,
) -> Result<()> {
    let directory_of_file = directory_of_file.as_os_str().to_str();
    let file_to_rename_path = format!("{}/{}", directory_of_file, file_to_rename);
    let new_file_path = format!("{}/{}", directory_of_file, new_file_name);

    let cmd_output = Command::new("mv")
        .args([file_to_rename_path, new_file_path])
        .status()
        .expect("io: Failed to rename file");

    if cmd_output.success() {
        println!("\x1b[1;33mSuccessfully renamed \x1b[0m{file_to_rename}\x1b[1;33m to \x1b[0m{new_file_name}");
        Ok(())
    } else {
        panic!("\x1b[1;31mCould not rename file with error: {cmd_output}\x1b[0m")
    }
}

#[cfg(test)]
mod tests {
    use std::{fs::File, path::PathBuf};

    use super::{get_possible_file_names, rename_file_in_dir};

    #[test]
    fn check_prefixes() {
        assert_eq!(
            get_possible_file_names("hi").unwrap(),
            vec![
                "cs_hi".to_string(),
                "math_hi".to_string(),
                "ened_hi".to_string(),
                "coop_hi".to_string(),
                "cheml_hi".to_string(),
                "chem_hi".to_string()
            ]
        )
    }

    #[test]
    pub fn check_renaming_file() {
        let _ = File::create("testing/test.txt");
        let file_test_one_name: String = PathBuf::from("testing/test.txt")
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        let directory: PathBuf =
            PathBuf::from(r"/Users/anshmendiratta/dev/rs_renamefile_tui/testing");
        let new_file_name: String = String::from("test_success.txt");

        assert!(rename_file_in_dir(directory, file_test_one_name, new_file_name).is_ok());
    }
}
