use btleplug::api::{bleuuid::uuid_from_u16, Central, Manager as _, Peripheral as _, ScanFilter, WriteType};
use btleplug::platform::{Adapter, Manager as BTManager, Peripheral, PeripheralId};

use serde::de::DeserializeOwned;
use tauri::{AppHandle, Manager, Runtime};
use tauri::plugin::PluginApi;
use crate::models::{ScanRequest, ScanResponse};
use crate::error::{Error, Result};

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
    pub async fn scan(&self, payload: ScanRequest) -> Result<ScanResponse> {
        println!("Scan parameters {:?}", payload);
        let manager = BTManager::new()
            .await
            .map_err(|e| Error::Bluetooth(e.to_string()))?;

        let adapters = manager
            .adapters()
            .await
            .map_err(|e| Error::Bluetooth(e.to_string()))?;

        let central = adapters.into_iter().next().ok_or(Error::NoAdapter)?;
        list_peripherals(central).await;

        Ok(ScanResponse { success: true })
    }
}

async fn list_peripherals(central: Adapter) -> () {
    for p in central.peripherals().await.unwrap() {
        println!("{:?}", p);
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
    app.bluetooth().scan(payload).await
}