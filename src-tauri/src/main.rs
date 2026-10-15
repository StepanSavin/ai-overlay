#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod clicker;

use tauri::image::Image;
use tauri::menu::{Menu, MenuItem};
use tauri::tray::TrayIconBuilder;
use tauri::{AppHandle, Emitter, Manager, PhysicalPosition, State, WebviewWindow};
use tauri_plugin_global_shortcut::{Code, Modifiers, Shortcut, ShortcutState};

use clicker::Clicker;

const OVERLAY_WINDOW: &str = "main";

const CLICKER_EVENT: &str = "clicker-state";

const OVERLAY_TOP_OFFSET: f64 = 16.0;

#[tauri::command]
fn clicker_state(clicker: State<Clicker>) -> bool {
    clicker.is_active()
}

fn toggle_clicker(app: &AppHandle) {
    let active = app.state::<Clicker>().toggle();
    let _ = app.emit(CLICKER_EVENT, active);
}

fn place_overlay(window: &WebviewWindow) {
    let Ok(Some(monitor)) = window.primary_monitor() else {
        return;
    };
    let Ok(size) = window.outer_size() else {
        return;
    };

    let area = monitor.work_area();
    let x = area.position.x + (area.size.width as i32 - size.width as i32) / 2;
    let y = area.position.y + (OVERLAY_TOP_OFFSET * monitor.scale_factor()) as i32;

    let _ = window.set_position(PhysicalPosition::new(x, y));
}

fn main() {
    let toggle_shortcut = Shortcut::new(
        Some(Modifiers::CONTROL | Modifiers::ALT | Modifiers::SHIFT),
        Code::KeyQ,
    );

    tauri::Builder::default()
        .manage(Clicker::spawn())
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_shortcut(toggle_shortcut)
                .expect("failed to parse the clicker shortcut")
                .with_handler(move |app, shortcut, event| {
                    if shortcut == &toggle_shortcut && event.state() == ShortcutState::Pressed {
                        toggle_clicker(app);
                    }
                })
                .build(),
        )
        .invoke_handler(tauri::generate_handler![clicker_state])
        .setup(|app| {
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            if let Some(window) = app.get_webview_window(OVERLAY_WINDOW) {
                let _ = window.set_visible_on_all_workspaces(true);
                let _ = window.set_ignore_cursor_events(true);
                place_overlay(&window);
                let _ = window.show();
            }

            let quit = MenuItem::with_id(app, "quit", "Выйти", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&quit])?;

            TrayIconBuilder::new()
                .icon(Image::from_bytes(include_bytes!("../icons/tray.png"))?)
                .icon_as_template(true)
                .tooltip("Auto Clicker — Ctrl+Alt+Shift+Q")
                .menu(&menu)
                .on_menu_event(|app, event| {
                    if event.id.as_ref() == "quit" {
                        app.exit(0);
                    }
                })
                .build(app)?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
