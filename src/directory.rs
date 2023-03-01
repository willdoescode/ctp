use std::fs::{self, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;

fn replace_name_and_out(s: String, proj_name: &str, proj_out: &str) -> String {
    s.replace(crate::REPLACEABLE_NAME, proj_name)
        .replace(crate::REPLACEABLE_OUTPUT, proj_out)
}

fn dir_exists_check<P>(path: P) -> Result<(), anyhow::Error>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();

    if path.is_dir() && path.exists() {
        return Err(anyhow::anyhow!(
            "Directory already exists: {}",
            path.display()
        ));
    }

    Ok(())
}

fn copy_file(
    dst: &impl AsRef<Path>,
    entry: fs::DirEntry,
    proj_name: &str,
    proj_out: &str,
) -> Result<(), anyhow::Error> {
    let output = dst.as_ref().join(entry.file_name());
    let mut contents = String::new();

    fs::File::open(&entry.path())?.read_to_string(&mut contents)?;
    let contents = replace_name_and_out(contents, proj_name, proj_out);

    fs::copy(entry.path(), &output)?;

    let mut f = OpenOptions::new()
        .read(false)
        .write(true)
        .truncate(true)
        .open(output)?;
    f.seek(SeekFrom::Start(0))?;
    f.write_all(contents.as_bytes())?;
    Ok(())
}

fn check_git(src: &impl AsRef<Path>) -> bool {
    src.as_ref()
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .starts_with(".git")
}

pub fn copy_dir_all(
    src: impl AsRef<Path>,
    dst: impl AsRef<Path>,
    proj_name: &str,
    proj_out: &str,
) -> anyhow::Result<()> {
    if check_git(&src) {
        return Ok(());
    };

    dir_exists_check(&dst)?;
    fs::create_dir_all(&dst)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ft = entry.file_type()?;

        match ft.is_dir() {
            true => {
                copy_dir_all(
                    entry.path(),
                    dst.as_ref().join(entry.file_name()),
                    proj_name,
                    proj_out,
                )?;
            }
            _ => {
                copy_file(&dst, entry, proj_name, proj_out)?;
            }
        }
    }

    Ok(())
}
