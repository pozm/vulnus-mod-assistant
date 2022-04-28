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
  	.invoke_handler(tauri::generate_handler![check_dir_exists,install_bepinex,install_plugin,check_install_bepinex])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}


#[tauri::command(async)]
fn check_dir_exists(direc:&str) -> bool {
	Path::new(direc).exists()
}
#[tauri::command]
async fn install_bepinex(vulnus_dir:String) -> bool {
	let bytes = reqwest::get("https://cdn.discordapp.com/attachments/649995055603384320/968213010197925898/UnityIL2CPP_x64.zip").await.unwrap().bytes().await.unwrap();
	let mut read = Cursor::new(bytes.to_vec());
	let mut zip = zip::ZipArchive::new(&mut read).unwrap();

	zip.extract(&vulnus_dir);
	return true
}
#[tauri::command]
async fn check_install_bepinex(vulnus_dir:String) -> bool {
	
	let vulnusPath = Path::new(&vulnus_dir);

	let winhttp_exists = vulnusPath.join("./winhttp.dll").exists();
	let bepinex_folder_exists = vulnusPath.join("./BepInEx").exists();

	return winhttp_exists && bepinex_folder_exists
}


#[tauri::command]
async fn install_plugin(remote_dir:String,plugin_name:String,vulnus_dir:String) -> bool {
	let bytes = reqwest::get(&remote_dir).await.unwrap().bytes().await.unwrap();
	// let mut read = Cursor::new(bytes.to_vec());
	
	let PluginPath = Path::new(&vulnus_dir).join("./BepInEx/plugins/");
	fs::create_dir_all(&PluginPath).await.unwrap();
	File::create(PluginPath.join(format!("{}.dll",plugin_name))).await.unwrap().write_all(&bytes.to_vec()).await.unwrap();

	return true
}