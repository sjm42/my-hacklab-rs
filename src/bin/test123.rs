// main.rs

use log::*;
use std::{thread, time};
use structopt::StructOpt;

use my_hacklab::*;

const LAB_LOAD: &str = "lab-load.siu.ro:5025";
const LAB_POWER: &str = "lab-power.siu.ro:5025";

fn main() -> anyhow::Result<()> {
    let opts = OptsCommon::from_args();
    start_pgm(&opts, "My Hacklab");
    debug!("Global config: {opts:?}");

    let mut lp = SPD3303X::new(LAB_POWER, "PWR".into())?;
    let mut ld = SDL1000X::new(LAB_LOAD, "LOAD".into())?;
    lp.verbose = true;
    ld.verbose = true;
    info!("Lab PWR at {:?}", lp.addr());
    info!("Lab LOAD at {:?}", ld.addr());

    lp.req("*IDN?")?;
    lp.req("ip?")?;
    lp.req("mask?")?;
    lp.req("gate?")?;

    ld.req("*IDN?")?;
    ld.req("lan:mac?")?;
    ld.req("lan:ipad?")?;
    ld.req("lan:smask?")?;
    ld.req("lan:gat?")?;

    ld.req("func?")?;
    ld.get_state("system:sense")?;
    ld.set_state(":short:state", PortState::Off)?;
    ld.set_state(":input:state", PortState::Off)?;
    ld.set_state("system:sense", PortState::On)?;

    lp.send("output:track 0")?;
    lp.send("output ch1,off")?;
    lp.send("output ch2,off")?;
    lp.send("output ch3,off")?;
    lp.send("output:wave ch1,off")?;
    lp.send("output:wave ch2,off")?;

    lp.set("ch1:volt", 4.250)?;
    lp.set("ch1:curr", 0.250)?;
    lp.send("output ch1,on")?;

    ld.req(":input:state?")?;
    ld.req(":short:state?")?;
    ld.req(":current:irange?")?;
    ld.req(":current:vrange?")?;

    info!("***");
    thread::sleep(time::Duration::new(1, 0));

    ld.set_func(sdl1000x::Func::Curr)?;
    ld.set(":current:irange", 5.0)?; // 5A or 30A
    ld.set(":current:vrange", 36.0)?; // 36V or 150V

    ld.req(":current:irange?")?;
    ld.req(":current:vrange?")?;

    ld.set(":current", 0.120)?;
    ld.set_state(":input:state", PortState::On)?;

    info!("***");
    thread::sleep(time::Duration::new(2, 0));

    ld.get_state("system:sense")?;
    ld.meas(sdl1000x::Meas::Volt)?;
    ld.meas(sdl1000x::Meas::Curr)?;
    ld.meas(sdl1000x::Meas::Pow)?;
    ld.meas(sdl1000x::Meas::Res)?;

    lp.meas(spd3303x::Ch::Ch1, spd3303x::Meas::Volt)?;
    lp.meas(spd3303x::Ch::Ch1, spd3303x::Meas::Curr)?;
    lp.meas(spd3303x::Ch::Ch1, spd3303x::Meas::Pow)?;

    info!("***");
    lp.set("ch1:volt", 8.500)?;
    ld.set(":curr", 0.150)?;

    info!("***");
    thread::sleep(time::Duration::new(2, 0));

    ld.meas(sdl1000x::Meas::Volt)?;
    ld.meas(sdl1000x::Meas::Curr)?;
    ld.meas(sdl1000x::Meas::Pow)?;
    ld.meas(sdl1000x::Meas::Res)?;

    lp.meas(spd3303x::Ch::Ch1, spd3303x::Meas::Volt)?;
    lp.meas(spd3303x::Ch::Ch1, spd3303x::Meas::Curr)?;
    lp.meas(spd3303x::Ch::Ch1, spd3303x::Meas::Pow)?;

    info!("***");
    thread::sleep(time::Duration::new(1, 0));

    ld.set_state(":input:state", PortState::Off)?;
    ld.set_state("system:sense", PortState::Off)?;
    lp.send("output ch1,off")?;

    Ok(())
}
// EOF
