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
    // pwr.lxi.v_on();
    // load.lxi.v_on();
    info!("Lab PWR at {:?}", pwr.lxi.addr());
    info!("Lab LOAD at {:?}", load.lxi.addr());

    info!("PWR idn: {}", pwr.idn_q()?);
    info!("PWR version: {}", pwr.version_q()?);
    info!("PWR error: {}", pwr.error_q()?);
    info!("PWR status:\n{:#?}", pwr.status_q()?);

    info!("PWR lan_addr: {}", pwr.lan_addr_q()?);
    info!("PWR lan_mask: {}", pwr.lan_mask_q()?);
    info!("PWR lan_gw: {}", pwr.lan_gw_q()?);

    info!("LOAD idn: {}", pwr.idn_q()?);
    info!("LOAD lan_mac: {}", load.lan_mac_q()?);
    info!("LOAD lan_addr: {}", load.lan_addr_q()?);
    info!("LOAD lan_mask: {}", load.lan_mask_q()?);
    info!("LOAD lan_gw: {}", load.lan_gw_q()?);

    info!("LOAD setting up");
    load.short_off()?;
    load.input_off()?;
    load.sense_on()?;

    info!("PWR setting up");
    pwr.output_independent()?;
    pwr.output_off(Ch::Ch1)?;
    pwr.output_off(Ch::Ch2)?;
    pwr.output_off(Ch::Ch3)?;
    pwr.wave_display(Ch::Ch1, PortState::Off)?;
    pwr.wave_display(Ch::Ch2, PortState::Off)?;

    let mut p_volt: f32 = 4.250;
    let mut p_curr: f32 = 0.250;
    info!("PWR set ch1 volt={:.3} curr={:.3}, out=ON", p_volt, p_curr);
    pwr.volt(Ch::Ch1, p_volt)?;
    pwr.curr(Ch::Ch1, p_curr)?;
    pwr.output_on(Ch::Ch1)?;
    info!("PWR status:\n{:#?}", pwr.status_q()?);

    info!("LOAD sense: {}", load.sense_q()?);
    info!("LOAD input: {}", load.input_q()?);
    info!("LOAD short: {}", load.short_q()?);

    info!("LOAD setting IRange+VRange");
    load.set_func(sdl1000x::Func::Curr)?;
    load.curr_irange(sdl1000x::IRange::I5A)?;
    load.curr_vrange(sdl1000x::VRange::V36V)?;
    info!("LOAD IRange: {}", load.curr_irange_q()?);
    info!("LOAD VRange: {}", load.curr_vrange_q()?);

    load.curr_curr(0.120)?;
    load.input_on()?;

    info!("*** sleep 1");
    thread::sleep(time::Duration::new(1, 0));

    info!("LOAD sense: {}", load.sense_q()?);
    info!("LOAD meas volt: {:.3}", load.volt_m()?);
    info!("LOAD meas curr: {:.3}", load.curr_m()?);
    info!("LOAD meas powr: {:.3}", load.powr_m()?);
    info!("LOAD meas res: {:.3}", load.res_m()?);

    info!("PWR set volt: {:.3}", pwr.volt_q(Ch::Ch1)?);
    info!("PWR set curr: {:.3}", pwr.curr_q(Ch::Ch1)?);

    info!("PWR meas volt: {:.3}", pwr.volt_m(Ch::Ch1)?);
    info!("PWR meas curr: {:.3}", pwr.curr_m(Ch::Ch1)?);
    info!("PWR meas powr: {:.3}", pwr.powr_m(Ch::Ch1)?);

    p_volt = 8.500;
    p_curr = 0.500;
    info!("PWR set ch1 volt={:.3} curr={:.3}, out=ON", p_volt, p_curr);
    pwr.volt(Ch::Ch1, p_volt)?;
    pwr.curr(Ch::Ch1, p_curr)?;

    load.curr_curr(0.150)?;

    info!("*** sleep 1");
    thread::sleep(time::Duration::new(1, 0));

    info!("LOAD meas volt: {:.3}", load.volt_m()?);
    info!("LOAD meas curr: {:.3}", load.curr_m()?);
    info!("LOAD meas powr: {:.3}", load.powr_m()?);
    info!("LOAD meas res: {:.3}", load.res_m()?);

    info!("PWR status:\n{:#?}", pwr.status_q()?);
    info!("PWR set volt: {:.3}", pwr.volt_q(Ch::Ch1)?);
    info!("PWR set curr: {:.3}", pwr.curr_q(Ch::Ch1)?);

    info!("PWR meas volt: {:.3}", pwr.volt_m(Ch::Ch1)?);
    info!("PWR meas curr: {:.3}", pwr.curr_m(Ch::Ch1)?);
    info!("PWR meas powr: {:.3}", pwr.powr_m(Ch::Ch1)?);

    info!("*** sleep 1");
    thread::sleep(time::Duration::new(1, 0));

    load.input_off()?;
    load.sense_off()?;
    pwr.output_off(Ch::Ch1)?;
    info!("PWR status:\n{:#?}", pwr.status_q()?);

    Ok(())
}
// EOF
