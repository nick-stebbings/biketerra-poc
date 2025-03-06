#[derive(Clone, serde::Serialize)]
pub struct Payload {
    pub message: String,
}
#[derive(Clone, serde::Serialize)]
pub struct DeviceInfo {
    pub name: String,
    pub id: String,
}