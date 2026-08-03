#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::image::Image;
use tauri::menu::{Menu, MenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{Manager, WebviewWindow, WindowEvent};
use tauri_plugin_global_shortcut::{Code, Modifiers, Shortcut, ShortcutState};

const OVERLAY_WINDOW: &str = "main";

const HIDE_ON_BLUR: bool = !cfg!(debug_assertions);

fn show_overlay(window: &WebviewWindow) {
    let _ = window.center();
    let _ = window.show();
    let _ = window.set_focus();
}

fn toggle_overlay(window: &WebviewWindow) {
    if window.is_visible().unwrap_or(false) {
        let _ = window.hide();
    } else {
        show_overlay(window);
    }
}

fn main() {
    let toggle_shortcut = Shortcut::new(Some(Modifiers::SUPER | Modifiers::SHIFT), Code::Space);

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_shortcut(toggle_shortcut)
                .expect("failed to parse the overlay shortcut")
                .with_handler(move |app, shortcut, event| {
                    if shortcut != &toggle_shortcut || event.state() != ShortcutState::Pressed {
                        return;
                    }

                    if let Some(window) = app.get_webview_window(OVERLAY_WINDOW) {
                        toggle_overlay(&window);
                    }
                })
                .build(),
        )
        .setup(|app| {
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            if let Some(window) = app.get_webview_window(OVERLAY_WINDOW) {
                let _ = window.set_visible_on_all_workspaces(true);
            }

            let show = MenuItem::with_id(
                app,
                "show",
                "Показать оверлей",
                true,
                Some("Cmd+Shift+Space"),
            )?;
            let quit = MenuItem::with_id(app, "quit", "Выйти", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show, &quit])?;

            TrayIconBuilder::new()
                .icon(Image::from_bytes(include_bytes!("../icons/tray.png"))?)
                .icon_as_template(true)
                .tooltip("AI Overlay")
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "show" => {
                        if let Some(window) = app.get_webview_window(OVERLAY_WINDOW) {
                            show_overlay(&window);
                        }
                    }
                    "quit" => app.exit(0),
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        if let Some(window) = tray.app_handle().get_webview_window(OVERLAY_WINDOW) {
                            toggle_overlay(&window);
                        }
                    }
                })
                .build(app)?;

            Ok(())
        })
        .on_window_event(|window, event| match event {
            WindowEvent::Focused(false) if HIDE_ON_BLUR => {
                let _ = window.hide();
            }
            WindowEvent::CloseRequested { api, .. } => {
                api.prevent_close();
                let _ = window.hide();
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
