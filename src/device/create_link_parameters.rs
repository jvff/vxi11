#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateLinkParameters {
    client_id: i32,
    lock_device: bool,
    lock_timeout: u32,
    device: String,
}
