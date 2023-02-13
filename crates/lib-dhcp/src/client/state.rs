pub enum ClientState {
    Init,
    InitReboot,
    Selecting,
    Rebooting,
    Requesting,
    Rebinding,
    Bound,
    Renewing,
}

impl Default for ClientState {
    fn default() -> Self {
        Self::Init
    }
}
