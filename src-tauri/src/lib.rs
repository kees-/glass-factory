// use std::process::Command;

mod masks;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![masks::contour])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// Checking whether command line options exist and are configured in shell to path
// For ffmpeg etc
// #[tauri::command]
// fn process_exists(cmd: &str) {
//     match Command::new("command").args(["-v", cmd]).spawn() {
//         Ok(_) => println!("{cmd} is ok"),
//         Err(e) => {
//             if let not_found = e.kind() {
//                 println!("{cmd} not found")
//             } else {
//                 println!("?");
//             }
//         }
//     }
// }
