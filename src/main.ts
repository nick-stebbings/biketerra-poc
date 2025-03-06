import { invoke } from "@tauri-apps/api/core";

class BluetoothClient {
  constructor() {
    console.log("constructed")
  }

  // Start scanning for devices
  async scan(filterUuids = []) {
    if (window.__TAURI__) {
      return window.__TAURI__.invoke('scan', {filter_uuids: filterUuids});
    }
    return Promise.reject('Tauri API not available');
  }
}

window.__TAURI__.btClient = new BluetoothClient();

window.addEventListener("DOMContentLoaded", () => {
  console.log('hey')
  window.__TAURI__.btClient.scan().then((msg) => {

    console.log(msg)
  });
});
