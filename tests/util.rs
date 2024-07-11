use std::path::{Path, PathBuf};

use uuid::Uuid;

pub struct TestContext {
    pub source: PathBuf,
    pub result: PathBuf,
    pub expected: PathBuf,
}

impl TestContext {
    pub fn new(name: &str) -> Self {
        let tempdir = get_tempdir();
        std::fs::create_dir_all(&tempdir).unwrap();
        Self {
            source: format!("tests/fixtures/source/{}", name).into(),
            result: tempdir,
            expected: format!("tests/fixtures/expected/{}", name).into(),
        }
    }

    pub fn assert(&self) {
        dir_assert::assert_paths!(&self.result, &self.expected);
        // assert_contents_equal(&self.result, &self.expected);
    }
}

fn assert_contents_equal(target: &Path, expected: &Path) {
    if target.is_dir() {
        for entry in std::fs::read_dir(target).unwrap() {
            let entry = entry.unwrap();
            let target = entry.path();
            let expected = expected.join(entry.file_name());
            assert_contents_equal(&target, &expected);
        }

        return;
    }

    let target_contents = std::fs::read_to_string(target).unwrap();
    let expected_contents = std::fs::read_to_string(expected).unwrap();
    assert_eq!(target_contents.trim(), expected_contents.trim());
}

fn get_tempdir() -> PathBuf {
    let run_id = Uuid::new_v4();
    std::env::temp_dir().join(run_id.to_string())
}
