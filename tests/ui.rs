use assert_cmd::Command;
use insta::{assert_snapshot, Settings};
use regex::Regex;

#[test]
fn test_bf() {
    let mut settings = Settings::new();
    settings.remove_snapshot_suffix();
    settings.set_prepend_module_to_snapshot(false);

    insta::glob!("ui/*.bf", |path| {
        let file_stem = path.file_stem().unwrap().to_string_lossy();

        let mut com = Command::cargo_bin("braindamage").unwrap();
        com.arg("run").arg("--file").arg(&path);

        let file_content = std::fs::read_to_string(path).unwrap();
        let re = Regex::new(r"//@arg\s+([^=\n]+)=([^\n]+)").unwrap();
        for cap in re.captures_iter(&file_content) {
            com.arg(&cap[1]);
            com.arg(&cap[2]);
        }

        let output = com.output().unwrap();

        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();

        settings.set_snapshot_path(path.parent().unwrap());
        settings.bind(|| {
            assert_snapshot!(format!("{}.stdout", file_stem), &stdout);
            assert_snapshot!(format!("{}.stderr", file_stem), &stderr);
        });
    });
}
