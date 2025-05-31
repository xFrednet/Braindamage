use std::{io::Write, path::Path};

pub(crate) fn load_file_or_exit(file: &Path) -> String {
    match std::fs::read_to_string(file) {
        Ok(value) => value,
        Err(err) => {
            eprintln!("error: couldn't read {}: {err}", file.display());
            std::process::exit(-1);
        }
    }
}

pub(crate) fn write_file_or_exit(file: &Path, data: &str) {
    fn inner(path: &Path, data: &str) -> std::io::Result<()> {
        let mut file = std::fs::File::create(path)?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            file.metadata()?.permissions().set_mode(0o655);
        }
        file.write_all(data.as_bytes())
    }
    inner(file, data).expect("Unable to write file");
}
