pub mod back_logic {
    use anyhow::Result;
    use std::{
        fs::{read_dir, DirEntry},
        path::{Path, PathBuf},
        process::Command,
    };

    pub fn get_possible_file_names(file_name: &str) -> Result<Vec<String>> {
        let assignments_directory = Path::new("/Users/anshmendiratta/Desktop/assignments/");
        let file_iter = read_dir(assignments_directory)?;
        let folders: Vec<DirEntry> = file_iter
            .filter_map(|f| {
                let file = f.ok()?;
                let file_metadata = file.metadata().ok()?;
                if file_metadata.is_dir() {
                    Some(file)
                } else {
                    None
                }
            })
            .collect();

        let possible_names: Vec<String> = folders
            .iter()
            .map(|folder| {
                let name_prefix = folder
                    .path()
                    .file_name()
                    .unwrap_or_default()
                    .to_str()
                    .unwrap_or_default()
                    .to_owned();
                format!("{}_{}", name_prefix, file_name)
            })
            .collect();

        Ok(possible_names)
    }

    pub fn rename_file_in_dir(
        directory_of_file: PathBuf,
        file_to_rename: String,
        new_file_name: String,
    ) -> std::io::Result<()> {
        let directory_of_file = directory_of_file.as_os_str().to_str().unwrap_or_default();
        let file_to_rename_path = format!("{}/{}", directory_of_file, file_to_rename);
        let new_file_path = format!("{}/{}", directory_of_file, new_file_name);

        let cmd_output = Command::new("mv")
            .args([file_to_rename_path, new_file_path])
            .status()?;

        if cmd_output.success() {
            println!("\x1b[1;33mSuccessfully renamed \x1b[0m{file_to_rename}\x1b[1;33m to \x1b[0m{new_file_name}");
            return Ok(());
        }

        eprintln!("\x1b[1;31mCould not rename file with error: {cmd_output}\x1b[0m");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::{fs::File, path::PathBuf};

    use super::back_logic::{get_possible_file_names, rename_file_in_dir};

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
