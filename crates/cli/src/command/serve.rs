use std::net::IpAddr;

use anyhow::Result;
use clap::Args;

use couriers::Couriers;

const HOST_ENV_VAR: &str = "HOST";
const PORT_ENV_VAR: &str = "PORT";

#[derive(Args, Clone, Debug)]
pub struct ServeOpt {
    #[clap(short = 'a', long = "address", env = HOST_ENV_VAR, default_value = "127.0.0.1")]
    pub host: IpAddr,
    #[clap(short = 'p', long = "port", env = PORT_ENV_VAR, default_value = "7878")]
    pub port: u16,
}

impl ServeOpt {
    pub async fn exec(&self) -> Result<()> {
        println!("Thanks for using Couriers! 🚀");
        println!("Starting server at http://{}:{}", self.host, self.port);
        println!("{:?}", Couriers { field: 32 });

        Ok(())
    }
}
