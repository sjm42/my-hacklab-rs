// main.rs

// use chrono::*;
use log::*;
use std::{env, error::Error, thread, time};
use structopt::StructOpt;

mod utils;
use utils::sdl1000x;
use utils::spd3303x;

use crate::utils::{sdl1000x::SDL1000X, spd3303x::SPD3303X};

const LAB_LOAD: &str = "lab-LOAD.siu.ro:5025";
const LAB_POWER: &str = "lab-power.siu.ro:5025";

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

    let mut lp = SPD3303X::new(LAB_POWER).await?;
    let mut ll = SDL1000X::new(LAB_LOAD).await?;
    info!("Lab PWR at {:?}", lp.addr());
    info!("Lab LOAD at {:?}", ll.addr());

    info!("PWR idn? {}", &lp.req("*IDN?").await?);
    info!("PWR ip? {}", &lp.req("ip?").await?);
    info!("PWR mask? {}", &lp.req("mask?").await?);
    info!("PWR gw? {}", &lp.req("gate?").await?);

    info!("LOAD idn? {}", &ll.req("*IDN?").await?);
    info!("LOAD mac? {}", &ll.req("lan:mac?").await?);
    info!("LOAD ip? {}", &ll.req("lan:ipad?").await?);
    info!("LOAD mask? {}", &ll.req("lan:smask?").await?);
    info!("LOAD gw? {}", &ll.req("lan:gat?").await?);

    info!("LOAD function? {}", &ll.req("func?").await?);
    info!("LOAD sense? {}", &ll.get_state("syst:sens").await?);

    info!("LOAD short SET {}", &ll.set_state(":shor", sdl1000x::State::Off).await?);
    info!("LOAD input SET {}", &ll.set_state(":inp", sdl1000x::State::Off).await?);
    info!("LOAD sense SET {}", &ll.set_state("syst:sens", sdl1000x::State::On).await?);

    info!("PWR track 0"); lp.send("output:track 0").await?;
    info!("PWR ch1 off"); lp.send("output ch1,off").await?;
    info!("PWR ch2 off"); lp.send("output ch2,off").await?;
    info!("PWR ch3 off"); lp.send("output ch3,off").await?;
    info!("PWR wave ch1 off"); lp.send("output:wave ch1,off").await?;
    info!("PWR wave ch2 off"); lp.send("output:wave ch2,off").await?;

    info!("PWR ch1 volt SET {}", &lp.set("ch1:volt", 4.250).await?);
    info!("PWR ch1 curr SET {}", &lp.set("ch1:curr", 0.250).await?);
    info!("PWR ch1 SET on"); lp.send("output ch1,on").await?;

    info!("LOAD input? {}", &ll.req(":inp?").await?);
    info!("LOAD short? {}", &ll.req(":shor?").await?);
    info!("LOAD irange? {}", &ll.req(":curr:irang?").await?);
    info!("LOAD vrange? {}", &ll.req(":curr:vrang?").await?);

    info!("***");
    thread::sleep(time::Duration::new(1, 0));

    info!("LOAD func SET {}", &ll.set_func(sdl1000x::Func::Curr).await?);
    // 5A or 30A
    info!("LOAD irange SET {}", &ll.set(":curr:irang", 5.0).await?);
    // 36V or 150V
    info!("LOAD vrange SET {}", &ll.set(":curr:vrang", 36.0).await?);
    info!("LOAD current SET {:.3}", &ll.set(":curr", 0.120).await?);
    info!("LOAD input SET {}", &ll.set_state(":inp", sdl1000x::State::On).await?);

    info!("***");
    thread::sleep(time::Duration::new(2, 0));

    info!("LOAD sense? {}", &ll.get_state("syst:sens").await?);
    info!("LOAD volt? {:.3}", &ll.meas(sdl1000x::Meas::Volt).await?);
    info!("LOAD curr? {:.3}", &ll.meas(sdl1000x::Meas::Curr).await?);
    info!("LOAD power? {:.3}", &ll.meas(sdl1000x::Meas::Pow).await?);
    info!("LOAD res? {:.3}", &ll.meas(sdl1000x::Meas::Res).await?);

    info!("PWR ch1 volt? {:.3}", &lp.meas(spd3303x::Ch::Ch1, spd3303x::Meas::Volt).await?);
    info!("PWR ch1 curr? {:.3}", &lp.meas(spd3303x::Ch::Ch1, spd3303x::Meas::Curr).await?);
    info!("PWR ch1 power? {:.3}", &lp.meas(spd3303x::Ch::Ch1, spd3303x::Meas::Pow).await?);
    info!("***");
    info!("PWR ch1 volt SET {}", &lp.set("ch1:volt", 8.500).await?);
    info!("LOAD current SET {:.3}", &ll.set(":curr", 0.150).await?);

    info!("***");
    thread::sleep(time::Duration::new(2, 0));

    info!("LOAD volt? {:.3}", &ll.meas(sdl1000x::Meas::Volt).await?);
    info!("LOAD curr? {:.3}", &ll.meas(sdl1000x::Meas::Curr).await?);
    info!("LOAD power? {:.3}", &ll.meas(sdl1000x::Meas::Pow).await?);
    info!("LOAD res? {:.3}", &ll.meas(sdl1000x::Meas::Res).await?);
    info!("PWR ch1 volt? {:.3}", &lp.meas(spd3303x::Ch::Ch1, spd3303x::Meas::Volt).await?);
    info!("PWR ch1 curr? {:.3}", &lp.meas(spd3303x::Ch::Ch1, spd3303x::Meas::Curr).await?);
    info!("PWR ch1 power? {:.3}", &lp.meas(spd3303x::Ch::Ch1, spd3303x::Meas::Pow).await?);

    info!("***");
    thread::sleep(time::Duration::new(1, 0));

    info!("LOAD input SET {}", &ll.set_state(":inp", sdl1000x::State::Off).await?);
    info!("LOAD sense SET {}", &ll.set_state("syst:sens", sdl1000x::State::Off).await?);
    info!("PWR ch1 SET off"); lp.send("output ch1,off").await?;

    Ok(())
}
// EOF
