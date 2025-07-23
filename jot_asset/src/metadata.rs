use std::{fs::File, path::Path};

use serde::Deserialize;

pub fn read_metadata<T: for<'a> Deserialize<'a>>(asset_path: &Path) -> Option<T> {
    let meta_path = Path::new(asset_path).with_extension("meta");

    meta_path.exists().then(|| {
        let file = File::open(&meta_path).expect(&format!(
            "failed to load asset metadata \"{}\"",
            meta_path.display()
        ));

        serde_yaml::from_reader(file).expect(&format!(
            "failed to deserialize asset metadata \"{}\"",
            meta_path.display()
        ))
    })
}
