// use std::process::Command;

mod masks;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![masks::contour, generate_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn generate_command(text_fragment: &str) -> String {
    let my_text: String = format!(
        "I have magically created the phrase '{text_fragment}'.
I hope you enjoy it."
    );
    println!("{my_text}");
    return my_text;
}
