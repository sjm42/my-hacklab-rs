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

    let mut lp = SPD3303X::new("PWR".into(), LAB_POWER)?;
    let mut ld = SDL1000X::new("LOAD".into(), LAB_LOAD)?;
    lp.lxi.v_on();
    ld.lxi.v_on();
    info!("Lab PWR at {:?}", lp.lxi.addr());
    info!("Lab LOAD at {:?}", ld.lxi.addr());

    lp.idn()?;
    lp.lan_addr()?;
    lp.lan_mask()?;
    lp.lan_gw()?;

    ld.idn()?;
    ld.lan_mac()?;
    ld.lan_addr()?;
    ld.lan_mask()?;
    ld.lan_gw()?;

    ld.lxi.req("func?")?;
    ld.lxi.get_state("system:sense")?;
    ld.lxi.set_state(":short:state", PortState::Off)?;
    ld.lxi.set_state(":input:state", PortState::Off)?;
    ld.lxi.set_state("system:sense", PortState::On)?;

    lp.lxi.send("output:track 0")?;
    lp.lxi.send("output ch1,off")?;
    lp.lxi.send("output ch2,off")?;
    lp.lxi.send("output ch3,off")?;
    lp.lxi.send("output:wave ch1,off")?;
    lp.lxi.send("output:wave ch2,off")?;

    lp.lxi.set("ch1:volt", 4.250)?;
    lp.lxi.set("ch1:curr", 0.250)?;
    lp.lxi.send("output ch1,on")?;

    ld.lxi.req(":input:state?")?;
    ld.lxi.req(":short:state?")?;
    ld.lxi.req(":current:irange?")?;
    ld.lxi.req(":current:vrange?")?;

    info!("***");
    thread::sleep(time::Duration::new(1, 0));

    ld.set_func(sdl1000x::Func::Curr)?;
    ld.lxi.set(":current:irange", 5.0)?; // 5A or 30A
    ld.lxi.set(":current:vrange", 36.0)?; // 36V or 150V

    ld.lxi.req(":current:irange?")?;
    ld.lxi.req(":current:vrange?")?;

    ld.lxi.set(":current", 0.120)?;
    ld.lxi.set_state(":input:state", PortState::On)?;

    info!("***");
    thread::sleep(time::Duration::new(2, 0));

    ld.lxi.get_state("system:sense")?;
    ld.meas(sdl1000x::Meas::Volt)?;
    ld.meas(sdl1000x::Meas::Curr)?;
    ld.meas(sdl1000x::Meas::Pow)?;
    ld.meas(sdl1000x::Meas::Res)?;

    lp.meas(spd3303x::Ch::Ch1, spd3303x::Meas::Volt)?;
    lp.meas(spd3303x::Ch::Ch1, spd3303x::Meas::Curr)?;
    lp.meas(spd3303x::Ch::Ch1, spd3303x::Meas::Pow)?;

    info!("***");
    lp.lxi.set("ch1:volt", 8.500)?;
    ld.lxi.set(":curr", 0.150)?;

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

    ld.lxi.set_state(":input:state", PortState::Off)?;
    ld.lxi.set_state("system:sense", PortState::Off)?;
    lp.lxi.send("output ch1,off")?;

    Ok(())
}
// EOF
