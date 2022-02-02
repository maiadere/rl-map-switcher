#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod map_loader;
mod texture_installer;

use map_loader::load_maps;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{Read, Write};

use native_dialog::{FileDialog, MessageDialog, MessageType};

use configparser::ini::Ini;
use directories::ProjectDirs;
use notify::{watcher, RecursiveMode, Watcher};
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::time::Duration;
use tauri::Manager;

struct AppState {
  workshop_path: String,
  rocket_league_path: String,
}

#[tauri::command]
fn init(app_handle: tauri::AppHandle, state: tauri::State<AppState>) {
  let maps = load_maps(state.workshop_path.clone());
  app_handle.emit_all("update-maps", maps).unwrap();
}

#[tauri::command]
fn watch_maps(app_handle: tauri::AppHandle, state: tauri::State<AppState>) {
  let workshop_path = state.workshop_path.clone();

  std::thread::spawn(move || loop {
    let (tx, rx) = channel();

    let mut watcher = watcher(tx, Duration::from_secs(1)).unwrap();
    watcher
      .watch(workshop_path.clone(), RecursiveMode::Recursive)
      .unwrap();

    match rx.recv() {
      Ok(_event) => {
        let maps = load_maps(workshop_path.clone());
        app_handle.emit_all("update-maps", maps).unwrap();
      }
      Err(e) => println!("watch error: {:?}", e),
    }
  });
}

#[tauri::command]
fn change_map(path: String, state: tauri::State<AppState>) -> bool {
  let mut mods_path = PathBuf::from(state.rocket_league_path.clone());
  mods_path.push("TAGame");
  mods_path.push("CookedPCConsole");
  mods_path.push("mods");

  let mut map = mods_path.clone();
  map.push("Labs_Underpass_P.upk");

  std::fs::create_dir_all(mods_path).unwrap();

  let mut in_file = File::open(path).unwrap();
  let mut buffer = Vec::new();
  in_file.read_to_end(&mut buffer).unwrap();

  let mut out_file = OpenOptions::new().write(true).open(map).unwrap();

  match out_file.write_all(&buffer) {
    Ok(_) => true,
    Err(_) => false,
  }
}

fn main() {
  let mut app_state = AppState {
    workshop_path: "C:\\Program Files (x86)\\Steam\\steamapps\\workshop\\content\\252950"
      .to_string(),
    rocket_league_path: "C:\\Program Files (x86)\\Steam\\steamapps\\common\\rocketleague"
      .to_string(),
  };

  if let Some(config_dir) = ProjectDirs::from("com", "vlo-software", "rl-map-switcher") {
    let config_dir = config_dir.config_dir();
    std::fs::create_dir_all(config_dir).unwrap();
    let mut config_path = PathBuf::from(config_dir);
    config_path.push("settings.ini");

    if config_path.exists() {
      let mut config = Ini::new();
      config.load(config_path).unwrap();

      app_state.workshop_path = config.get("settings", "workshop_path").unwrap();
      app_state.rocket_league_path = config.get("settings", "rocket_league_path").unwrap();
    } else {
      if MessageDialog::new()
        .set_type(MessageType::Info)
        .set_title("RL Map Switcher")
        .set_text("Do you want to install the textures?")
        .show_confirm()
        .unwrap()
      {
        texture_installer::install_textures(app_state.rocket_league_path.clone());
      }

      if MessageDialog::new()
        .set_type(MessageType::Info)
        .set_title("RL Map Switcher")
        .set_text("Do you want to choose custom maps folder?")
        .show_confirm()
        .unwrap()
      {
        let path = FileDialog::new().show_open_single_dir().unwrap().unwrap();
        app_state.workshop_path = path.display().to_string();
      }

      if MessageDialog::new()
        .set_type(MessageType::Info)
        .set_title("RL Map Switcher")
        .set_text("Do you want to choose custom Rocket League folder?")
        .show_confirm()
        .unwrap()
      {
        let path = FileDialog::new().show_open_single_dir().unwrap().unwrap();
        app_state.rocket_league_path = path.display().to_string();
      }

      let mut config = Ini::new();
      config.set(
        "settings",
        "workshop_path",
        Some(app_state.workshop_path.clone()),
      );
      config.set(
        "settings",
        "rocket_league_path",
        Some(app_state.rocket_league_path.clone()),
      );
      config.write(config_path).unwrap();
    };
  }

  tauri::Builder::default()
    .manage(app_state)
    .invoke_handler(tauri::generate_handler![init, watch_maps, change_map])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
