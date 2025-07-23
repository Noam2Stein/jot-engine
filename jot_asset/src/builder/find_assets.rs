use std::{
    ffi::OsStr,
    fs::read_dir,
    path::{Path, PathBuf},
};

pub fn find_assets(assets_dir: &Path) -> impl Iterator<Item = PathBuf> {
    fn push_dir(dir: &Path, output: &mut Vec<PathBuf>) {
        for entry in read_dir(dir).expect("failed to read asset directory") {
            let entry = entry.expect("failed to read asset directory entry");

            let path = entry.path();
            let metadata = entry
                .metadata()
                .expect("failed to get asset directory entry metadata");

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
                panic!("multiple assets \"{}\"", path.with_extension("").display(),);
            }

            output.push(path);
        }

        for entry in read_dir(dir).expect("failed to read asset directory") {
            let entry = entry.expect("failed to read asset directory entry");

            let path = entry.path();
            let metadata = entry
                .metadata()
                .expect("failed to get asset directory entry metadata");

            if !metadata.is_dir() {
                continue;
            }

            if output.iter().any(|other| other.with_extension("") == dir) {
                continue;
            }

            push_dir(&path, output);
        }
    }

    let mut output = Vec::new();

    push_dir(assets_dir, &mut output);

    output.into_iter()
}
