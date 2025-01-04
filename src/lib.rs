pub mod back_logic {
    use anyhow::Result;
    use std::{
        fs::{read_dir, DirEntry},
        path::{Path, PathBuf},
        process::Command,
    };

    pub fn get_possible_file_names(
        assignments_directory: &Path,
        file_name: &str,
    ) -> Result<Vec<String>> {
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
            .map(move |folder| {
                let folder_path = folder.path().clone();
                let name_prefix = folder_path
                    .file_name()
                    .unwrap_or_default()
                    .to_str()
                    .unwrap_or_default();
                // joined_string.as_str()
                [name_prefix, file_name].join("_").clone()
            })
            .collect();

        Ok(possible_names)
    }

    pub fn rename_file_in_dir<'a>(
        file_to_rename: &'a str,
        new_file_name: &'a str,
    ) -> anyhow::Result<String> {
        let binding = PathBuf::from(file_to_rename);
        let file_to_rename_parent_dir = binding
            .parent()
            .unwrap_or(Path::new(""))
            .to_str()
            .unwrap_or_default();
        let new_file_path = format!("{}/{}", file_to_rename_parent_dir, new_file_name);

        let mut cmd_base = Command::new("mv");
        let cmd = cmd_base.args([file_to_rename, &new_file_path]);
        let cmd_output = cmd.spawn()?.wait_with_output()?;

        let cmd_succeeded = cmd_output.status.success();
        if cmd_succeeded {
            return Ok(format!("\x1b[1;33mSuccessfully renamed \x1b[0m{file_to_rename}\x1b[1;33m to \x1b[0m{new_file_name}"));
        };

        let stderr_as_string: String = String::from_utf8(cmd_output.stderr)?;
        return Ok(format!(
            "\x1b[1;31mCould not rename file with error: {stderr_as_string}\x1b[0m",
        ));
    }
}

#[cfg(test)]
mod tests {
    use std::{
        fs::File,
        path::{Path, PathBuf},
    };

    use super::back_logic::{get_possible_file_names, rename_file_in_dir};

    #[test]
    fn check_prefixes() {
        let assignments_dir: &Path = Path::new("/home/mintdesktop/Desktop/assignments/");
        assert_eq!(
            get_possible_file_names(assignments_dir, "hi")
                .unwrap()
                .sort(),
            vec![
                "physics_hi".to_string(),
                "math_hi".to_string(),
                "ened_hi".to_string(),
                "chem_hi".to_string()
            ]
            .sort()
        )
    }

    #[test]
    pub fn check_renaming_file() {
        let _ = File::create("testing/test.txt");
        let binding = PathBuf::from("testing/test.txt");
        let file_test_one_name: &str = binding.file_name().unwrap().to_str().unwrap();
        let new_file_name: &str = "test_success.txt";

        assert!(rename_file_in_dir(file_test_one_name, new_file_name).is_ok());
    }
}
