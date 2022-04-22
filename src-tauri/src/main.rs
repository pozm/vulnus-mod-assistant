#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use std::path::Path;
use std::io::Cursor;
use tauri::Runtime;
use tokio::{fs::{self, File}, io::AsyncWriteExt};
fn main() {

  tauri::Builder::default()
  	.invoke_handler(tauri::generate_handler![check_dir_exists,install_bepinex,install_plugin])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}


#[tauri::command(async)]
fn check_dir_exists(direc:&str) -> bool {
	Path::new(direc).exists()
}
#[tauri::command]
async fn install_bepinex(vulnus_dir:String) -> bool {
	let bytes = reqwest::get("https://builds.bepinex.dev/projects/bepinex_be/557/BepInEx_UnityMono_x64_da48b77_6.0.0-be.557.zip").await.unwrap().bytes().await.unwrap();
	let mut read = Cursor::new(bytes.to_vec());
	let mut zip = zip::ZipArchive::new(&mut read).unwrap();

	zip.extract(&vulnus_dir);
	return true
}
#[tauri::command]
async fn install_plugin(remote_dir:String,plugin_name:String,vulnus_dir:String) -> bool {
	let bytes = reqwest::get(&remote_dir).await.unwrap().bytes().await.unwrap();
	// let mut read = Cursor::new(bytes.to_vec());
	
	let PluginPath = Path::new(&vulnus_dir).join("./BepInEx/plugins/");
	File::create(PluginPath.join(format!("{}.dll",plugin_name))).await.unwrap().write_all(&bytes.to_vec()).await.unwrap();

	return true
}