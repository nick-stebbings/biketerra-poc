mod bluetooth;
mod models;
mod error;
use tauri::Manager;

const BT_CLIENT_INIT: &str = r#"class BluetoothClient {
      constructor() {
        console.log("JS BluetoothClient instantiated")
      }

      // Start scanning for devices
      async scan(filterUuids = []) {
        if (window.__TAURI__) {
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
