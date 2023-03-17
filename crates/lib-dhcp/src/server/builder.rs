use thiserror::Error;

use crate::{
    server::config::ServerConfig, Server, DEFAULT_REBIND_PERCENT, DEFAULT_RENEW_PERCENT,
    ONE_HOUR_SECS,
};

#[derive(Debug, Error)]
pub enum ServerBuilderError {
    #[error("using explicit renew and rebind times requires to set both values")]
    InvalidTimes,

    #[error("renew time (T1) must be smaller than rebind time (T2)")]
    InvalidPercent,

    #[error("at least one pool configuration is required")]
    InvalidPoolCount,
}

pub struct ServerBuilder {
    rebind_time: Option<u32>,
    rebind_percent: f64,

    renew_time: Option<u32>,
    renew_percent: f64,

    calculates_times: bool,
    lease_time: u32,

    pools: Vec<(String, String)>,
}

impl Default for ServerBuilder {
    fn default() -> Self {
        Self {
            rebind_percent: DEFAULT_REBIND_PERCENT,
            renew_percent: DEFAULT_RENEW_PERCENT,
            lease_time: ONE_HOUR_SECS,
            calculates_times: false,
            rebind_time: None,
            pools: Vec::new(),
            renew_time: None,
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

    pub fn with_renew_time(mut self, time: u32) -> Self {
        self.renew_time = Some(time);
        self
    }

    pub fn with_renew_percent(mut self, percent: f64) -> Self {
        self.renew_percent = percent;
        self
    }

    pub fn with_calculate_times(mut self, calc: bool) -> Self {
        self.calculates_times = calc;
        self
    }

    pub fn with_lease_time(mut self, time: u32) -> Self {
        self.lease_time = time;
        self
    }

    pub fn with_pool(mut self, name: String, range: String) -> Self {
        self.pools.push((name, range));
        self
    }

    pub fn build(self) -> Result<Server, ServerBuilderError> {
        // Determine if the server should send the T1 and T2 time
        let send_times =
            self.calculates_times || (self.rebind_time.is_some() && self.renew_time.is_some());

        // Make sure that both times are set when the user provided explicit
        // times for T1 and T2
        if (self.rebind_time.is_some() && self.renew_time.is_none())
            || (self.rebind_time.is_none() && self.renew_time.is_some())
        {
            return Err(ServerBuilderError::InvalidTimes);
        }

        // Make sure that T1 < T2
        if self.rebind_percent >= self.renew_percent {
            return Err(ServerBuilderError::InvalidPercent);
        }

        // Use the explicit time or default back to the default percent of lease time
        let rebind_time = self
            .rebind_time
            .unwrap_or((self.lease_time as f64 * self.rebind_percent) as u32);

        let renew_time = self
            .renew_time
            .unwrap_or((self.lease_time as f64 * self.renew_percent) as u32);

        // Check that there is at least one pool configured
        if self.pools.is_empty() {
            return Err(ServerBuilderError::InvalidPoolCount);
        }

        // Parse the pools
        // let pools = Vec::new();

        Ok(Server {
            is_running: false,
            config: ServerConfig {
                send_times,
                rebind_time,
                renew_time,
            },
        })
    }
}
