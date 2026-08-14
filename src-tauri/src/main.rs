use device_query::{DeviceQuery, DeviceState, Keycode};
use std::sync::{
    atomic::{AtomicBool, AtomicU64, Ordering},
    Arc,
};
use std::thread;
use std::time::{Duration, Instant};
use tauri::menu::{CheckMenuItem, Menu, MenuItem, PredefinedMenuItem, Submenu};
use tauri::tray::{TrayIcon, TrayIconBuilder};
use tauri::{ActivationPolicy, AppHandle, Manager};

const TITLE_NORMAL: &str = "🍏";
const TITLE_ALERT: &str = "🍎";
const POLL_INTERVAL_MS: u64 = 50;

struct SharedState {
    threshold_ms: AtomicU64,
    is_alert: AtomicBool,
}

fn update_tray_title(tray: &TrayIcon, is_alert: bool) {
    let title = if is_alert { TITLE_ALERT } else { TITLE_NORMAL };
    let _ = tray.set_title(Some(title));
}

fn set_threshold_and_refresh_checks(app: &AppHandle, threshold_ms: u64) {
    let Some(state) = app
        .try_state::<Arc<SharedState>>()
        .map(|s| s.inner().clone())
    else {
        return;
    };
    state.threshold_ms.store(threshold_ms, Ordering::Relaxed);
}

fn spawn_keyboard_monitor(app: AppHandle, tray: TrayIcon) {
    thread::spawn(move || {
        let device = DeviceState::new();
        let mut left_cmd_down_since: Option<Instant> = None;

        loop {
            let keys = device.get_keys();
            let left_cmd_down = keys.contains(&Keycode::Command)
                || keys.contains(&Keycode::LMeta)
                || keys.contains(&Keycode::RMeta);

            if left_cmd_down {
                if left_cmd_down_since.is_none() {
                    left_cmd_down_since = Some(Instant::now());
                }
            } else {
                left_cmd_down_since = None;
            }

            let Some(state) = app
                .try_state::<Arc<SharedState>>()
                .map(|s| s.inner().clone())
            else {
                thread::sleep(Duration::from_millis(POLL_INTERVAL_MS));
                continue;
            };

            let threshold = Duration::from_millis(state.threshold_ms.load(Ordering::Relaxed));
            let should_alert = left_cmd_down_since
                .map(|t| t.elapsed() >= threshold)
                .unwrap_or(false);

            let was_alert = state.is_alert.swap(should_alert, Ordering::Relaxed);
            if was_alert != should_alert {
                update_tray_title(&tray, should_alert);
            }

            thread::sleep(Duration::from_millis(POLL_INTERVAL_MS));
        }
    });
}

fn build_tray(app: &AppHandle) -> tauri::Result<TrayIcon> {
    let threshold_1 =
        CheckMenuItem::with_id(app, "threshold_1", "阈值 1 秒", true, false, None::<&str>)?;
    let threshold_2 =
        CheckMenuItem::with_id(app, "threshold_2", "阈值 2 秒", true, true, None::<&str>)?;
    let threshold_3 =
        CheckMenuItem::with_id(app, "threshold_3", "阈值 3 秒", true, false, None::<&str>)?;
    let threshold_5 =
        CheckMenuItem::with_id(app, "threshold_5", "阈值 5 秒", true, false, None::<&str>)?;

    let threshold_submenu = Submenu::with_items(
        app,
        "触发阈值",
        true,
        &[&threshold_1, &threshold_2, &threshold_3, &threshold_5],
    )?;

    let quit = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
    let separator = PredefinedMenuItem::separator(app)?;
    let menu = Menu::with_items(app, &[&threshold_submenu, &separator, &quit])?;

    TrayIconBuilder::with_id("main")
        .menu(&menu)
        .title(TITLE_NORMAL)
        .build(app)
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(target_os = "macos")]
            app.handle()
                .set_activation_policy(ActivationPolicy::Accessory)?;

            let shared = Arc::new(SharedState {
                threshold_ms: AtomicU64::new(2000),
                is_alert: AtomicBool::new(false),
            });
            app.manage(shared);

            let tray = build_tray(app.handle())?;

            app.on_menu_event(|app, event| match event.id().as_ref() {
                "threshold_1" => set_threshold_and_refresh_checks(app, 1000),
                "threshold_2" => set_threshold_and_refresh_checks(app, 2000),
                "threshold_3" => set_threshold_and_refresh_checks(app, 3000),
                "threshold_5" => set_threshold_and_refresh_checks(app, 5000),
                "quit" => app.exit(0),
                _ => {}
            });

            spawn_keyboard_monitor(app.handle().clone(), tray);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
