
extern crate assert_cmd;
extern crate assert_fs;
extern crate predicates;

#[cfg(test)]
mod integration_tests {
    use assert_cmd::Command;
    use assert_fs::prelude::*;
    use predicates::prelude::*;
    use std::path::{ Path };

    #[test]
    fn test_help() {
        let mut cmd = Command::cargo_bin("atlasbuilder").unwrap();
        cmd.arg("--help")
            .assert()
            .success()
            .code(0)
            .stdout(predicate::str::contains("Builds texture atlas images with JSON output"));
    }

    #[test]
    fn test_no_input_args() {
        let mut cmd = Command::cargo_bin("atlasbuilder").unwrap();
        cmd.assert()
            .failure()
            .code(2)
            .stderr(predicate::str::contains("error: The following required arguments were not provided:"));
    }

    #[test]
    fn test_specify_image_doesnt_exist() {
        let mut cmd = Command::cargo_bin("atlasbuilder").unwrap();
        let assert = cmd
            .arg("dontexist.png")
            .assert();
        assert
            .failure()
            .code(1)
            .stderr(predicate::str::contains("Error: File does not exist"));
    }

    #[test]
    fn test_specify_directory_doesnt_exist() {
    }

    #[test]
    fn test_specify_empty_directory() {
    }

    #[test]
    fn test_specify_directory_with_random_files_no_images() {
    }

    #[test]
    fn test_single_input_image() {
        let test_data_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("test_fixtures");
        
        let temp_dir = assert_fs::TempDir::new().unwrap().into_persistent();
        let out_image = temp_dir.child("out.png");
        let out_json = temp_dir.child("out.json");
        let mut cmd = Command::cargo_bin("atlasbuilder").unwrap();
        let assert = cmd
            .arg("--image-output")
            .arg(out_image.to_owned())
            .arg("--meta-output")
            .arg(out_json.to_owned())
            .arg(test_data_path.join("input/input1.png"))
            .assert();
        assert
            .success()
            .code(0);

        out_image.assert(predicate::path::exists());
        out_image.assert(predicate::path::eq_file(test_data_path.join("results/single_input_file_result/out.png")));
        out_json.assert(predicate::path::exists());
        out_json.assert(predicate::path::eq_file(test_data_path.join("results/single_input_file_result/out.json")));
    }
}
