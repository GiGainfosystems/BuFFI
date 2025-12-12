#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::env::temp_dir;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn test_basic_example() {
        // load config from example
        let mut config_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        config_dir.push("..");
        config_dir.push("example");
        config_dir.push("generate_bindings");
        config_dir.push("api_config.toml");
        let config_dir = config_dir.canonicalize().unwrap();

        let toml_string = fs::read_to_string(&config_dir).expect("Config path does not exist");
        let config: buffi::Config =
            toml::from_str(&toml_string).expect("Could not read config toml");

        // generate bindings and write them to temp directory
        // make sure to clean temp directory first
        let mut temp_dir = temp_dir();
        temp_dir.push("buffi");
        fs::create_dir_all(&temp_dir).unwrap();
        for file in fs::read_dir(&temp_dir).unwrap() {
            let file = file.unwrap();
            fs::remove_file(file.path().as_path()).unwrap();
        }
        buffi::generate_bindings(temp_dir.as_path(), config);

        // prepare path to already generated example files in "include"
        let mut include_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        include_dir.push("..");
        include_dir.push("example");
        include_dir.push("buffi_example");
        include_dir.push("src");
        include_dir.push("include");
        let include_dir = include_dir.canonicalize().unwrap();

        // collect files that exist in the example and the temp dir
        let mut read_file = HashSet::new();

        // go over generated files and compare them with existing ones
        for f in fs::read_dir(&temp_dir).unwrap() {
            let entry = f.unwrap();
            let path = &entry.path();
            let file_name = path
                .file_name()
                .expect("Output directory should only contain files")
                .to_str()
                .unwrap();

            // find already generated file
            let old_path = include_dir.join(file_name);

            if old_path.exists() {
                let generated = fs::read_to_string(path).unwrap();
                let old = fs::read_to_string(&old_path).unwrap();
                similar_asserts::assert_eq!(example_code: old, temp_code: generated);
            } else {
                panic!("A file was generated that did not exist before: {file_name}");
            }

            read_file.insert(file_name.to_owned());
        }

        let ignore_files = [".DS_Store"];

        // now check if all the files in the include directory are represented in the temp directory
        for f in fs::read_dir(&include_dir).unwrap() {
            let entry = f.unwrap();
            let path = &entry.path();
            let file_name = path
                .file_name()
                .expect("Output directory should only contain files")
                .to_str()
                .unwrap();

            if !read_file.contains(file_name) && !ignore_files.contains(&file_name) {
                panic!(
                    "`{}` exists in the include directory, but not in the generated output",
                    path.display()
                );
            }
        }
    }
}
