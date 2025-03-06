use ::std::{fs::create_dir, path::PathBuf};

fn main() {
    let publish_path = PathBuf::from("..").join("publish");

    if !publish_path.exists() {
        create_dir(&publish_path).unwrap();
    }
    if !publish_path.join("data").exists() {
        create_dir(publish_path.join("data")).unwrap();
    }
    if !publish_path.join("data").join("cert").exists() {
        create_dir(publish_path.join("data").join("cert")).unwrap();
    }
    if !publish_path.join("data").join("www").exists() {
        create_dir(publish_path.join("data").join("www")).unwrap();
    }
    if !publish_path.join("data").join("public").exists() {
        create_dir(publish_path.join("data").join("public")).unwrap();
    }
    if !publish_path.join("data").join("private").exists() {
        create_dir(publish_path.join("data").join("private")).unwrap();
    }

    if PathBuf::from("..").join(".env").exists() {
        std::fs::copy(
            PathBuf::from("..").join(".env"),
            PathBuf::from("..").join("publish").join(".env"),
        ).unwrap();
    }
    if PathBuf::from("..").join("server-config.json").exists() {
        std::fs::copy(
            PathBuf::from("..").join("server-config.json"),
            PathBuf::from("..").join("publish").join("server-config.json"),
        ).unwrap();
    }
}
