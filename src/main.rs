// main.rs

// use chrono::*;
use log::*;
use std::{env, error::Error, thread, time};
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

    let mut ll = SDL1000X::new(LAB_LOAD).await?;
    info!("Lab load at {:?}", ll.addr());
    info!("idn? {}", &ll.qry("*IDN?").await?);
    info!("mac? {}", &ll.qry("lan:mac?").await?);
    info!("ip? {}", &ll.qry("lan:ipad?").await?);
    info!("mask? {}", &ll.qry("lan:smask?").await?);
    info!("gw? {}", &ll.qry("lan:gat?").await?);

    info!("function? {}", &ll.qry("func?").await?);
    info!("sense? {}", &ll.get_state("syst:sens").await?);

    // info!("wave: {:?}", &lab_load.wave(Meas::Volt).await?);
    info!("voltage? {}", &ll.meas(Meas::Volt).await?);
    info!("current? {}", &ll.meas(Meas::Curr).await?);
    info!("power? {}", &ll.meas(Meas::Pow).await?);
    info!("resistance? {}", &ll.meas(Meas::Res).await?);

    info!("short {}", &ll.set_state(":shor", State::Off).await?);
    info!("input {}", &ll.set_state(":inp", State::Off).await?);
    info!("sense {}", &ll.set_state("syst:sens", State::On).await?);
    info!("input? {}", &ll.qry(":inp?").await?);
    info!("short? {}", &ll.qry(":shor?").await?);
    info!("irange? {}", &ll.qry(":curr:irang?").await?);
    info!("vrange? {}", &ll.qry(":curr:vrang?").await?);

    thread::sleep(time::Duration::new(1, 0));

    info!("func {}", &ll.set_func(Func::Curr).await?);
    // 5A or 30A
    info!("current range {}", &ll.set(":curr:irang", 5.0).await?);
    // 36V or 150V
    info!("voltage range {}", &ll.set(":curr:vrang", 36.0).await?);
    info!("current {}", &ll.set(":curr", 0.120).await?);
    info!("input {}", &ll.set_state(":inp", State::On).await?);

    thread::sleep(time::Duration::new(5, 0));

    info!("sense? {}", &ll.get_state("syst:sens").await?);
    info!("voltage? {}", &ll.meas(Meas::Volt).await?);
    info!("current? {}", &ll.meas(Meas::Curr).await?);
    info!("power? {}", &ll.meas(Meas::Pow).await?);
    info!("resistance? {}", &ll.meas(Meas::Res).await?);
    info!("current {}", &ll.set(":curr", 0.150).await?);

    thread::sleep(time::Duration::new(5, 0));

    info!("sense? {}", &ll.get_state("syst:sens").await?);
    info!("voltage? {}", &ll.meas(Meas::Volt).await?);
    info!("current? {}", &ll.meas(Meas::Curr).await?);
    info!("power? {}", &ll.meas(Meas::Pow).await?);
    info!("resistance? {}", &ll.meas(Meas::Res).await?);

    thread::sleep(time::Duration::new(1, 0));

    info!("input off {}", &ll.set_state(":inp", State::Off).await?);
    info!("sense {}", &ll.set_state("syst:sens", State::Off).await?);

    Ok(())
}
// EOF
