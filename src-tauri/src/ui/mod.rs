use crate::ui::service::{stand_or_sit, get_records};
use tauri::{AppHandle, Manager, SystemTray, SystemTrayEvent};
use tauri::{CustomMenuItem, SystemTrayMenu, SystemTrayMenuItem};

pub mod service;

const STAND_OR_SIT: &str = "stand or sit";
const TOGGLE_WINDOW: &str = "toggle window";
const QUIT: &str = "quit";

pub fn init_tray_menu() -> SystemTray {
    let tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new(STAND_OR_SIT.to_string(), "站立"))
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new(TOGGLE_WINDOW.to_string(), "显示"))
        .add_item(CustomMenuItem::new(QUIT.to_string(), "关闭"));
    return SystemTray::new().with_menu(tray_menu);
}

pub fn tray_menu_handler(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            STAND_OR_SIT => {
                let is_standing = stand_or_sit(app.state());
                let item_handle = app.tray_handle().get_item(&id);
                let title = if !is_standing { "站立" } else { "坐下" };
                item_handle.set_title(title).unwrap();
                app.emit_all("records-update", get_records(app.state())).expect("Emit [records-update] failed");
            }
            QUIT => {
                std::process::exit(0);
            }
            TOGGLE_WINDOW => {
                let window = app.get_window("main").unwrap();
                let item_handle = app.tray_handle().get_item(&id);
                if let Ok(window_visible) = window.is_visible() {
                    let title = if !window_visible { "隐藏" } else { "显示" };
                    item_handle.set_title(title).unwrap();
                    if window_visible {
                        window.hide().unwrap();
                    } else {
                        window.show().unwrap();
                    }
                }
            }
            _ => {}
        },
        _ => {}
    }
}
