use tauri::{
    App, AppHandle, CustomMenuItem, Manager, RunEvent, SystemTray, SystemTrayEvent, SystemTrayMenu,
    SystemTrayMenuItem,
};

mod ping;
use ping::Worker;

#[tauri::command]
fn change_host(app_handle: AppHandle, new_host: &str) {
    app_handle
        .get_window("change-host-window")
        .unwrap()
        .hide()
        .unwrap();

    let tray_title = match Worker::set_hostname(new_host) {
        Ok(_) => format!("Host: {}", new_host),
        Err(e) => format!("error setting hostname: {}", e),
    };
    app_handle
        .tray_handle()
        .get_item("change-host")
        .set_title(tray_title)
        .unwrap();
}

fn tray_menu() -> SystemTrayMenu {
    SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("change-host", "Host: google.com:443"))
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("line0", "---").disabled())
        .add_item(CustomMenuItem::new("line1", "---").disabled())
        .add_item(CustomMenuItem::new("line2", "---").disabled())
        .add_item(CustomMenuItem::new("line3", "---").disabled())
        .add_item(CustomMenuItem::new("line4", "---").disabled())
        .add_item(CustomMenuItem::new("line5", "---").disabled())
        .add_item(CustomMenuItem::new("line6", "---").disabled())
        .add_item(CustomMenuItem::new("line7", "---").disabled())
        .add_item(CustomMenuItem::new("line8", "---").disabled())
        .add_item(CustomMenuItem::new("line9", "---").disabled())
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("quit", "Quit"))
}

fn init_tray(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let handle = app.handle();

    SystemTray::new()
        .with_id("main-tray")
        .with_menu(tray_menu())
        .on_event(move |event| {
            if let SystemTrayEvent::MenuItemClick { id, .. } = event {
                match id.as_str() {
                    "quit" => {
                        handle.exit(0);
                    }
                    "change-host" => {
                        let w = handle.get_window("change-host-window").unwrap();
                        w.show().unwrap();
                        w.set_focus().unwrap()
                    }
                    _ => {}
                }
            }
        })
        .build(app)?;

    Ok(())
}

fn run_ui_sync_thread(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let handle = app.handle();

    std::thread::spawn(move || loop {
        let tray = handle.tray_handle();
        let lines = Worker::current_stats();

        for (i, line) in lines.iter().enumerate() {
            let id = format!("line{}", i);
            tray.get_item(&id).set_title(format!("{}", line)).unwrap();
        }

        std::thread::sleep(std::time::Duration::from_secs(1));
    });

    Ok(())
}

fn main() {
    Worker::init(10);
    Worker::set_hostname("google.com:443").unwrap();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![change_host])
        .setup(|app| {
            init_tray(app)?;
            run_ui_sync_thread(app)?;
            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|_app_handle, event| {
            if let RunEvent::ExitRequested { api, .. } = event {
                api.prevent_exit();
            }
        })
}
