const URL: &str = "https://cdn.discordapp.com/attachments/875044086841753671/938209095662567545/Workshop-textures.zip";

use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use tempfile::NamedTempFile;

pub fn install_textures(rocket_league_path: String) {
  let resp = reqwest::blocking::get(URL).unwrap().bytes().unwrap();
  let mut file = NamedTempFile::new().unwrap();
  file.write_all(&resp).unwrap();

  let mut archive = zip::ZipArchive::new(file).unwrap();

  for i in 0..archive.len() {
    let mut out_path = PathBuf::from(rocket_league_path.clone());
    let mut file = archive.by_index(i).unwrap();
    println!("{:?}", file.name());

    out_path.push("TAGame");
    out_path.push("CookedPCConsole");
    out_path.push(file.name());

    let mut out_file = OpenOptions::new().write(true).open(out_path).unwrap();
    std::io::copy(&mut file, &mut out_file).unwrap();
  }
}
