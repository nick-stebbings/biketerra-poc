use std::thread::sleep;
use std::time::{Duration, Instant};

use btleplug::api::{
    Central, CentralEvent, Manager as _, Peripheral as _, ScanFilter
};
use btleplug::platform::{Adapter, Manager as BTManager, Peripheral, PeripheralId};

use serde::de::DeserializeOwned;
use tauri::plugin::PluginApi;
use tauri::{AppHandle, Emitter, Manager, Runtime};

use crate::error::{Error, Result};
use crate::event::{DeviceInfo, Payload};
use crate::models::{ScanRequest, ScanResponse};
use futures::stream::StreamExt;

pub fn init_plugin<R: Runtime>() -> tauri::plugin::TauriPlugin<R> {
    tauri::plugin::Builder::new("bluetooth")
        .invoke_handler(tauri::generate_handler![scan])
        .setup(|app, api| {
            #[cfg(desktop)]
            let bluetooth = init(app, api)?;
            app.manage(bluetooth);
            Ok(())
        })
        .build()
}

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> Result<Bluetooth<R>> {
    Ok(Bluetooth(app.clone()))
}

impl<R: Runtime> Bluetooth<R> {
    pub async fn scan(&self, app: AppHandle<R>, payload: ScanRequest) -> Result<ScanResponse> {
        println!("Scan parameters {:?}", payload);

        let main_window = app.get_webview_window("main").unwrap();

        let manager = BTManager::new()
            .await
            .map_err(|e| Error::Bluetooth(e.to_string()))?;

        let adapters = manager
            .adapters()
            .await
            .map_err(|e| Error::Bluetooth(e.to_string()))?;

        let central = adapters.into_iter().next().ok_or(Error::NoAdapter)?;

        let mut events = central.events().await?;

        central.start_scan(ScanFilter::default()).await?;

        main_window
            .emit(
                "scan-start",
                Payload {
                    message: "Started scanning for bluetooth devices...".into(),
                },
            )
            .unwrap();

        let scan_duration = Duration::from_secs(10);
        let start_time = Instant::now();

        while let Some(event) = events.next().await {
            if Instant::now().duration_since(start_time) >= scan_duration {
                break;
            }

            if let CentralEvent::DeviceDiscovered(id) = event {
                let peripheral = central.peripheral(&id).await?;
                let properties = peripheral.properties().await?;
                let name = properties
                    .and_then(|p| p.local_name)
                    .map(|local_name| format!("Name: {local_name}"))
                    .unwrap_or_default();
                main_window
                    .emit(
                        "scan-device-found",
                        DeviceInfo {
                            name,
                            id: id.to_string()
                        },
                    )
                    .unwrap();
            }
        }

        Ok(ScanResponse { success: true })
    }
}

pub struct Bluetooth<R: Runtime>(AppHandle<R>);

pub trait BluetoothExt<R: Runtime> {
    fn bluetooth(&self) -> &Bluetooth<R>;
}

impl<R: Runtime, T: Manager<R>> BluetoothExt<R> for T {
    fn bluetooth(&self) -> &Bluetooth<R> {
        self.state::<Bluetooth<R>>().inner()
    }
}

#[tauri::command]
pub(crate) async fn scan<R: Runtime>(
    app: AppHandle<R>,
    payload: ScanRequest,
) -> crate::error::Result<ScanResponse> {
    app.bluetooth().scan(app.clone(), payload).await
}
