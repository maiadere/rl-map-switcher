use std::fs::read_dir;
use std::path::PathBuf;
use steam_workshop_api::{Workshop, WorkshopItem};

#[derive(serde::Serialize, Clone)]
pub struct Map {
  pub path: String,
  pub title: String,
  pub preview_url: String,
}

fn find_map_file(workshop_path: String, id: String) -> Option<String> {
  let mut map_folder = PathBuf::new();
  map_folder.push(workshop_path);
  map_folder.push(id);

  for path in read_dir(map_folder).unwrap() {
    let path = path.unwrap().path();
    let ext = path.extension().unwrap();

    if ext == "udk" || ext == "upk" {
      return Some(path.display().to_string());
    }
  }

  None
}

pub fn load_maps(workshop_path: String) -> Vec<Map> {
  let workshop = Workshop::new(None);

  let map_ids: Vec<String> = read_dir(workshop_path.clone())
    .unwrap()
    .map(|path| {
      path
        .unwrap()
        .path()
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
    })
    .collect();

  let details: Vec<WorkshopItem> = match workshop.get_published_file_details(&map_ids) {
    Ok(details) => details,
    Err(err) => panic!("Failed to get file info: {}", err),
  };

  details
    .iter()
    .map(|item| {
      let path = find_map_file(workshop_path.clone(), item.publishedfileid.clone())
        .expect("Ce n'est pas possible");
      Map {
        path,
        title: item.title.clone(),
        preview_url: item.preview_url.clone(),
      }
    })
    .collect()
}
