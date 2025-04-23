#![windows_subsystem = "windows"]

use device_query::{DeviceQuery, DeviceState, MousePosition};
use screenlocker::lock_screen;
use std::{
    thread,
    time::{Duration, Instant},
};
use tray_item::{IconSource, TrayItem};

fn main() {
    let app_name: String = String::from("No croissant party");
    let idle_threshold = Duration::from_secs(60);
    let check_interval = Duration::from_secs(5);

    let mut tray = create_tray(&app_name);

    let inactivity_label_id = tray
        .inner_mut()
        .add_label_with_id("Inactive for 0s")
        .unwrap();

    tray.inner_mut().add_separator().unwrap();

    tray.add_menu_item("Quit", || {
        std::process::exit(0);
    })
    .unwrap();

    start_inactivity_monitor(tray, inactivity_label_id, idle_threshold, check_interval);

    loop {
        thread::sleep(Duration::from_secs(60));
    }
}

fn create_tray(app_name: &str) -> TrayItem {
    TrayItem::new(app_name, IconSource::Resource("croissant-icon")).unwrap()
}

fn start_inactivity_monitor(
    mut tray: TrayItem,
    label_id: u32,
    idle_threshold: Duration,
    check_interval: Duration,
) {
    thread::spawn(move || {
        let device_state = DeviceState::new();
        let mut last_activity = Instant::now();
        let mut last_mouse_coords: MousePosition = (0, 0);

        loop {
            thread::sleep(check_interval);

            let mouse = device_state.get_mouse();

            if mouse.coords != last_mouse_coords {
                last_mouse_coords = mouse.coords;
                last_activity = Instant::now();
            }

            let idle_time = Instant::now().duration_since(last_activity);

            set_inactivity_label_text(&mut tray, label_id, &idle_time.as_secs().to_string());

            if idle_time > idle_threshold {
                handle_inactivity();
                thread::sleep(Duration::from_secs(60));
            }
        }
    });
}

fn set_inactivity_label_text(tray: &mut TrayItem, label_id: u32, idle_time: &str) {
    tray.inner_mut()
        .set_label(&format!("Inactive for {}s", idle_time), label_id)
        .unwrap();
}

fn handle_inactivity() {
    if let Err(e) = lock_screen() {
        eprintln!("Failed to lock screen: {}", e);
    }
}
