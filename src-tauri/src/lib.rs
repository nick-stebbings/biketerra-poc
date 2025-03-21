mod bluetooth;
mod models;
mod error;
mod event;
use tauri::{Manager, Listener};

const BT_CLIENT_INIT: &str = r#"class BluetoothClient {
      constructor() {
        console.log("JS BluetoothClient instantiated and listening to scan events");
        window.__TAURI__.event.listen('scan-start', (event) => {
            console.log(event);
        });
        window.__TAURI__.event.listen('scan-device-found', (event) => {
            console.log(event);
        });
      }

      // Start scanning for devices
      async scan(filterUuids = []) {
        if (window.__TAURI__) {
          console.log("Scanning...");
          return window.__TAURI__.core.invoke('scan', {
            payload: {
              filterUuids
            }
          });
        }
        return Promise.reject('Tauri API not available');
      }
    }

    window.btClient = new BluetoothClient();
    window.btClient.scan();
"#;

pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let webview = app.get_webview_window("main").unwrap();
            webview.eval(BT_CLIENT_INIT)?;
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(bluetooth::init_plugin())
        .invoke_handler(tauri::generate_handler![bluetooth::scan])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
