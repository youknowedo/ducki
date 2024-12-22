use std::fs;

use cursive::{views::Dialog, View};
use cursive_hjkl::HjklToDirectionWrapperView;
use directories::ProjectDirs;

pub fn read_temp_file<T>(id: &str) -> Result<T, String>
where
    T: serde::de::DeserializeOwned,
{
    _read_temp_file(id, None)
}
pub fn read_temp_file_with_siv<T>(siv: &mut cursive::Cursive, id: &str) -> Result<T, String>
where
    T: serde::de::DeserializeOwned,
{
    _read_temp_file(id, Some(siv))
}

fn _read_temp_file<T>(id: &str, siv: Option<&mut cursive::Cursive>) -> Result<T, String>
where
    T: serde::de::DeserializeOwned,
{
    let project_dirs = match ProjectDirs::from("dev", "sigfredo", "ducki") {
        Some(dirs) => dirs,
        None => {
            match siv {
                Some(siv) => {
                    siv.add_layer(Dialog::info("Could not find project directories"));
                }
                None => {
                    panic!("Could not find project directories");
                }
            }
            return Err("Could not find project directories".to_string());
        }
    };
    let cache_dir = project_dirs.cache_dir().to_path_buf();
    let temp_data_path = cache_dir.join(id).to_str().unwrap().to_string();

    match fs::read_to_string(temp_data_path.as_str()) {
        Ok(contents) => match serde_json::from_str::<T>(&contents) {
            Ok(data) => Ok(data),
            Err(err) => Err(err.to_string()),
        },
        Err(err) => {
            panic!("Something went wrong: {}", err);
        }
    }
}

pub fn write_temp_file<T>(id: &str, data: &T) -> Result<(), String>
where
    T: serde::Serialize,
{
    _write_temp_file(id, data, None)
}
pub fn write_temp_file_with_siv<T>(
    siv: &mut cursive::Cursive,
    id: &str,
    data: &T,
) -> Result<(), String>
where
    T: serde::Serialize,
{
    _write_temp_file(id, data, Some(siv))
}

fn _write_temp_file<T>(id: &str, data: &T, siv: Option<&mut cursive::Cursive>) -> Result<(), String>
where
    T: serde::Serialize,
{
    let project_dirs = match ProjectDirs::from("dev", "sigfredo", "ducki") {
        Some(dirs) => dirs,
        None => {
            match siv {
                Some(siv) => {
                    siv.add_layer(Dialog::info("Could not find project directories"));
                }
                None => {
                    panic!("Could not find project directories");
                }
            }
            return Err("Could not find project directories".to_string());
        }
    };
    let cache_dir = project_dirs.cache_dir().to_path_buf();

    match fs::exists(cache_dir.clone()) {
        Ok(exists) => {
            if !exists {
                match fs::create_dir_all(cache_dir.clone()) {
                    Ok(_) => {}
                    Err(err) => match siv {
                        Some(siv) => {
                            siv.add_layer(Dialog::info(format!("Something went wrong: {}", err)));
                        }
                        None => {
                            panic!("Something went wrong: {}", err);
                        }
                    },
                }
            }
        }
        Err(_) => match fs::create_dir_all(cache_dir.clone()) {
            Ok(_) => {}
            Err(err) => match siv {
                Some(siv) => {
                    siv.add_layer(Dialog::info(format!("Something went wrong: {}", err)));
                }
                None => {
                    panic!("Something went wrong: {}", err);
                }
            },
        },
    }

    let temp_data_path = cache_dir.join(id).to_str().unwrap().to_string();

    match fs::write(
        temp_data_path.as_str(),
        serde_json::to_string(data).unwrap(),
    ) {
        Ok(_) => Ok(()),
        Err(err) => {
            panic!("Something went wrong: {}", err);
        }
    }
}

pub fn hjkl<V: View>(view: V) -> HjklToDirectionWrapperView<V> {
    HjklToDirectionWrapperView::new(view)
}
