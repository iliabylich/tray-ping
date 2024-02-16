use std::sync::mpsc::Receiver;

use tauri::{
    ActivationPolicy, App, CustomMenuItem, RunEvent, SystemTray, SystemTrayEvent, SystemTrayMenu,
    SystemTrayMenuItem,
};

mod ping;
use ping::{PingResult, Worker};

mod fixed_size_queue;

const DEFAULT_HOST: &str = "google.com:443";

const QUIT_MENU_ITEM_ID: &str = "quit";
const TRAY_HEIGHT: usize = 15;

fn tray_menu() -> SystemTrayMenu {
    let mut tray = SystemTrayMenu::new();

    for i in 0..TRAY_HEIGHT {
        tray = tray.add_item(CustomMenuItem::new(format!("line{}", i), "---").disabled());
    }

    tray.add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new(QUIT_MENU_ITEM_ID, "Quit"))
}

fn init_tray(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let app_handle = app.handle();

    SystemTray::new()
        .with_menu(tray_menu())
        .on_event(move |event| {
            if let SystemTrayEvent::MenuItemClick { id, .. } = event {
                if id == QUIT_MENU_ITEM_ID {
                    app_handle.exit(0);
                }
            }
        })
        .build(app)?;

    Ok(())
}

fn subscribe_to_ping_worker(
    app: &mut App,
    recv_pings: Receiver<[Option<PingResult>; TRAY_HEIGHT]>,
) {
    let tray_handle = app.tray_handle();
    std::thread::spawn(move || {
        while let Ok(pings) = recv_pings.recv() {
            for (i, line) in pings.into_iter().enumerate() {
                let id = format!("line{}", i);
                let title = match line {
                    Some(Ok(result)) => result.to_string(),
                    Some(Err(err)) => err,
                    None => "---".to_string(),
                };

                tray_handle
                    .get_item(&id)
                    .set_title(title)
                    .expect("failed to show ping status");
            }
        }
    });
}

fn main() {
    let recv_pings = Worker::init(DEFAULT_HOST).expect("failed to init ping worker");

    let mut app = tauri::Builder::default()
        .setup(move |app| {
            init_tray(app)?;
            subscribe_to_ping_worker(app, recv_pings);
            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while building tauri application");

    app.set_activation_policy(ActivationPolicy::Accessory);

    app.run(|_app_handle, event| {
        if let RunEvent::ExitRequested { api, .. } = event {
            api.prevent_exit();
        }
    })
}
