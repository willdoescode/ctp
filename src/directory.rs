use anyhow::{Context, Result};
use std::fs::{self, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

fn replace_placeholders(s: String, proj_name: &str, proj_out: &str) -> String {
    s.replace(crate::REPLACEABLE_NAME, proj_name)
        .replace(crate::REPLACEABLE_OUTPUT, proj_out)
}

fn ensure_directory_does_not_exist<P>(path: P) -> Result<()>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();

    if path.exists() && path.is_dir() {
        return Err(anyhow::anyhow!(
            "Directory already exists: {}",
            path.display()
        ));
    }

    Ok(())
}

fn copy_file(dst: &Path, entry: fs::DirEntry, proj_name: &str, proj_out: &str) -> Result<()> {
    let output_path = dst.join(entry.file_name());
    let mut contents = String::new();

    fs::File::open(&entry.path())
        .with_context(|| format!("Failed to source file: {}", entry.path().display()))?
        .read_to_string(&mut contents)
        .with_context(|| format!("Failed to read file: {}", entry.path().display()))?;

    let contents = replace_placeholders(contents, proj_name, proj_out);

    let mut output_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&output_path)
        .with_context(|| format!("Failed to create output file: {}", output_path.display()))?;

    output_file
        .write_all(contents.as_bytes())
        .with_context(|| format!("Failed to write to output file: {}", output_path.display()))?;

    Ok(())
}

fn is_git_directory<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref()
        .file_name()
        .map(|name| name.to_str().unwrap_or("").starts_with(".git"))
        .unwrap_or(false)
}

pub fn copy_dir_recur(
    src: impl AsRef<Path>,
    dst: impl AsRef<Path>,
    proj_name: &str,
    proj_out: &str,
) -> Result<()> {
    if is_git_directory(&src) {
        return Ok(());
    };

    ensure_directory_does_not_exist(&dst)?;

    fs::create_dir_all(&dst).with_context(|| {
        format!(
            "Failed to destination directory: {}",
            dst.as_ref().display()
        )
    })?;

    let src = src.as_ref().to_owned();

    for entry in fs::read_dir(&src)
        .with_context(|| format!("Failed to read source directory: {}", src.display()))?
    {
        let entry = entry?;
        let file_type = entry
            .file_type()
            .with_context(|| format!("Failed to get file type for: {}", entry.path().display()))?;

        if file_type.is_dir() {
            copy_dir_recur(
                entry.path(),
                dst.as_ref().join(entry.file_name()),
                proj_name,
                proj_out,
            )?;
        } else {
            copy_file(dst.as_ref(), entry, proj_name, proj_out)?;
        }
    }

    Ok(())
}
