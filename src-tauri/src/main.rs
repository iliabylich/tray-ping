use tauri::{
    App, AppHandle, CustomMenuItem, Manager, RunEvent, SystemTray, SystemTrayEvent, SystemTrayMenu,
    SystemTrayMenuItem,
};

mod ping;
use ping::Worker;

const DEFAULT_HOST: &str = "google.com:443";

fn display_host(host: &str) -> String {
    format!("Host: {}", host)
}

const CHANGE_HOST_MENU_ITEM_ID: &str = "change-host";
const CHANGE_HOST_WINDOW_ID: &str = "change-host-window";
const QUIT_MENU_ITEM_ID: &str = "quit";
const TRAY_HEIGHT: usize = 15;

#[tauri::command]
fn change_host(app_handle: AppHandle, new_host: &str) {
    app_handle
        .get_window(CHANGE_HOST_WINDOW_ID)
        .expect("no 'change host' window")
        .hide()
        .expect("failed to hide window");

    let tray_title = match Worker::set_hostname(new_host) {
        Ok(_) => display_host(new_host),
        Err(e) => format!("error setting hostname: {}", e),
    };
    app_handle
        .tray_handle()
        .get_item(CHANGE_HOST_MENU_ITEM_ID)
        .set_title(tray_title)
        .expect("failed to set tray title");
}

fn tray_menu() -> SystemTrayMenu {
    let mut tray = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new(
            CHANGE_HOST_MENU_ITEM_ID,
            display_host(DEFAULT_HOST),
        ))
        .add_native_item(SystemTrayMenuItem::Separator);

    for i in 0..TRAY_HEIGHT {
        tray = tray.add_item(CustomMenuItem::new(format!("line{}", i), "---").disabled());
    }

    tray.add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new(QUIT_MENU_ITEM_ID, "Quit"))
}

fn init_tray(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let app_handle = app.handle();
    let window = app
        .get_window(CHANGE_HOST_WINDOW_ID)
        .expect("no 'change host' window");

    SystemTray::new()
        .with_id("main-tray")
        .with_menu(tray_menu())
        .on_event(move |event| {
            if let SystemTrayEvent::MenuItemClick { id, .. } = event {
                match id.as_str() {
                    QUIT_MENU_ITEM_ID => {
                        app_handle.exit(0);
                    }
                    CHANGE_HOST_MENU_ITEM_ID => {
                        window.show().expect("failed to show window");
                        window.set_focus().expect("failed to focus window");
                    }
                    _ => {}
                }
            }
        })
        .build(app)?;

    Ok(())
}

fn subscribe_to_ping_worker(app: &mut App) {
    let tray_handle = app.tray_handle();
    Worker::subscribe(move |lines| {
        for (i, line) in lines.iter().enumerate() {
            tray_handle
                .get_item(&format!("line{}", i))
                .set_title(format!("{}", line))
                .expect("failed to show ping status");
        }
    });
}

fn main() {
    Worker::init(TRAY_HEIGHT);
    Worker::set_hostname("google.com:443").expect("failed to set initial hostname");

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![change_host])
        .setup(|app| {
            init_tray(app)?;
            subscribe_to_ping_worker(app);
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
