use tauri::{AppHandle, Result, Window};

pub fn create_main_window(app: AppHandle) -> Result<Window> {
    tauri::WindowBuilder::new(
        &app,
        "main",
        tauri::WindowUrl::App("".into())
    )
        .focused(true)
        .resizable(false)
        .inner_size(f64::from(252), f64::from(240))
        .title("")
        .build()
}