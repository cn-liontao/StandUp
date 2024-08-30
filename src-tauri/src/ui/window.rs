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


pub fn create_settings_window(app: AppHandle) -> Result<Window> {
    tauri::WindowBuilder::new(
        &app,
        "settings",
        tauri::WindowUrl::App("settings".into()),
    )
        .title("设置")
        .inner_size(f64::from(229), f64::from(115))
        .build()
}

pub fn toggle_main_window(app: &AppHandle) -> bool {
    if let Some(window) = app.get_window("main") {
        if let Ok(window_visible) = window.is_visible() {
            if window_visible {
                window.hide().unwrap();
                false
            } else {
                window.show().unwrap();
                window.set_focus().unwrap();
                true
            }
        } else { false }
    } else {
        create_main_window(app.clone()).unwrap();
        true
    }
}