// main.rs

// use chrono::*;
use log::*;
use std::{env, error::Error, thread, time};
use structopt::StructOpt;

use my_hacklab::*;

const LAB_LOAD: &str = "lab-load.siu.ro:5025";
const LAB_POWER: &str = "lab-power.siu.ro:5025";

fn main() -> Result<(), Box<dyn Error>> {
    let opt = OptsCommon::from_args();
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

    let mut lp = SPD3303X::new(LAB_POWER)?;
    let mut ll = SDL1000X::new(LAB_LOAD)?;
    info!("Lab PWR at {:?}", lp.addr());
    info!("Lab LOAD at {:?}", ll.addr());

    info!("PWR idn? {}", &lp.req("*IDN?")?);
    info!("PWR ip? {}", &lp.req("ip?")?);
    info!("PWR mask? {}", &lp.req("mask?")?);
    info!("PWR gw? {}", &lp.req("gate?")?);

    info!("LOAD idn? {}", &ll.req("*IDN?")?);
    info!("LOAD mac? {}", &ll.req("lan:mac?")?);
    info!("LOAD ip? {}", &ll.req("lan:ipad?")?);
    info!("LOAD mask? {}", &ll.req("lan:smask?")?);
    info!("LOAD gw? {}", &ll.req("lan:gat?")?);

    info!("LOAD function? {}", &ll.req("func?")?);
    info!("LOAD sense? {}", &ll.get_state("syst:sens")?);

    info!("LOAD short SET {}", &ll.set_state(":shor", PortState::Off)?);
    info!("LOAD input SET {}", &ll.set_state(":inp", PortState::Off)?);
    info!(
        "LOAD sense SET {}",
        &ll.set_state("syst:sens", PortState::On)?
    );

    info!("PWR track 0");
    lp.send("output:track 0")?;
    info!("PWR ch1 off");
    lp.send("output ch1,off")?;
    info!("PWR ch2 off");
    lp.send("output ch2,off")?;
    info!("PWR ch3 off");
    lp.send("output ch3,off")?;
    info!("PWR wave ch1 off");
    lp.send("output:wave ch1,off")?;
    info!("PWR wave ch2 off");
    lp.send("output:wave ch2,off")?;

    info!("PWR ch1 volt SET {}", &lp.set("ch1:volt", 4.250)?);
    info!("PWR ch1 curr SET {}", &lp.set("ch1:curr", 0.250)?);
    info!("PWR ch1 SET on");
    lp.send("output ch1,on")?;

    info!("LOAD input? {}", &ll.req(":inp?")?);
    info!("LOAD short? {}", &ll.req(":shor?")?);
    info!("LOAD irange? {}", &ll.req(":curr:irang?")?);
    info!("LOAD vrange? {}", &ll.req(":curr:vrang?")?);

    info!("***");
    thread::sleep(time::Duration::new(1, 0));

    info!("LOAD func SET {}", &ll.set_func(sdl1000x::Func::Curr)?);
    // 5A or 30A
    info!("LOAD irange SET {}", &ll.set(":curr:irang", 5.0)?);
    // 36V or 150V
    info!("LOAD vrange SET {}", &ll.set(":curr:vrang", 36.0)?);
    info!("LOAD current SET {:.3}", &ll.set(":curr", 0.120)?);
    info!("LOAD input SET {}", &ll.set_state(":inp", PortState::On)?);

    info!("***");
    thread::sleep(time::Duration::new(2, 0));

    info!("LOAD sense? {}", &ll.get_state("syst:sens")?);
    info!("LOAD volt? {:.3}", &ll.meas(sdl1000x::Meas::Volt)?);
    info!("LOAD curr? {:.3}", &ll.meas(sdl1000x::Meas::Curr)?);
    info!("LOAD power? {:.3}", &ll.meas(sdl1000x::Meas::Pow)?);
    info!("LOAD res? {:.3}", &ll.meas(sdl1000x::Meas::Res)?);

    info!(
        "PWR ch1 volt? {:.3}",
        &lp.meas(spd3303x::Ch::Ch1, spd3303x::Meas::Volt)?
    );
    info!(
        "PWR ch1 curr? {:.3}",
        &lp.meas(spd3303x::Ch::Ch1, spd3303x::Meas::Curr)?
    );
    info!(
        "PWR ch1 power? {:.3}",
        &lp.meas(spd3303x::Ch::Ch1, spd3303x::Meas::Pow)?
    );
    info!("***");
    info!("PWR ch1 volt SET {}", &lp.set("ch1:volt", 8.500)?);
    info!("LOAD current SET {:.3}", &ll.set(":curr", 0.150)?);

    info!("***");
    thread::sleep(time::Duration::new(2, 0));

    info!("LOAD volt? {:.3}", &ll.meas(sdl1000x::Meas::Volt)?);
    info!("LOAD curr? {:.3}", &ll.meas(sdl1000x::Meas::Curr)?);
    info!("LOAD power? {:.3}", &ll.meas(sdl1000x::Meas::Pow)?);
    info!("LOAD res? {:.3}", &ll.meas(sdl1000x::Meas::Res)?);
    info!(
        "PWR ch1 volt? {:.3}",
        &lp.meas(spd3303x::Ch::Ch1, spd3303x::Meas::Volt)?
    );
    info!(
        "PWR ch1 curr? {:.3}",
        &lp.meas(spd3303x::Ch::Ch1, spd3303x::Meas::Curr)?
    );
    info!(
        "PWR ch1 power? {:.3}",
        &lp.meas(spd3303x::Ch::Ch1, spd3303x::Meas::Pow)?
    );

    info!("***");
    thread::sleep(time::Duration::new(1, 0));

    info!("LOAD input SET {}", &ll.set_state(":inp", PortState::Off)?);
    info!(
        "LOAD sense SET {}",
        &ll.set_state("syst:sens", PortState::Off)?
    );
    info!("PWR ch1 SET off");
    lp.send("output ch1,off")?;

    Ok(())
}
// EOF
