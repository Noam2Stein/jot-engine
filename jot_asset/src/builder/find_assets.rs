use std::{
    ffi::OsStr,
    fs::read_dir,
    path::{Path, PathBuf},
};

use super::*;

pub fn find_assets(assets_dir: &Path) -> BuilderResult<impl Iterator<Item = PathBuf>> {
    fn push_dir(dir: &Path, output: &mut Vec<PathBuf>) -> BuilderResult<()> {
        let entries = read_dir(dir);

        let entries = entries
            .map_err(|_| builder_error!("failed to read asset directory \"{}\"", dir.display()))?;

        for entry in entries {
            let entry = entry.map_err(|_| {
                builder_error!(
                    "failed to read asset directory entry in \"{}\"",
                    dir.display()
                )
            })?;

            let path = entry.path();
            let metadata = entry.metadata().map_err(|_| {
                builder_error!("failed to read asset metadata in \"{}\"", dir.display())
            })?;

            if !metadata.is_file() {
                continue;
            }

            let ext = match path.extension() {
                Some(ext) => ext,
                None => continue,
            };

            if ext == OsStr::new("meta") {
                continue;
            }

            if output
                .iter()
                .any(|other| other.with_extension("") == path.with_extension(""))
            {
                return Err(builder_error!(
                    "multiple assets \"{}\"",
                    path.with_extension("").display(),
                ));
            }

            output.push(path);
        }

        let entries = read_dir(dir);

        let entries = entries
            .map_err(|_| builder_error!("failed to read asset directory \"{}\"", dir.display()))?;

        for entry in entries {
            let entry = entry.map_err(|_| {
                builder_error!(
                    "failed to read asset directory entry in \"{}\"",
                    dir.display()
                )
            })?;

            let path = entry.path();
            let metadata = entry.metadata().map_err(|_| {
                builder_error!("failed to read asset metadata in \"{}\"", dir.display())
            })?;

            if !metadata.is_dir() {
                continue;
            }

            if output.iter().any(|other| other.with_extension("") == dir) {
                continue;
            }

            push_dir(&path, output)?;
        }

        Ok(())
    }

    let mut output = Vec::new();

    push_dir(assets_dir, &mut output)?;

    Ok(output.into_iter())
}
