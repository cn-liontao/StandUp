use std::borrow::{Borrow, BorrowMut};
use tauri::api::dialog::FileDialogBuilder;
use tauri::{AppHandle, Manager, SystemTray, SystemTrayEvent, SystemTraySubmenu};
use tauri::{CustomMenuItem, SystemTrayMenu, SystemTrayMenuItem};
use crate::storage::io::{read_external_storage, save_to_external_storage};
use crate::storage::state::StandingState;
use crate::ui::service::{stand_or_sit, get_records, merge_records};
use crate::ui::window::{create_settings_window, toggle_main_window};

pub mod service;
pub mod window;

const STAND_OR_SIT: &str = "stand or sit";
const TOGGLE_WINDOW: &str = "toggle window";
const IMPORT_EXTERNAL: &str = "import external records";
const EXPORT_RECORDS: &str = "export records";
const SETTINGS: &str = "settings";
const QUIT: &str = "quit";

pub fn init_tray_menu(hide_on_start: bool) -> SystemTray {
    let tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new(STAND_OR_SIT.to_string(), "站起来"))
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new(TOGGLE_WINDOW.to_string(), if hide_on_start { "打开面板" } else { "隐藏面板" }))
        .add_item(CustomMenuItem::new(SETTINGS.to_string(), "打开设置"))
        .add_submenu(SystemTraySubmenu::new(
            "备份",
            SystemTrayMenu::new()
                .add_item(CustomMenuItem::new(IMPORT_EXTERNAL.to_string(), "导入记录"))
                .add_item(CustomMenuItem::new(EXPORT_RECORDS.to_string(), "导出记录"))
        ))
        .add_item(CustomMenuItem::new(QUIT.to_string(), "退出"));

    SystemTray::new().with_menu(tray_menu)
}

pub fn tray_menu_handler(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            STAND_OR_SIT => {
                let is_standing = stand_or_sit(&app.state());
                let item_handle = app.tray_handle().get_item(&id);
                let title = if !is_standing { "站起来" } else { "坐下歇会" };
                item_handle.set_title(title).unwrap();
                app.emit_all("records-update", get_records(&app.state())).expect("Emit [records-update] failed");
            }
            QUIT => {
                std::process::exit(0);
            }
            TOGGLE_WINDOW => {
                let visible = toggle_main_window(app);

                let item_handle = app.tray_handle().get_item(&id);
                let title = if visible { "隐藏面板" } else { "打开面板" };
                item_handle.set_title(title).unwrap();
            }
            IMPORT_EXTERNAL => {
                let app_handle = app.clone();
                FileDialogBuilder::new().pick_file(move |file_path| {
                    if let Some(external_path) = file_path {
                        let read_res = read_external_storage(external_path);
                        if let Ok(records) = read_res {
                            merge_records(app_handle, records);
                        }
                    }
                });
            }
            EXPORT_RECORDS => {
                let mut data: Vec<String> = vec![];
                {
                    let state = &app.state::<StandingState>();
                    let day_records = state.day_records.lock().unwrap();
                    data = (*day_records).iter().map(|r| r.to_string()).collect();
                }
                FileDialogBuilder::new().save_file(move |file_path| {
                    if let Some(external_path) = file_path {
                        println!("{:?}", external_path);
                        save_to_external_storage(external_path, data).unwrap()
                    }
                })
            }
            SETTINGS => {
                if let Some(settings_window) = app.get_window(SETTINGS) {
                    settings_window.set_focus().unwrap();
                } else {
                    create_settings_window(app.clone()).unwrap();
                }
            }
            _ => {}
        },
        _ => {}
    }
}
