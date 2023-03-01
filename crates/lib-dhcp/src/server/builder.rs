use crate::{Server, ONE_HOUR_SECS};

pub enum ServerBuilderError {}

pub struct ServerBuilder {
    rebind_time: Option<u32>,
    rebind_percent: f64,

    renew_time: Option<u32>,
    renew_percent: f64,

    calculates_times: bool,
    lease_time: u32,
}

impl Default for ServerBuilder {
    fn default() -> Self {
        Self {
            rebind_time: None,
            rebind_percent: 0.875,
            renew_time: None,
            renew_percent: 0.5,
            calculates_times: false,
            lease_time: ONE_HOUR_SECS,
        }
    }
}

impl ServerBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_rebind_time(mut self, time: u32) -> Self {
        self.rebind_time = Some(time);
        self
    }

    pub fn with_rebind_percent(mut self, percent: f64) -> Self {
        self.rebind_percent = percent;
        self
    }

    pub fn build(self) -> Result<Server, ServerBuilderError> {
        Ok(Server::new())
    }
}
