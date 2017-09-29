#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateLinkParameters {
    client_id: i32,
    lock_device: bool,
    lock_timeout: u32,
    device: String,
}

impl CreateLinkParameters {
    pub fn new() -> Self {
        CreateLinkParameters {
            client_id: 0,
            lock_device: false,
            lock_timeout: 0,
            device: "inst0".to_string(),
        }
    }
}
