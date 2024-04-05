use std::{
    fs,
    path::{Path, PathBuf},
};

use super::DistributionCenter;

pub struct FileSystemStorage;

impl DistributionCenter for FileSystemStorage {
    fn store(&self, receiver: String, content: String, express: bool) {
        let project_dir = directories::ProjectDirs::from("dev", "buenzli", "paekli").unwrap();
        let storage_dir = project_dir.data_dir();

        let mut receiver_dir = storage_dir.join(receiver);
        if express {
            receiver_dir.push("express")
        }
        let receiver_dir = receiver_dir;

        fs::create_dir_all(&receiver_dir).unwrap();

        let time = time::OffsetDateTime::now_utc().to_string();
        let paekli_path = receiver_dir.join(time);

        if fs::metadata(&paekli_path).is_ok() {
            panic!("Cannot send paekli, storage is full.");
        }
        fs::write(paekli_path, content).expect("failed to store paekli");
    }

    fn retrieve(&self, receiver: String) -> Option<String> {
        let project_dir = directories::ProjectDirs::from("dev", "buenzli", "paekli").unwrap();
        let storage_dir = project_dir.data_dir();

        let receiver_dir = storage_dir.join(receiver);
        let express_dir = receiver_dir.join("express");

        let paekli_path = get_first_paekli_path_in_dir(&express_dir)
            .or_else(|| get_first_paekli_path_in_dir(&receiver_dir))?;

        match fs::read_to_string(&paekli_path) {
            Ok(content) => {
                fs::remove_file(paekli_path)
                    .expect("failed to remove received paekli from storage");
                Some(content)
            }
            Err(_) => panic!("There is no paekli to receive."),
        }
    }
}

fn get_first_paekli_path_in_dir(dir: &Path) -> Option<PathBuf> {
    let mut paekli: Vec<_> = fs::read_dir(dir)
        .into_iter()
        .flatten()
        .flatten()
        .filter(|e| e.metadata().is_ok_and(|m| m.is_file()))
        .map(|e| e.file_name())
        .collect();
    paekli.sort();
    paekli
        .into_iter()
        .next()?
        .into_string()
        .ok()
        .map(|name| dir.join(name))
}
