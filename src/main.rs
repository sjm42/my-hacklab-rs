// main.rs

// use chrono::*;
use log::*;
use std::{env, error::Error};
use structopt::StructOpt;

mod utils;
use utils::sdl1000x::*;

// const LAB_LOAD: &str = "lab-load.siu.ro:5025";
const LAB_LOAD: &str = "10.28.0.62:5025";

#[derive(Debug, Clone, StructOpt)]
pub struct GlobalOptions {
    #[structopt(short, long)]
    pub debug: bool,
    #[structopt(short, long)]
    pub trace: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let opt = GlobalOptions::from_args();
    let loglevel = if opt.trace {
        LevelFilter::Trace
    } else if opt.debug {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };

    env_logger::Builder::new()
        .filter_level(loglevel)
        .format_timestamp_secs()
        .init();
    info!("Starting up...");
    debug!("Git branch: {}", env!("GIT_BRANCH"));
    debug!("Git commit: {}", env!("GIT_COMMIT"));
    debug!("Source timestamp: {}", env!("SOURCE_TIMESTAMP"));
    debug!("Compiler version: {}", env!("RUSTC_VERSION"));
    debug!("Global config: {:?}", &opt);

    let mut lab_load = SDL1000X::new(LAB_LOAD).await?;
    info!("Lab load at {:?}", lab_load.addr());
    info!("SCPI idn: {}", &lab_load.cmd("*IDN?").await?);

    info!("mac: {}", &lab_load.cmd("lan:mac?").await?);
    info!("ip: {}", &lab_load.cmd("lan:ipad?").await?);
    info!("mask: {}", &lab_load.cmd("lan:smask?").await?);
    info!("gw: {}", &lab_load.cmd("lan:gat?").await?);

    info!("wave: {:?}", &lab_load.wave("volt").await?);
    info!("voltage: {}", &lab_load.volt().await?);
    info!("current: {}", &lab_load.curr().await?);
    info!("power: {}", &lab_load.pow().await?);
    info!("resistance: {}", &lab_load.res().await?);

    Ok(())
}
// EOF
