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

    let mut pwr = SPD3303X::new("PWR", LAB_POWER)?;
    let mut load = SDL1000X::new("LOAD", LAB_LOAD)?;
    pwr.lxi.v_on();
    load.lxi.v_on();
    info!("Lab PWR at {:?}", pwr.lxi.addr());
    info!("Lab LOAD at {:?}", load.lxi.addr());

    pwr.idn()?;
    pwr.lan_addr()?;
    pwr.lan_mask()?;
    pwr.lan_gw()?;
    info!("PWR status:\n{:#?}", pwr.status()?);

    load.idn()?;
    load.lan_mac()?;
    load.lan_addr()?;
    load.lan_mask()?;
    load.lan_gw()?;

    load.lxi.req("func?")?;
    load.q_sense()?;
    load.short_off()?;
    load.input_off()?;
    load.sense_on()?;

    pwr.output_independent()?;
    pwr.output_off(Ch::Ch1)?;
    pwr.output_off(Ch::Ch2)?;
    pwr.output_off(Ch::Ch3)?;
    pwr.wave_display(Ch::Ch1, PortState::Off)?;
    pwr.wave_display(Ch::Ch2, PortState::Off)?;

    pwr.volt(Ch::Ch1, 4.250)?;
    pwr.curr(Ch::Ch1, 0.250)?;
    pwr.output_on(Ch::Ch1)?;
    info!("PWR status:\n{:#?}", pwr.status()?);

    load.q_sense()?;
    load.q_input()?;
    load.q_short()?;

    load.lxi.req(":current:irange?")?;
    load.lxi.req(":current:vrange?")?;

    info!("***");
    thread::sleep(time::Duration::new(1, 0));

    load.set_func(sdl1000x::Func::Curr)?;
    load.lxi.set(":current:irange", 5.0)?; // 5A or 30A
    load.lxi.set(":current:vrange", 36.0)?; // 36V or 150V

    load.lxi.req(":current:irange?")?;
    load.lxi.req(":current:vrange?")?;

    load.lxi.set(":current", 0.120)?;
    load.input_on()?;

    info!("***");
    thread::sleep(time::Duration::new(2, 0));

    load.q_sense()?;
    load.m_volt()?;
    load.m_curr()?;
    load.m_pow()?;
    load.m_res()?;

    pwr.q_volt(Ch::Ch1)?;
    pwr.q_curr(Ch::Ch1)?;

    pwr.m_volt(Ch::Ch1)?;
    pwr.m_curr(Ch::Ch1)?;
    pwr.m_pow(Ch::Ch1)?;

    info!("***");
    pwr.volt(Ch::Ch1, 8.500)?;
    load.lxi.set(":curr", 0.150)?;

    info!("***");
    thread::sleep(time::Duration::new(2, 0));

    load.m_volt()?;
    load.m_curr()?;
    load.m_pow()?;
    load.m_res()?;

    info!("PWR status:\n{:#?}", pwr.status()?);
    pwr.q_volt(Ch::Ch1)?;
    pwr.q_curr(Ch::Ch1)?;

    pwr.m_volt(Ch::Ch1)?;
    pwr.m_curr(Ch::Ch1)?;
    pwr.m_pow(Ch::Ch1)?;

    info!("***");
    thread::sleep(time::Duration::new(1, 0));

    load.input_off()?;
    load.sense_off()?;
    pwr.output_off(Ch::Ch1)?;
    info!("PWR status:\n{:#?}", pwr.status()?);

    info!("PWR testing modes...");
    thread::sleep(time::Duration::new(1, 0));

    info!("independent");
    pwr.output_independent()?;
    info!("PWR status:\n{:#?}", pwr.status()?);
    thread::sleep(time::Duration::new(1, 0));

    info!("series");
    pwr.output_series()?;
    info!("PWR status:\n{:#?}", pwr.status()?);
    thread::sleep(time::Duration::new(1, 0));

    info!("parallel");
    pwr.output_parallel()?;
    info!("PWR status:\n{:#?}", pwr.status()?);
    thread::sleep(time::Duration::new(1, 0));

    pwr.output_independent()?;
    info!("PWR status:\n{:#?}", pwr.status()?);

    Ok(())
}
// EOF
