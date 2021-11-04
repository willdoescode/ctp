use std::fs;
use std::io::{Read, Write};
use std::path::Path;

fn replace_name_and_out(s: String, proj_name: &str, proj_out: &str) -> String {
    s.replace(crate::REPLACEABLE_NAME, proj_name)
        .replace(crate::REPLACEABLE_OUTPUT, proj_out)
}

pub fn copy_dir_all(
    src: impl AsRef<Path>,
    dst: impl AsRef<Path>,
    proj_name: &str,
    proj_out: &str,
) -> anyhow::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ft = entry.file_type()?;
        if ft.is_dir() {
            copy_dir_all(
                entry.path(),
                dst.as_ref().join(entry.file_name()),
                proj_name,
                proj_out,
            )?;
        } else {
            let mut contents = String::new();
            fs::File::open(&entry.path())?.read_to_string(&mut contents)?;
            fs::File::create(&entry.path())?.write(contents.as_bytes())?;
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }

    Ok(())
}
