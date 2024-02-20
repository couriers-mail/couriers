use std::net::IpAddr;

use anyhow::Result;
use clap::Args;

use server::{serve, Config};

const HOST_ENV_VAR: &str = "HOST";
const PORT_ENV_VAR: &str = "PORT";

#[derive(Args, Clone, Debug)]
pub struct ServeOpt {
    #[clap(long = "host", env = HOST_ENV_VAR, default_value = "127.0.0.1")]
    pub host: IpAddr,
    #[clap(short = 'p', long = "port", env = PORT_ENV_VAR, default_value = "7878")]
    pub port: u16,
}

impl ServeOpt {
    pub async fn exec(&self) -> Result<()> {
        tracing_subscriber::fmt::init();

        let config = Config::from(self);

        serve(config).await?;

        Ok(())
    }
}

impl From<&ServeOpt> for server::Config {
    fn from(opt: &ServeOpt) -> Self {
        server::Config {
            address: (opt.host, opt.port).into(),
        }
    }
}
