use tauri::menu::{Menu, MenuItem, PredefinedMenuItem};
use tauri::tray::{MouseButton, TrayIconBuilder, TrayIconEvent};
use tauri::{AppHandle, Emitter, Manager};

pub fn setup_tray(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let settings_i = MenuItem::with_id(app, "settings", "Settings", true, None::<&str>)?;
    let clean_i = MenuItem::with_id(app, "clean", "Clean Now", true, None::<&str>)?;
    let separator = PredefinedMenuItem::separator(app)?;

    let menu = Menu::with_items(app, &[&clean_i, &settings_i, &separator, &quit_i])?;

    let mut builder = TrayIconBuilder::with_id("tray")
        .tooltip("Mouzi")
        .menu(&menu)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "quit" => {
                app.exit(0);
            }
            "settings" => {
                show_settings_window(app);
            }
            "clean" => {
                let _ = app.emit("trigger-clean", "");
            }
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click { button, .. } = event {
                if button == MouseButton::Left {
                    show_popup_window(tray.app_handle());
                }
            }
        });

    if let Some(icon) = app.default_window_icon() {
        builder = builder.icon(icon.clone());
    }

    let _tray = builder.build(app)?;

    Ok(())
}

fn show_popup_window(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("popup") {
        let _ = window.show();
        let _ = window.set_focus();
    } else {
        let window = tauri::WebviewWindowBuilder::new(
            app,
            "popup",
            tauri::WebviewUrl::App("/#/popup".into()),
        )
        .title("Mouzi")
        .inner_size(300.0, 420.0)
        .decorations(false)
        .transparent(true)
        .always_on_top(true)
        .skip_taskbar(true)
        .build();

        if let Ok(win) = window {
            let _ = win.show();
            let _ = win.set_focus();
        }
    }
}

fn show_settings_window(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("settings") {
        let _ = window.show();
        let _ = window.set_focus();
    } else {
        let window = tauri::WebviewWindowBuilder::new(
            app,
            "settings",
            tauri::WebviewUrl::App("/#/settings".into()),
        )
        .title("Mouzi Settings")
        .inner_size(900.0, 650.0)
        .min_inner_size(700.0, 500.0)
        .build();

        if let Ok(win) = window {
            let _ = win.show();
            let _ = win.set_focus();
        }
    }
}
