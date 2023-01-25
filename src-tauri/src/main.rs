#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn helloworld(name: &str) {
    println!("hello world {}!!", name);
}

#[tauri::command]
fn mensajeDesdeElBackend(mensaje: &str) -> String{
    let mut respuesta: String = "Este mensaje viene del backend ".to_string();
    respuesta.push_str(mensaje);
    respuesta.into()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![helloworld, mensajeDesdeElBackend, greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
